# hexi

## install

```
clark$> cargo install --path . --force
```

## use

```
[~/code/hexi](master)
clark$> hexi --help
hexi 0.1.0

USAGE:
    hexi <number>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <number>    Either a hex number like 0x0A or a positive integer like 10
```

## examples

```
clark$> hexi 49
0x31
clark$> hexi 0x31
49
```
