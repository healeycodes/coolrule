# coolrule

Boolean expression evaluation engine (a port of [boolrule](https://github.com/tailsdotcom/boolrule) to Rust).

The boolrule test suite has also been ported (and passes) see `lib.rs`.

Expressions are parsed via PEG parser combinators (powered by [pom](https://github.com/J-F-Liu/pom)).

It's around 3x faster than the Python version (before any kind of optimization work).

## Tests

`cargo test`
