# Chapter 2
## 2.2 Inspecting Values with dbg!()
* dbg!() is easier to use than println! for debugging while iterating
    * but remember to remove dbb!() statements when you are done with time
    * use an actual logger if you need debug logger
* create 128KB file with random data
    * dd if=/dev/urandom bs=1024 count=128 of=myfile

## 2.3
* use `cargo fmt` and `cargo clippy` in CI pipeline or client side git commit hook
    * `.git/hooks/precommit`
    ```sh
    cargo fmt
    exec cargo clippy -- -D warnings
    ```

## 2.4 Handling errors gracefully
* `yes | cargo run | head -n 1 > /dev/null`
* `yes` to generate infinite stream of "y"s
    * `head -n 1` only take the first line
    * `> /dev/null` close stdin, output to special device file (to suppress output) 

## 2.5 Clap - Command Line Argument processor
* https://docs.rs/clap/2.33.3/clap/
* define your arguments and flags, add help text, etc
* clap will match was it passed and your program can handle as needed

## 2.6 Reading/Writing Files, Buffered I/O, and Traits
* Box - pointer type for heap allocation
    * takes a trait, in our ex. Read which is present for in and out files
* `dyn Read` specifies a trait object
    * type is determined at runtime
    * are dynamically sized types
* Use Buffers to improve performance for file io
    * also works for stdin/out because they share the same Read/Write traits