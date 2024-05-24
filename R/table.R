#' @export
print.RGlareDbTable <- function(x, ...) {
  x$print()
  invisible(x)
}

#' Create a GlareDB table
#'
#' TODO
#' @export
#' @aliases RGlareDbTable
#' @param x An object to be coerced to a GlareDB table.
#' @param ... Additional arguments passed to methods.
#' @return A GlareDB table.
#' @examples
#' con <- glaredb_connect()
#' dat <- as_glaredb_table(data.frame(a = 1:3, b = letters[1:3]))
#'
#' glaredb_sql("SELECT * FROM dat", con) |>
#'   as.data.frame()
as_glaredb_table <- function(x, ...) {
  UseMethod("as_glaredb_table")
}


#' @rdname as_glaredb_table
#' @export
as_glaredb_table.default <- function(x, ...) {
  as_nanoarrow_array_stream(x, ...) |>
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
