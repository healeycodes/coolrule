[![Rust](https://github.com/healeycodes/coolrule/actions/workflows/rust.yml/badge.svg)](https://github.com/healeycodes/coolrule/actions/workflows/rust.yml)

# coolrule

Boolean expression evaluation engine (a port of [boolrule](https://github.com/tailsdotcom/boolrule) to Rust).

```rust
// Without context
let expr = coolrule::new("1 in (1, 2, 3) or 2 > 3")?;
let test = expr.test()?; // true

// With context
let expr = coolrule::new("x ∉ (5, 6, 7)")?;
let test = expr.test_with_context(
    HashMap::from([(vec!["x"], Value::Number(8.0))])
)?; // true
```

The boolrule test suite has also been ported (and passes) see `lib.rs`.

Expressions are parsed via PEG parser combinators (powered by [pom](https://github.com/J-F-Liu/pom)).

It's around 3x faster than the Python version (before any kind of optimization work).

## Tests

`cargo test`
