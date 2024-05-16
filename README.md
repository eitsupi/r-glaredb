
<!-- README.md is generated from README.Rmd. Please edit that file -->

# R bindings for GlareDB

Check out the [GlareDB repo](https://github.com/GlareDB/glaredb) to
learn more.

``` r
library(glaredb)

glaredb_sql("SELECT 'hello from R' as hello") |>
  as.data.frame()
#>          hello
#> 1 hello from R
```
