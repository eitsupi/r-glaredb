#' @export
print.RGlareDbExecutionOutput <- function(x, ...) {
  x$print()
  invisible(x)
}


#' Run a query against a GlareDB database
#' @param query A character of the query to run.
#' - For [glaredb_sql()] and [glaredb_execute()], an SQL query.
#' - For [glaredb_prql()], a PRQL query.
#' @param connection A [GlareDB connection object][glaredb_connect] or `NULL`.
#' If `NULL`, the default in-memory database is used.
#' @return A GlareDB execusion output (Query plan).
#' For [glaredb_execute()], the value is returned invisibly.
#' @export
#' @examples
#' # You can materialize the query result by `as_glaredb_table()` etc.
#' glaredb_sql("SELECT 'hello from R' as hello") |>
#'   as_glaredb_table()
#'
#' glaredb_prql("from [
#'   {a=5, b=false},
#'   {a=6, b=true},
#' ]") |>
#'   as.data.frame()
#'
#' # `glaredb_execute()` is useful for manipulating the database
#' glaredb_execute("CREATE TABLE my_table (a int)")
#' glaredb_execute("INSERT INTO my_table VALUES (1), (2)")
#'
#' glaredb_sql("SELECT * FROM my_table") |>
#'   as_glaredb_table()
#' @export
#' @order 1
glaredb_execute <- function(query, connection = NULL) {
  execute(query, connection) |>
    invisible()
}
