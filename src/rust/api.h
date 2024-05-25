SEXP savvy_sql__ffi(SEXP query, SEXP connection);
SEXP savvy_prql__ffi(SEXP query, SEXP connection);
SEXP savvy_execute__ffi(SEXP query, SEXP connection);
SEXP savvy_connect__ffi(SEXP cloud_addr, SEXP disable_tls, SEXP data_dir_or_cloud_url, SEXP spill_path, SEXP location, SEXP env);

// methods and associated functions for RGlareDbConnection
SEXP savvy_RGlareDbConnection_default_in_memory__ffi(void);
SEXP savvy_RGlareDbConnection_sql__ffi(SEXP self__, SEXP query);
SEXP savvy_RGlareDbConnection_prql__ffi(SEXP self__, SEXP query);
SEXP savvy_RGlareDbConnection_execute__ffi(SEXP self__, SEXP query);

// methods and associated functions for RGlareDbExecutionOutput
SEXP savvy_RGlareDbExecutionOutput_print__ffi(SEXP self__);
SEXP savvy_RGlareDbExecutionOutput_to_table__ffi(SEXP self__);

// methods and associated functions for RGlareDbTable
SEXP savvy_RGlareDbTable_print__ffi(SEXP self__);
SEXP savvy_RGlareDbTable_import_stream__ffi(SEXP stream_ptr);
SEXP savvy_RGlareDbTable_export_stream__ffi(SEXP self__, SEXP stream_ptr);

// methods and associated functions for RGlareDbTokioRuntime
