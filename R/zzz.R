.onLoad <- function(...) {
  s3_register("arrow::as_arrow_table", "RGlareDbTable")
  s3_register("arrow::as_arrow_table", "RGlareDbExecutionOutput")
  s3_register("polars::as_polars_df", "RGlareDbTable")
  s3_register("polars::as_polars_df", "RGlareDbExecutionOutput")
}
