#' Connect to a GlareDB database
#'
#' @param data_dir_or_cloud_url A character of path to a local GlareDB database or
#' a cloud URL or `NULL`. If `NULL`, a in-memory database is used.
#' @param ... Ignored.
#' @param spill_path TODO
#' @param disable_tls `TRUE` or `FALSE` to indicating whether to disable TLS.
#' @param cloud_addr A character of a GlareDB cloud URL.
#' @param location TODO
#' @param env TODO
#' @return GlareDB connection object
#' @export
#' @examples
#' con <- glaredb_connect()
#'
#' con$sql("SELECT 'hello from R' as hello") |>
#'   as.data.frame()
glaredb_connect <- function(
    data_dir_or_cloud_url = NULL,
    ...,
    spill_path = NULL,
    disable_tls = FALSE,
    cloud_addr = "https://console.glaredb.com",
    location = NULL,
    env = parent.frame()) {
  connect(
    cloud_addr = cloud_addr,
    disable_tls = disable_tls,
    data_dir_or_cloud_url = data_dir_or_cloud_url,
    spill_path = spill_path,
    location = location,
    env = env
  )
}
