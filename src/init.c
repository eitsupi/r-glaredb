
#include <stdint.h>
#include <Rinternals.h>
#include <R_ext/Parse.h>

#include "rust/api.h"

static uintptr_t TAGGED_POINTER_MASK = (uintptr_t)1;

SEXP handle_result(SEXP res_) {
    uintptr_t res = (uintptr_t)res_;

    // An error is indicated by tag.
    if ((res & TAGGED_POINTER_MASK) == 1) {
        // Remove tag
        SEXP res_aligned = (SEXP)(res & ~TAGGED_POINTER_MASK);

        // Currently, there are two types of error cases:
        //
        //   1. Error from Rust code
        //   2. Error from R's C API, which is caught by R_UnwindProtect()
        //
        if (TYPEOF(res_aligned) == CHARSXP) {
            // In case 1, the result is an error message that can be passed to
            // Rf_errorcall() directly.
            Rf_errorcall(R_NilValue, "%s", CHAR(res_aligned));
        } else {
            // In case 2, the result is the token to restart the
            // cleanup process on R's side.
            R_ContinueUnwind(res_aligned);
        }
    }

    return (SEXP)res;
}

SEXP savvy_glaredb_sql__impl(SEXP c_arg__query, SEXP c_arg__connection) {
    SEXP res = savvy_glaredb_sql__ffi(c_arg__query, c_arg__connection);
    return handle_result(res);
}

SEXP savvy_glaredb_prql__impl(SEXP c_arg__query, SEXP c_arg__connection) {
    SEXP res = savvy_glaredb_prql__ffi(c_arg__query, c_arg__connection);
    return handle_result(res);
}

SEXP savvy_execute__impl(SEXP c_arg__query, SEXP c_arg__connection) {
    SEXP res = savvy_execute__ffi(c_arg__query, c_arg__connection);
    return handle_result(res);
}

SEXP savvy_connect__impl(SEXP c_arg__cloud_addr, SEXP c_arg__disable_tls, SEXP c_arg__env, SEXP c_arg__data_dir_or_cloud_url, SEXP c_arg__spill_path, SEXP c_arg__location, SEXP c_arg__storage_options) {
    SEXP res = savvy_connect__ffi(c_arg__cloud_addr, c_arg__disable_tls, c_arg__env, c_arg__data_dir_or_cloud_url, c_arg__spill_path, c_arg__location, c_arg__storage_options);
    return handle_result(res);
}


SEXP savvy_RGlareDbExecutionOutput_print__impl(SEXP self__) {
    SEXP res = savvy_RGlareDbExecutionOutput_print__ffi(self__);
    return handle_result(res);
}

SEXP savvy_RGlareDbExecutionOutput_to_table__impl(SEXP self__) {
    SEXP res = savvy_RGlareDbExecutionOutput_to_table__ffi(self__);
    return handle_result(res);
}

SEXP savvy_RGlareDbTable_print__impl(SEXP self__) {
    SEXP res = savvy_RGlareDbTable_print__ffi(self__);
    return handle_result(res);
}

SEXP savvy_RGlareDbTable_import_stream__impl(SEXP c_arg__stream_ptr) {
    SEXP res = savvy_RGlareDbTable_import_stream__ffi(c_arg__stream_ptr);
    return handle_result(res);
}

SEXP savvy_RGlareDbTable_export_stream__impl(SEXP self__, SEXP c_arg__stream_ptr) {
    SEXP res = savvy_RGlareDbTable_export_stream__ffi(self__, c_arg__stream_ptr);
    return handle_result(res);
}


static const R_CallMethodDef CallEntries[] = {
    {"savvy_glaredb_sql__impl", (DL_FUNC) &savvy_glaredb_sql__impl, 2},
    {"savvy_glaredb_prql__impl", (DL_FUNC) &savvy_glaredb_prql__impl, 2},
    {"savvy_execute__impl", (DL_FUNC) &savvy_execute__impl, 2},
    {"savvy_connect__impl", (DL_FUNC) &savvy_connect__impl, 7},

    {"savvy_RGlareDbExecutionOutput_print__impl", (DL_FUNC) &savvy_RGlareDbExecutionOutput_print__impl, 1},
    {"savvy_RGlareDbExecutionOutput_to_table__impl", (DL_FUNC) &savvy_RGlareDbExecutionOutput_to_table__impl, 1},
    {"savvy_RGlareDbTable_print__impl", (DL_FUNC) &savvy_RGlareDbTable_print__impl, 1},
    {"savvy_RGlareDbTable_import_stream__impl", (DL_FUNC) &savvy_RGlareDbTable_import_stream__impl, 1},
    {"savvy_RGlareDbTable_export_stream__impl", (DL_FUNC) &savvy_RGlareDbTable_export_stream__impl, 2},
    {NULL, NULL, 0}
};

void R_init_glaredb(DllInfo *dll) {
    R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
    R_useDynamicSymbols(dll, FALSE);

    // Functions for initialzation, if any.

}
