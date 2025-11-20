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

// Helper functions for rb_protect callbacks
static VALUE const_get_json(VALUE unused) {
    (void)unused;
    return rb_const_get(rb_cObject, rb_intern("JSON"));
}

typedef struct {
    VALUE receiver;
    ID method;
    int argc;
    VALUE *argv;
} funcall_args;

static VALUE protected_funcall(VALUE arg) {
    funcall_args *args = (funcall_args *)arg;
    return rb_funcall2(args->receiver, args->method, args->argc, args->argv);
}

// Helper to convert Ruby array to JSON string
// Defensive: Validates input and handles errors
static VALUE ruby_to_json_string(VALUE obj) {
    int state = 0;
    VALUE json_module, result;

    // Defensive: Protect against exceptions during constant lookup
    json_module = rb_protect(const_get_json, Qnil, &state);
    if (state) {
        rb_raise(rb_eLoadError, "JSON module not available. Please require 'json'.");
    }

    // Defensive: Check if obj is valid
    if (obj == Qundef) {
        rb_raise(rb_eArgError, "Cannot convert undefined value to JSON");
    }

    // Defensive: Protect against exceptions during JSON generation
    VALUE argv[1] = { obj };
    funcall_args args = { json_module, rb_intern("generate"), 1, argv };
    result = rb_protect(protected_funcall, (VALUE)&args, &state);
    if (state) {
        rb_jump_tag(state); // Re-raise the exception
    }

    return result;
}

// Helper to convert JSON string to Ruby object
// Defensive: Validates input and handles errors
static VALUE json_string_to_ruby(const char* json_str) {
    if (json_str == NULL) {
        rb_raise(rb_eArgError, "JSON string is NULL");
    }

    // Defensive: Check JSON string length
    size_t json_len = strlen(json_str);
    if (json_len == 0) {
        rb_raise(rb_eArgError, "JSON string is empty");
    }
    if (json_len > 100 * 1024 * 1024) { // 100MB limit
        rb_raise(rb_eArgError, "JSON string too large: %zu bytes (max: 100MB)", json_len);
    }

    int state = 0;
    VALUE json_module = rb_const_get(rb_cObject, rb_intern("JSON"));
    VALUE ruby_str = rb_str_new_cstr(json_str);

    // Defensive: Protect against exceptions during JSON parsing
    VALUE argv[1] = { ruby_str };
    funcall_args args = { json_module, rb_intern("parse"), 1, argv };
    VALUE result = rb_protect(protected_funcall, (VALUE)&args, &state);
    if (state) {
        rb_jump_tag(state); // Re-raise the exception
    }

    // Convert symbol markers back to Ruby symbols
    if (RB_TYPE_P(result, T_HASH)) {
        VALUE symbol_key = rb_str_new_cstr("__symbol__");
        VALUE symbol_val = rb_hash_aref(result, symbol_key);
        if (!NIL_P(symbol_val)) {
            // Defensive: Validate symbol value is a string
            if (!RB_TYPE_P(symbol_val, T_STRING)) {
                rb_raise(rb_eTypeError, "__symbol__ value must be a string");
            }
            return ID2SYM(rb_intern(StringValueCStr(symbol_val)));
        }
    }

    return result;
}

// Forward declaration for recursive function
static VALUE prepare_value_for_json_with_depth(VALUE obj, long depth);

// Recursively convert Ruby value to use symbol markers
// Defensive: Validates input and prevents stack overflow
static VALUE prepare_value_for_json(VALUE obj) {
    return prepare_value_for_json_with_depth(obj, 0);
}

static VALUE prepare_value_for_json_with_depth(VALUE obj, long depth) {
    // Defensive: Prevent stack overflow from deeply nested structures
    const long MAX_DEPTH = 100;
    if (depth > MAX_DEPTH) {
        rb_raise(rb_eArgError, "Data structure too deeply nested (max depth: %ld)", MAX_DEPTH);
    }

    if (SYMBOL_P(obj)) {
        VALUE hash = rb_hash_new();
        rb_hash_aset(hash, rb_str_new_cstr("__symbol__"), rb_sym2str(obj));
        return hash;
    } else if (RB_TYPE_P(obj, T_ARRAY)) {
        long len = RARRAY_LEN(obj);

        // Defensive: Check array size
        if (len > 1000000) {
            rb_raise(rb_eArgError, "Array too large: %ld elements (max: 1,000,000)", len);
        }

        VALUE new_array = rb_ary_new();
        for (long i = 0; i < len; i++) {
            VALUE elem = rb_ary_entry(obj, i);
            rb_ary_push(new_array, prepare_value_for_json_with_depth(elem, depth + 1));
        }
        return new_array;
    } else if (RB_TYPE_P(obj, T_HASH)) {
        VALUE keys = rb_funcall(obj, rb_intern("keys"), 0);
        long len = RARRAY_LEN(keys);

        // Defensive: Check hash size
        if (len > 100000) {
            rb_raise(rb_eArgError, "Hash too large: %ld keys (max: 100,000)", len);
        }

        VALUE new_hash = rb_hash_new();
        for (long i = 0; i < len; i++) {
            VALUE key = rb_ary_entry(keys, i);
            VALUE val = rb_hash_aref(obj, key);

            // Defensive: Validate key is a string or symbol
            if (!RB_TYPE_P(key, T_STRING) && !SYMBOL_P(key)) {
                rb_raise(rb_eTypeError, "Hash key must be String or Symbol, got %s",
                         rb_obj_classname(key));
            }

            rb_hash_aset(new_hash, key, prepare_value_for_json_with_depth(val, depth + 1));
        }
        return new_hash;
    } else if (RB_TYPE_P(obj, T_STRING) || RB_TYPE_P(obj, T_FIXNUM) ||
               RB_TYPE_P(obj, T_BIGNUM) || RB_TYPE_P(obj, T_FLOAT) ||
               RB_TYPE_P(obj, T_TRUE) || RB_TYPE_P(obj, T_FALSE) ||
               NIL_P(obj)) {
        // These types are JSON-serializable as-is
        return obj;
    } else {
        // Defensive: Reject unsupported types
        rb_raise(rb_eTypeError, "Unsupported type for JSON conversion: %s",
                 rb_obj_classname(obj));
    }
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

    // Defensive: Validate source argument
    if (NIL_P(source)) {
        rb_raise(rb_eArgError, "source cannot be nil");
    }
    Check_Type(source, T_STRING);

    // Defensive: Check source length
    long source_len = RSTRING_LEN(source);
    if (source_len == 0) {
        rb_raise(rb_eArgError, "source cannot be empty");
    }
    if (source_len > 10 * 1024 * 1024) { // 10MB limit
        rb_raise(rb_eArgError, "source too large: %ld bytes (max: 10MB)", source_len);
    }

    const char* source_cstr = StringValueCStr(source);

    // Defensive: Additional null check
    if (source_cstr == NULL) {
        rb_raise(rb_eArgError, "source string conversion failed");
    }

    char* symbols_json = NULL;
    VALUE symbols_json_str = Qnil; // Keep alive to prevent GC

    if (!NIL_P(symbols)) {
        Check_Type(symbols, T_ARRAY);

        // Defensive: Check symbols array size
        long symbols_len = RARRAY_LEN(symbols);
        if (symbols_len > 10000) {
            rb_raise(rb_eArgError, "Too many symbols: %ld (max: 10,000)", symbols_len);
        }

        // Defensive: Validate all symbols are strings or symbols
        for (long i = 0; i < symbols_len; i++) {
            VALUE sym = rb_ary_entry(symbols, i);
            if (!RB_TYPE_P(sym, T_STRING) && !SYMBOL_P(sym)) {
                rb_raise(rb_eTypeError, "symbols[%ld] must be String or Symbol, got %s",
                         i, rb_obj_classname(sym));
            }
        }

        symbols_json_str = ruby_to_json_string(symbols);
        symbols_json = StringValueCStr(symbols_json_str);
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

    // Defensive: Check malloc result
    ProgramWrapper* wrapper = malloc(sizeof(ProgramWrapper));
    if (wrapper == NULL) {
        amoskeag_program_free(program);
        rb_raise(rb_eNoMemError, "Failed to allocate memory for program wrapper");
    }

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
    // Defensive: Validate program_obj
    if (NIL_P(program_obj)) {
        rb_raise(rb_eArgError, "program cannot be nil");
    }

    ProgramWrapper* wrapper;
    TypedData_Get_Struct(program_obj, ProgramWrapper, &program_type, wrapper);

    // Defensive: Validate wrapper and program
    if (wrapper == NULL) {
        rb_raise(rb_eArgError, "Invalid program object (wrapper is NULL)");
    }
    if (wrapper->program == NULL) {
        rb_raise(rb_eArgError, "Invalid program object (program is NULL)");
    }

    // Defensive: Validate data
    if (NIL_P(data)) {
        rb_raise(rb_eArgError, "data cannot be nil");
    }
    Check_Type(data, T_HASH);

    // Defensive: Check data size
    long data_size = RHASH_SIZE(data);
    if (data_size > 100000) {
        rb_raise(rb_eArgError, "data hash too large: %ld keys (max: 100,000)", data_size);
    }

    // Convert data to JSON
    VALUE prepared_data = prepare_value_for_json(data);
    VALUE data_json_str = ruby_to_json_string(prepared_data);

    // Defensive: Validate JSON conversion result
    if (NIL_P(data_json_str) || !RB_TYPE_P(data_json_str, T_STRING)) {
        rb_raise(rb_eRuntimeError, "Failed to convert data to JSON string");
    }

    const char* data_json = StringValueCStr(data_json_str);

    // Defensive: Additional null check
    if (data_json == NULL) {
        rb_raise(rb_eRuntimeError, "JSON string conversion failed");
    }

    char* error_msg = NULL;
    char* result_json = amoskeag_evaluate(wrapper->program, data_json, &error_msg);

    if (result_json == NULL) {
        VALUE error_str = rb_str_new_cstr(error_msg ? error_msg : "Evaluation failed");
        if (error_msg) {
            amoskeag_string_free(error_msg);
        }
        rb_raise(eEvalError, "%s", StringValueCStr(error_str));
    }

    // Defensive: Validate result_json before conversion
    if (result_json[0] == '\0') {
        amoskeag_string_free(result_json);
        rb_raise(eEvalError, "Evaluation returned empty result");
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
static VALUE amoskeag_eval_expression_wrapper(int argc, VALUE* argv, VALUE self) {
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
    rb_define_module_function(mAmoskeag, "eval_expression", amoskeag_eval_expression_wrapper, -1);

    // Prevent instantiation of Program from Ruby
    rb_undef_alloc_func(cProgram);
}
