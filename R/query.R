#' Run a SQL query against the default in-memory GlareDB database
#' @param query A character of the SQL query to run
#' @param connection A [GlareDB connection object][glaredb_connect] or `NULL`.
#' If `NULL`, the default in-memory database is used.
#' @return GlareDB execusion output
#' @export
#' @examples
#' glaredb_sql("SELECT 'hello from R' as hello") |>
#'   as.data.frame()
glaredb_sql <- function(query, connection = NULL) {
  sql(query, connection)
}
