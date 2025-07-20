#' @export
print.RGlareDbTable <- function(x, ...) {
  x$print()
  invisible(x)
}

#' Create a GlareDB table
#'
#' GlareDB table is a class that has a struct similar to [arrow::Table] innerly and
#' can be converted from/to other classes via [nanoarrow::as_nanoarrow_array_stream()].
#'
#' The default method of [as_glaredb_table()] calls [nanoarrow::as_nanoarrow_array_stream()]
#' internally, and all arguments are passed to it.
#' @export
#' @aliases RGlareDbTable
#' @inheritParams nanoarrow::as_nanoarrow_array_stream
#' @param x An object to be coerced to a GlareDB table.
#' @param ... Additional arguments passed to methods.
#' @return A [GlareDB table][RGlareDbTable].
#' @examples
#' con <- glaredb_connect()
#'
#' # Create a GlareDB table from a data frame with a specified schema
#' dat <- data.frame(a = 1:3, b = letters[1:3]) |>
#'   as_glaredb_table(
#'     schema = nanoarrow::na_struct(
#'       list(
#'         a = nanoarrow::na_int64(),
#'         b = nanoarrow::na_large_string()
#'       )
#'     )
#'   )
#'
#' # Run an SQL query against the connection,
#' # and convert the result to a GlareDB table
#' glaredb_sql("SELECT * FROM dat", con) |>
#'   as_glaredb_table()
#'
#' # Convert the GlareDB table to an R data frame
#' dat |>
#'   as.data.frame()
#'
#' # Convert the GlareDB table to an arrow Table
#' if (requireNamespace("arrow", quietly = TRUE)) {
#'   dat |>
#'     arrow::as_arrow_table()
#' }
#'
#' # Convert the GlareDB table to a polars DataFrame
#' if (requireNamespace("polars", quietly = TRUE)) {
#'   dat |>
#'     polars::as_polars_df()
#' }
as_glaredb_table <- function(x, ...) {
  UseMethod("as_glaredb_table")
}


#' @rdname as_glaredb_table
#' @export
as_glaredb_table.default <- function(x, ..., schema = NULL) {
  as_nanoarrow_array_stream(x, ..., schema = schema) |>
    as_glaredb_table()
}


# Utf8View type is not supported in this version of arrow-rs,
# so using the compat level 0 is needed.
#' @export
as_glaredb_table.polars_object <- function(x, ..., schema = NULL) {
  as_nanoarrow_array_stream(
    x, ..., schema = schema, polars_compat_level = "oldest"
  ) |>
    as_glaredb_table()
}


#' @export
as_glaredb_table.RGlareDbTable <- function(x, ...) {
  x
}


#' @rdname as_glaredb_table
#' @export
as_glaredb_table.nanoarrow_array_stream <- function(x, ...) {
  if (!identical(nanoarrow_schema_parse(x$get_schema())$type, "struct")) {
    stop("Can't convert non-struct array stream to GlareDB table")
  }

  RGlareDbTable$import_stream(x)
}


#' @rdname as_glaredb_table
#' @export
as_glaredb_table.RGlareDbExecutionOutput <- function(x, ...) {
  x$to_table()
}
