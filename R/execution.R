#' @export
as_nanoarrow_array_stream.RGlareDbExecutionOutput <- function(x, ..., schema = NULL) {
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
