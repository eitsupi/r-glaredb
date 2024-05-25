
<!-- README.md is generated from README.Rmd. Please edit that file -->

# R bindings for GlareDB

Check out the [GlareDB repo](https://github.com/GlareDB/glaredb) to
learn more.

Use GlareDB directly in R to query and analyzer a variety of data
sources, including `arrow::Table` and `polars::RPolarsDataFrame`.

``` r
library(glaredb)
library(polars)

df = pl$DataFrame(
  A = 1:5,
  fruits = c("banana", "banana", "apple", "apple", "banana"),
  B = 5:1,
  C = c("beetle", "audi", "beetle", "beetle", "beetle")
)

df2 = glaredb_sql("select * from df where fruits = 'banana'") |>
  as_polars_df()

df2
#> shape: (3, 4)
#> ┌─────┬────────┬─────┬────────┐
#> │ A   ┆ fruits ┆ B   ┆ C      │
#> │ --- ┆ ---    ┆ --- ┆ ---    │
#> │ i32 ┆ str    ┆ i32 ┆ str    │
#> ╞═════╪════════╪═════╪════════╡
#> │ 1   ┆ banana ┆ 5   ┆ beetle │
#> │ 2   ┆ banana ┆ 4   ┆ audi   │
#> │ 5   ┆ banana ┆ 1   ┆ beetle │
#> └─────┴────────┴─────┴────────┘
```
