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
#' @param env The connected environment, an [environment class][environment-class] or `NULL`
#' (means the [global env][.GlobalEnv]).
#' GlareDB can resister some class of R objects inside the environment automatically,
#' so you can access the objects inside this environment by the object name in the query.
#' The default, the caller environment is used.
#' @return GlareDB connection object
#' @export
#' @examples
#' # Create a connection of in-memory database
#' con <- glaredb_connect()
#'
#' # The print result shows the connected environment
#' con
#'
#' # The connected environment can be accessed by `$.env`
#' con$.env
#'
#' # Create a table to the database and insert data
#' glaredb_execute("CREATE TABLE my_table (a int)", con)
#' glaredb_execute("INSERT INTO my_table VALUES (1), (2)", con)
#'
#' # Query the data and assign the result to a variable
#' res <- glaredb_sql("SELECT * FROM my_table", con)
#'
#' # Since the result `res` exists in the connected environment,
#' # it can be resolved by the object name in the query
#' exists("res", envir = con$.env)
#'
#' glaredb_sql("SELECT * FROM res", con) |>
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
  env <- env %||% .GlobalEnv

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
  assign(".env", env, envir = con)
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
