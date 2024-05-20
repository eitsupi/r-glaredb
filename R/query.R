#' Run a query against a GlareDB database
#' @param query A character of the query to run.
#' - For [glaredb_sql] and [glaredb_execute], a SQL query.
#' - For [glaredb_prql], a PRQL query.
#' @param connection A [GlareDB connection object][glaredb_connect] or `NULL`.
#' If `NULL`, the default in-memory database is used.
#' @return GlareDB execusion output
#' @export
#' @examples
#' glaredb_sql("SELECT 'hello from R' as hello") |>
#'   as.data.frame()
#'
#' glaredb_prql("from [
#'   {a=5, b=false},
#'   {a=6, b=true},
#' ]") |>
#'   as.data.frame()
#'
#' glaredb_execute("create table my_table (a int)")
#' glaredb_execute("insert into my_table values (1), (2)")
#' glaredb_sql("select * from my_table") |>
#'   as.data.frame()
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
  execute(query, connection)
}
