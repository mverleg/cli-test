
# cli-test

⚠️ **Under construction**

A mini language and test runner for testing command line utilities.

### Example

File 

In `count_lines.clts`

```
# this is the command to test
TEST:
    printf 'a\nb\nc' | wc -l

# this checks that the exit code is 0
EXIT_CODE: 0

# this check that the stdout is 3
OUT: it == 3

```

Which can be run using `cli-test`, which runs all `.clts` files by default. 

[More examples](./examples)

### Install

If you are a Rust user, you can install with Cargo:

```
cargo install cli-test
```

You can run using Docker:

```
(Under construction)
```

Or if your platform is supported, you can download the executable

```
(Under construction)
```

