SEXP savvy_sql__ffi(SEXP query);
SEXP savvy_connect__ffi(SEXP cloud_addr);

// methods and associated functions for RGlareDbConnection
SEXP savvy_RGlareDbConnection_sql__ffi(SEXP self__, SEXP query);

// methods and associated functions for RGlareDbExecutionOutput
SEXP savvy_RGlareDbExecutionOutput_export_stream__ffi(SEXP self__, SEXP stream_ptr);

// methods and associated functions for RGlareDbTokioRuntime
