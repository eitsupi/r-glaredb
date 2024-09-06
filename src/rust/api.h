SEXP savvy_glaredb_sql__ffi(SEXP c_arg__query, SEXP c_arg__connection);
SEXP savvy_glaredb_prql__ffi(SEXP c_arg__query, SEXP c_arg__connection);
SEXP savvy_execute__ffi(SEXP c_arg__query, SEXP c_arg__connection);
SEXP savvy_connect__ffi(SEXP c_arg__cloud_addr, SEXP c_arg__disable_tls, SEXP c_arg__env, SEXP c_arg__data_dir_or_cloud_url, SEXP c_arg__spill_path, SEXP c_arg__location, SEXP c_arg__storage_options);

// methods and associated functions for RGlareDbConnection


// methods and associated functions for RGlareDbExecutionOutput
SEXP savvy_RGlareDbExecutionOutput_print__ffi(SEXP self__);
SEXP savvy_RGlareDbExecutionOutput_to_table__ffi(SEXP self__);

// methods and associated functions for RGlareDbTable
SEXP savvy_RGlareDbTable_print__ffi(SEXP self__);
SEXP savvy_RGlareDbTable_import_stream__ffi(SEXP c_arg__stream_ptr);
SEXP savvy_RGlareDbTable_export_stream__ffi(SEXP self__, SEXP c_arg__stream_ptr);