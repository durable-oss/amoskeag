#include "ruby.h"
#include "amoskeag_native.h"
#include <string.h>
#include <stdlib.h>

static VALUE mAmoskeag;
static VALUE cProgram;
static VALUE eCompileError;
static VALUE eEvalError;

// Wrapper struct for CompiledProgram
typedef struct {
    AmoskeagProgram* program;
} ProgramWrapper;

// Free function for Program objects
static void program_free(void* ptr) {
    ProgramWrapper* wrapper = (ProgramWrapper*)ptr;
    if (wrapper->program) {
        amoskeag_program_free(wrapper->program);
    }
    free(wrapper);
}

// Size function for Program objects
static size_t program_memsize(const void* ptr) {
    return sizeof(ProgramWrapper);
}

static const rb_data_type_t program_type = {
    "Amoskeag::Program",
    {0, program_free, program_memsize,},
    0, 0,
    RUBY_TYPED_FREE_IMMEDIATELY,
};

// Helper to convert Ruby array to JSON string
static VALUE ruby_to_json_string(VALUE obj) {
    VALUE json_module = rb_const_get(rb_cObject, rb_intern("JSON"));
    return rb_funcall(json_module, rb_intern("generate"), 1, obj);
}

// Helper to convert JSON string to Ruby object
static VALUE json_string_to_ruby(const char* json_str) {
    VALUE json_module = rb_const_get(rb_cObject, rb_intern("JSON"));
    VALUE ruby_str = rb_str_new_cstr(json_str);
    VALUE result = rb_funcall(json_module, rb_intern("parse"), 1, ruby_str);

    // Convert symbol markers back to Ruby symbols
    if (RB_TYPE_P(result, T_HASH)) {
        VALUE symbol_key = rb_str_new_cstr("__symbol__");
        VALUE symbol_val = rb_hash_aref(result, symbol_key);
        if (!NIL_P(symbol_val)) {
            return ID2SYM(rb_intern(StringValueCStr(symbol_val)));
        }
    }

    return result;
}

// Recursively convert Ruby value to use symbol markers
static VALUE prepare_value_for_json(VALUE obj) {
    if (SYMBOL_P(obj)) {
        VALUE hash = rb_hash_new();
        rb_hash_aset(hash, rb_str_new_cstr("__symbol__"), rb_sym2str(obj));
        return hash;
    } else if (RB_TYPE_P(obj, T_ARRAY)) {
        VALUE new_array = rb_ary_new();
        long len = RARRAY_LEN(obj);
        for (long i = 0; i < len; i++) {
            rb_ary_push(new_array, prepare_value_for_json(rb_ary_entry(obj, i)));
        }
        return new_array;
    } else if (RB_TYPE_P(obj, T_HASH)) {
        VALUE new_hash = rb_hash_new();
        VALUE keys = rb_funcall(obj, rb_intern("keys"), 0);
        long len = RARRAY_LEN(keys);
        for (long i = 0; i < len; i++) {
            VALUE key = rb_ary_entry(keys, i);
            VALUE val = rb_hash_aref(obj, key);
            rb_hash_aset(new_hash, key, prepare_value_for_json(val));
        }
        return new_hash;
    }
    return obj;
}

/*
 * Compile an Amoskeag program
 *
 * @param source [String] The Amoskeag source code
 * @param symbols [Array<String>, nil] Optional array of valid symbol names
 * @return [Amoskeag::Program] Compiled program
 * @raise [Amoskeag::CompileError] If compilation fails
 */
static VALUE amoskeag_compile_wrapper(int argc, VALUE* argv, VALUE self) {
    VALUE source, symbols;
    rb_scan_args(argc, argv, "11", &source, &symbols);

    Check_Type(source, T_STRING);
    const char* source_cstr = StringValueCStr(source);

    char* symbols_json = NULL;
    if (!NIL_P(symbols)) {
        Check_Type(symbols, T_ARRAY);
        VALUE json_str = ruby_to_json_string(symbols);
        symbols_json = StringValueCStr(json_str);
    }

    char* error_msg = NULL;
    AmoskeagProgram* program = amoskeag_compile(source_cstr, symbols_json, &error_msg);

    if (program == NULL) {
        VALUE error_str = rb_str_new_cstr(error_msg ? error_msg : "Compilation failed");
        if (error_msg) {
            amoskeag_string_free(error_msg);
        }
        rb_raise(eCompileError, "%s", StringValueCStr(error_str));
    }

    ProgramWrapper* wrapper = malloc(sizeof(ProgramWrapper));
    wrapper->program = program;

    return TypedData_Wrap_Struct(cProgram, &program_type, wrapper);
}

/*
 * Evaluate a compiled program with data
 *
 * @param program [Amoskeag::Program] The compiled program
 * @param data [Hash] The data context for evaluation
 * @return [Object] The result of evaluation
 * @raise [Amoskeag::EvalError] If evaluation fails
 */
static VALUE amoskeag_evaluate_wrapper(VALUE self, VALUE program_obj, VALUE data) {
    ProgramWrapper* wrapper;
    TypedData_Get_Struct(program_obj, ProgramWrapper, &program_type, wrapper);

    Check_Type(data, T_HASH);

    // Convert data to JSON
    VALUE prepared_data = prepare_value_for_json(data);
    VALUE data_json_str = ruby_to_json_string(prepared_data);
    const char* data_json = StringValueCStr(data_json_str);

    char* error_msg = NULL;
    char* result_json = amoskeag_evaluate(wrapper->program, data_json, &error_msg);

    if (result_json == NULL) {
        VALUE error_str = rb_str_new_cstr(error_msg ? error_msg : "Evaluation failed");
        if (error_msg) {
            amoskeag_string_free(error_msg);
        }
        rb_raise(eEvalError, "%s", StringValueCStr(error_str));
    }

    VALUE result = json_string_to_ruby(result_json);
    amoskeag_string_free(result_json);

    return result;
}

/*
 * Compile and evaluate in one step (convenience method)
 *
 * @param source [String] The Amoskeag source code
 * @param data [Hash] The data context for evaluation
 * @param symbols [Array<String>, nil] Optional array of valid symbol names
 * @return [Object] The result of evaluation
 */
static VALUE amoskeag_eval_wrapper(int argc, VALUE* argv, VALUE self) {
    VALUE source, data, symbols;
    rb_scan_args(argc, argv, "21", &source, &data, &symbols);

    VALUE compile_args[2] = {source, symbols};
    VALUE program = amoskeag_compile_wrapper(NIL_P(symbols) ? 1 : 2, compile_args, self);

    return amoskeag_evaluate_wrapper(self, program, data);
}

void Init_amoskeag_native(void) {
    // Define module and classes
    mAmoskeag = rb_define_module("Amoskeag");
    cProgram = rb_define_class_under(mAmoskeag, "Program", rb_cObject);
    eCompileError = rb_define_class_under(mAmoskeag, "CompileError", rb_eStandardError);
    eEvalError = rb_define_class_under(mAmoskeag, "EvalError", rb_eStandardError);

    // Define module methods
    rb_define_module_function(mAmoskeag, "compile", amoskeag_compile_wrapper, -1);
    rb_define_module_function(mAmoskeag, "evaluate", amoskeag_evaluate_wrapper, 2);
    rb_define_module_function(mAmoskeag, "eval", amoskeag_eval_wrapper, -1);

    // Prevent instantiation of Program from Ruby
    rb_undef_alloc_func(cProgram);
}
