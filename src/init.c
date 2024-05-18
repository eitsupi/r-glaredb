
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

SEXP savvy_sql__impl(SEXP query) {
    SEXP res = savvy_sql__ffi(query);
    return handle_result(res);
}

SEXP savvy_connect__impl(SEXP cloud_addr, SEXP disable_tls, SEXP data_dir_or_cloud_url, SEXP spill_path, SEXP location) {
    SEXP res = savvy_connect__ffi(cloud_addr, disable_tls, data_dir_or_cloud_url, spill_path, location);
    return handle_result(res);
}

SEXP savvy_RGlareDbConnection_sql__impl(SEXP self__, SEXP query) {
    SEXP res = savvy_RGlareDbConnection_sql__ffi(self__, query);
    return handle_result(res);
}

SEXP savvy_RGlareDbExecutionOutput_export_stream__impl(SEXP self__, SEXP stream_ptr) {
    SEXP res = savvy_RGlareDbExecutionOutput_export_stream__ffi(self__, stream_ptr);
    return handle_result(res);
}



static const R_CallMethodDef CallEntries[] = {
    {"savvy_sql__impl", (DL_FUNC) &savvy_sql__impl, 1},
    {"savvy_connect__impl", (DL_FUNC) &savvy_connect__impl, 5},
    {"savvy_RGlareDbConnection_sql__impl", (DL_FUNC) &savvy_RGlareDbConnection_sql__impl, 2},
    {"savvy_RGlareDbExecutionOutput_export_stream__impl", (DL_FUNC) &savvy_RGlareDbExecutionOutput_export_stream__impl, 2},

    {NULL, NULL, 0}
};

void R_init_glaredb(DllInfo *dll) {
    R_registerRoutines(dll, NULL, CallEntries, NULL, NULL);
    R_useDynamicSymbols(dll, FALSE);

    // Functions for initialzation, if any.

}
