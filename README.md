# numora

A small numerical lab in pure Rust: dense matrices, statistics, and a MATLAB-style expression language with an interactive shell. No dependencies.

> Work in progress. The matrix engine, statistics, parser and shell are being built one pull request at a time.

## Planned

- **Matrices:** arithmetic with scalar broadcasting, transpose, determinant, inverse, solving `A x = b`, rank, powers, concatenation and reshaping.
- **Statistics:** mean, median, mode, variance, standard deviation, percentiles, covariance, correlation and least-squares line fitting.
- **Language:** `A = [1 2; 3 4]`, `x = A \ [5; 6]`, `v = 1:0.5:3`, `A(2, :)`, element-wise `.*` `./` `.^`, comparisons, and built-in functions.
- **Shell:** `cargo run` starts an interactive session.

## Development

```bash
cargo test          # unit, integration and doc tests
cargo clippy        # lints
cargo fmt           # formatting
```

CI runs formatting, clippy, the tests and a coverage check on every push and pull request.

## License

MIT. See [LICENSE](LICENSE).
