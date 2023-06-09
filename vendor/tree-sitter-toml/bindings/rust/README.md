# tree-sitter-toml

This crate provides TOML grammar for the [tree-sitter][] parsing library. To
use this crate, add it to the `[dependencies]` section of your `Cargo.toml`
file. (Note that you will probably also need to depend on the
[`tree-sitter`][tree-sitter crate] crate to use the parsed result in any useful
way.)

``` toml
[dependencies]
tree-sitter = "0.17"
tree-sitter-toml = "0.16"
```

Typically, you will use the [language][language func] function to add this
grammar to a tree-sitter [Parser][], and then use the parser to parse some code:

```rust
let code = r#"
[package]
name = "cargo"
version = "0.1.0"
edition = "2021"
"#;
let mut parser = Parser::new();
parser.set_language(tree_sitter_toml::language()).expect("Error loading TOML grammar");
let parsed = parser.parse(code, None);
```

It's based on the lovely bindings of [tree-sitter-rust][] and uses the awesome grammar
defined by [tree-sitter-toml][].

[Language]: https://docs.rs/tree-sitter/*/tree_sitter/struct.Language.html
[language func]: https://docs.rs/tree-sitter-toml/*/tree_sitter_rust/fn.language.html
[Parser]: https://docs.rs/tree-sitter/*/tree_sitter/struct.Parser.html
[tree-sitter]: https://tree-sitter.github.io/
[tree-sitter crate]: https://crates.io/crates/tree-sitter
[tree-sitter-rust]: https://github.com/tree-sitter/tree-sitter-rust/tree/master/bindings/rust
[tree-sitter-toml]: https://github.com/ikatyang/tree-sitter-toml
