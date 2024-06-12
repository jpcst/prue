08/06/24
TO-DO:


Error handling:
- [] Change output type of send_get_request() / find_ip() to Result<> to handle 'Error 429 Too Many Requests'
```
thread 'main' panicked at src/main.rs:25:32:
called `Result::unwrap()` on an `Err` value: Custom { kind: InvalidData, error: "Failed to read JSON: EOF while parsing a value at line 1 column 0" }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
- [] Different paths for Windows and Unix
- [x] Handle multiples IPs for different Bridges at the same network (need to push this live)

Later:
- [] Optimize old code
- [] Update to Hue API v2
