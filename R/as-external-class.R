#' @export
as_nanoarrow_array_stream.RGlareDbTable <- function(x, ..., schema = NULL) {
  if (!is.null(schema)) {
    # If a schema is passed, first resolve the stream as is and then use
    # as_nanoarrow_array_stream() to either cast (when this is supported)
    # or error.
    stream <- as_nanoarrow_array_stream(x, schema = NULL)
    return(as_nanoarrow_array_stream(stream, schema = schema))
  }

  stream <- nanoarrow_allocate_array_stream()
  x$export_stream(nanoarrow_pointer_addr_chr(stream))

  stream
}


#' @export
as.data.frame.RGlareDbTable <- function(x, ...) {
  as_nanoarrow_array_stream(x) |>
    as.data.frame()
}


#' @export
as_nanoarrow_array_stream.RGlareDbExecutionOutput <- function(x, ..., schema = NULL) {
  x |>
    as_glaredb_table() |>
    as_nanoarrow_array_stream(..., schema = schema)
}


#' @export
as.data.frame.RGlareDbExecutionOutput <- function(x, ...) {
  x |>
    as_glaredb_table() |>
    as.data.frame()
}


# For the arrow package
# exported in zzz.R
as_arrow_table.RGlareDbTable <- function(x, ...) {
  as_nanoarrow_array_stream(x) |>
    arrow::as_arrow_table()
}


# exported in zzz.R
as_arrow_table.RGlareDbExecutionOutput <- function(x, ...) {
  x |>
    as_glaredb_table() |>
    as_arrow_table.RGlareDbTable()
}


# For the polars package
# exported in zzz.R
as_polars_df.RGlareDbTable <- function(x, ...) {
  as_nanoarrow_array_stream(x) |>
    polars::as_polars_df()
}


# exported in zzz.R
as_polars_df.RGlareDbExecutionOutput <- function(x, ...) {
  x |>
    as_glaredb_table() |>
    as_polars_df.RGlareDbTable()
}
