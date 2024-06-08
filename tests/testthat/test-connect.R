test_that("print connection", {
  expect_snapshot(glaredb_connect(env = NULL))
  expect_snapshot(glaredb_connect(env = .GlobalEnv))
})
