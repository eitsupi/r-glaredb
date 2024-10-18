# glaredb 0.0.4

## Bug fixes

- Fix to report rustc version even if installing with pre-built binaries. (#71, #72)
- Set `OS_type: unix` in the DESCRIPTION file, because the package is not installable on Windows for now. (#78)

# glaredb 0.0.3

- Based on glaredb 0.9.5. (#56, thanks @tychoish)

# glaredb 0.0.2

- Support the `storage_options` argument of `glaredb_connect()` function. (#28)
- Based on glaredb 0.9.4. (#39)

# glaredb 0.0.1

Experimental initial release.

## Known issues:

- Can't install on Windows.
- Can't do cross-compilation for arm64 Linux.
  So pre-built libraries are available only for amd64 Linux, arm64 macOS, amd64 macOS.
- Documentation is work in progress.
