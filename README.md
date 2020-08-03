# error_show

A tool that shows the description of the error code.

## Installing

``` shellsession
cargo install error_show
```

## Usage

``` shellsession
% error_show -h
error_show 0.1.0
Show error code information

USAGE:
    error_show <errno>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <errno>    Decimal or hexadecimal error code
```

## Examples

On Linux:

``` shellsession
% error_show 12
Error(12): Cannot allocate memory
```

On Windows:

``` shellsession
> error_show 12006
Error(12006): URL 未使用可识别的协议

> error_show 2
Error(2): 系统找不到指定的文件。

> error_show 0x80020005
Error(2147614725): 类型不匹配。
```
