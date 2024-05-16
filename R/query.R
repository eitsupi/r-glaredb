#' Run a SQL query against the default in-memory GlareDB database
#' @param query A character of the SQL query to run
#' @return [nanoarrow_array_stream]
#' @export
#' @examples
#' glaredb_sql("SELECT 'hello from R' as hello") |>
#'   as.data.frame()
glaredb_sql <- function(query) {
  sql(query) |>
    as_nanoarrow_array_stream()
}
