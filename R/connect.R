#' Connect to a GlareDB database
#'
#' @aliases RGlareDbConnection
#' @param data_dir_or_cloud_url A character of path to a local GlareDB database or
#' a cloud URL or `NULL`. If `NULL`, a in-memory database is used.
#' @param ... Ignored.
#' @param spill_path TODO
#' @param disable_tls `TRUE` or `FALSE` to indicating whether to disable TLS.
#' @param cloud_addr A character of a GlareDB cloud URL.
#' @param location TODO
#' @param storage_options Named character vector of storage options or `NULL` (default).
#' @param env TODO
#' @return GlareDB connection object
#' @export
#' @examples
#' con <- glaredb_connect()
#' con
#'
#' glaredb_sql("SELECT 'hello from R' as hello", con) |>
#'   as_glaredb_table()
glaredb_connect <- function(
    data_dir_or_cloud_url = NULL,
    ...,
    spill_path = NULL,
    disable_tls = FALSE,
    cloud_addr = "https://console.glaredb.com",
    location = NULL,
    storage_options = NULL,
    env = parent.frame()) {
  con <- connect(
    cloud_addr = cloud_addr,
    disable_tls = disable_tls,
    data_dir_or_cloud_url = data_dir_or_cloud_url,
    spill_path = spill_path,
    location = location,
    storage_options = storage_options,
    env = env
  )

  # Store the environment for printing
  con$.env <- env
  lockBinding(".env", con)

  con
}


#' @export
print.RGlareDbConnection <- function(x, ...) {
  cat("GlareDB connection\n")
  cat("  Connected to ")

  if (is.environment(x$.env)) {
    print(x$.env)
  } else {
    cat("unknown env\n")
  }

  invisible(x)
}
