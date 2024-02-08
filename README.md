# vcl-formatter

This is alpha software. Use at your own risk.

Usage:

```
vcl-formatter [OPTIONS] <FILE>

Arguments:
  <FILE>  VCL file to format

Options:
  -i, --indent <INDENT>  Number of spaces to use for indentation [default: 4]
  -h, --help             Print help
  -V, --version          Print version
```

This will output formatted VCL to stdout.

## Building

```sh
make build
```

## Installing

Either download binaries from releases (only x86-64 linux available for now) or build from source.
The `Makefile` provides an `install` rule that installs the binary in `/usr/bin` by default.
You can change the install path like this:

```sh
make install PREFIX=$HOME/.local/bin
```
