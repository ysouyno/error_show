# error_show

A tool that shows the description of the error code.

## Installing

``` shellsession
cargo install error_show
```

## Usage

``` shellsession
> error_show -h
error_show 0.1.2
Show error code information

USAGE:
    error_show [FLAGS] <errno>

FLAGS:
    -h, --help        Prints help information
    -n, --ntstatus    Is this an ntstatus code on windows?
    -V, --version     Prints version information

ARGS:
    <errno>    Decimal or hexadecimal error code
```

## Examples

On Windows(Support NTSTATUS, HRESULT and WININET error code):

``` shellsession
> error_show 0xC0000005
Error(0xC0000005): Unknown.

> error_show -n 0xC0000005
Error(0xC0000005): Invalid access to memory location.

> error_show 2
Error(2): The system cannot find the file specified.

> error_show -n 2
Error(2): ERROR_WAIT_2

> error_show 12006
Error(12006): The URL does not use a recognized protocol

> error_show 0x80004002
Error(0x80004002): No such interface supported

> error_show -2147467262
Error(-2147467262): No such interface supported
```

On Linux or macOS:

``` shellsession
% error_show 12
Error(12): Cannot allocate memory
```
