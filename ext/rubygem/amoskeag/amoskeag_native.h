#ifndef AMOSKEAG_NATIVE_H
#define AMOSKEAG_NATIVE_H

#ifdef __cplusplus
extern "C" {
#endif

// Opaque types
typedef struct AmoskeagProgram AmoskeagProgram;

// FFI functions from Rust
AmoskeagProgram* amoskeag_compile(const char* source, const char* symbols_json, char** error_out);
char* amoskeag_evaluate(const AmoskeagProgram* program, const char* data_json, char** error_out);
void amoskeag_program_free(AmoskeagProgram* program);
void amoskeag_string_free(char* s);

#ifdef __cplusplus
}
#endif

#endif /* AMOSKEAG_NATIVE_H */
