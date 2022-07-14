# Requirements
- Bazel 5.2.0
- `cargo-bazel` 0.7.0 from https://github.com/bazelbuild/rules_rust/releases/tag/0.7.0

# Build

1. Place the `cargo-bazel` executable in `/tmp`
2. Set `CARGO_BAZEL_GENERATOR_URL` to point to the `cargo-bazel` executable
```shell
# Example using bash and Linux
$ export CARGO_BAZEL_GENERATOR_URL=file:/tmp/cargo-bazel-x86_64-unknown-linux-gnu
```
3. Sync the crate index
```shell
$ CARGO_BAZEL_REPIN=1 bazel sync --only=crate_index 
```
4. Run the test coverage
```shell
$ bazel coverage --combined_report=lcov //p1:p1_test 
```
5. Generate the coverage report
```shell
$ genhtml --output genhtml "$(bazel info output_path)/_coverage/_coverage_report.dat" 
```

The report generation will fail with

```shell
genhtml: ERROR: cannot read /rustc/[SOME_HASH]/library/std/src/thread/local.rs
```