patrick::with_parameters_test_that(
  "as_glaredb_table works",
  {
    tab <- as_glaredb_table(data)
    con <- glaredb_connect()

    out <- glaredb_sql("select * from tab", con) |>
      as.data.frame()

    expect_identical(
      out,
      as_nanoarrow_array_stream(data) |> as.data.frame()
    )
  },
  patrick::cases(
    dataframe = list(data = data.frame(a = 1:3, b = letters[1:3])),
    arrow_table = skip_if_not_installed("arrow") %||% list(data = arrow::arrow_table(c = 1:3, d = letters[1:3])),
    polars_data_frame = skip_if_not_installed("polars") %||% list(data = polars::as_polars_df(data.frame(e = 1:3, f = letters[1:3])))
  )
)
