#' Run a SQL query against the default in-memory GlareDB database
#' @param query A character of the SQL query to run
#' @param env TODO
#' @return GlareDB execusion output
#' @export
#' @examples
#' glaredb_sql("SELECT 'hello from R' as hello") |>
#'   as.data.frame()
glaredb_sql <- function(query) {
  sql(query)
}
