#' @export
print.RGlareDbExecutionOutput <- function(x, ...) {
  x$print()
  invisible(x)
}


#' Run a query against a GlareDB database
#' @param query A character of the query to run.
#' - For [glaredb_sql] and [glaredb_execute], an SQL query.
#' - For [glaredb_prql], a PRQL query.
#' @param connection A [GlareDB connection object][glaredb_connect] or `NULL`.
#' If `NULL`, the default in-memory database is used.
#' @return GlareDB execusion output.
#' For [glaredb_execute], the value is returned invisibly.
#' @export
#' @examples
#' glaredb_sql("SELECT 'hello from R' as hello") |>
#'   as_glaredb_table()
#'
#' glaredb_prql("from [
#'   {a=5, b=false},
#'   {a=6, b=true},
#' ]") |>
#'   as_glaredb_table()
#'
#' glaredb_execute("CREATE TABLE my_table (a int)")
#' glaredb_execute("INSERT INTO my_table VALUES (1), (2)")
#' glaredb_sql("SELECT * FROM my_table") |>
#'   as_glaredb_table()
glaredb_sql <- function(query, connection = NULL) {
  sql(query, connection)
}

#' @rdname glaredb_sql
#' @export
glaredb_prql <- function(query, connection = NULL) {
  prql(query, connection)
}

#' @rdname glaredb_sql
#' @export
glaredb_execute <- function(query, connection = NULL) {
  execute(query, connection) |>
    invisible()
}
