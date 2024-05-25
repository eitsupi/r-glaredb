`%||%` <- function(x, y) if (is.null(x)) y else x


# Copied from the vctrs package.
# <https://github.com/r-lib/vctrs/blob/920903d95fb405a5ac6c56c35a2c544c5aa3fa3b/R/register-s3.R#L25-L31>
# This s3_register function is licensed under [unlicense](https://choosealicense.com/licenses/unlicense/).
s3_register <- function(generic, class, method = NULL) {
  stopifnot(is.character(generic), length(generic) == 1)
  stopifnot(is.character(class), length(class) == 1)

  pieces <- strsplit(generic, "::")[[1]]
  stopifnot(length(pieces) == 2)
  package <- pieces[[1]]
  generic <- pieces[[2]]

  caller <- parent.frame()

  get_method_env <- function() {
    top <- topenv(caller)
    if (isNamespace(top)) {
      asNamespace(environmentName(top))
    } else {
      caller
    }
  }
  get_method <- function(method) {
    if (is.null(method)) {
      get(paste0(generic, ".", class), envir = get_method_env())
    } else {
      method
    }
  }

  register <- function(...) {
    envir <- asNamespace(package)

    # Refresh the method each time, it might have been updated by
    # `devtools::load_all()`
    method_fn <- get_method(method)
    stopifnot(is.function(method_fn))


    # Only register if generic can be accessed
    if (exists(generic, envir)) {
      registerS3method(generic, class, method_fn, envir = envir)
    } else if (identical(Sys.getenv("NOT_CRAN"), "true")) {
      warn <- .rlang_s3_register_compat("warn")

      warn(c(
        sprintf(
          "Can't find generic `%s` in package %s to register S3 method.",
          generic,
          package
        ),
        "i" = "This message is only shown to developers using devtools.",
        "i" = sprintf("Do you need to update %s to the latest version?", package)
      ))
    }
  }

  # Always register hook in case package is later unloaded & reloaded
  setHook(packageEvent(package, "onLoad"), function(...) {
    register()
  })

  # For compatibility with R < 4.0 where base isn't locked
  is_sealed <- function(pkg) {
    identical(pkg, "base") || environmentIsLocked(asNamespace(pkg))
  }

  # Avoid registration failures during loading (pkgload or regular).
  # Check that environment is locked because the registering package
  # might be a dependency of the package that exports the generic. In
  # that case, the exports (and the generic) might not be populated
  # yet (#1225).
  if (isNamespaceLoaded(package) && is_sealed(package)) {
    register()
  }

  invisible()
}
