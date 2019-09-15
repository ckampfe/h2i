# h2i

## install

```
clark$> cargo install --path . --force
```

## use

```
[~/code/h2i](master)
clark$> h2i --help
h2i 0.1.0

USAGE:
    h2i <number>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <number>    Either a hex number like 0x0A or a positive integer like 10
```

## examples

```
clark$> h2i 49
0x31
clark$> h2i 0x31
49
```
