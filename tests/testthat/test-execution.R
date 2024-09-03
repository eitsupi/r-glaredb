test_that("GlareDB version", {
  expect_snapshot(glaredb_sql("select version()") |> as.data.frame())
})
