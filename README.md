[OPAM] Package for [Topiary]
============================

[opam]: https://opam.ocaml.org/
[topiary]: https://topiary.tweag.io/
[topiary-github]: https://github.com/tweag/topiary

This repository provides an OPAM package for [Topiary]. For Topiary's GitHub
repository, see [here][topiary-github].

Context
-------

Topiary is written in Rust and is therefore not an OCaml project _per se_.
However, as it allows formatting OCaml (among other languages), it does belong
to its ecosystem and, therefore, it makes sense to package it via OPAM.

This is actually not so complicated. OPAM “is a source-based package manager for
OCaml”, which prevents using it to distribute precompiled binaries, but it is
actually OCaml agnostic, meaning it is possible to call `cargo build` in an OPAM
package.

The difficulty is then that such a package depends on the presence of Rust on
the system in the right version. Additionally, Cargo tends to download Rust
dependencies when building which cannot possibly happen in the OPAM sandbox. One
way around this is to provide another repository containing Topiary and all its
dependencies as well as what is necessary to get the OPAM package working. This
is where you are.

Credits are due to all the participants of [this OCaml discuss thread] for their
encouragements and their ideas and to the people behind [`tezos-rust-libs`] on
which this package is based.

[this OCaml discuss thread]: https://discuss.ocaml.org/t/two-questions-about-what-is-appropriate-to-package-with-opam/12030/
[`tezos-rust-libs`]: https://gitlab.com/tezos/tezos-rust-libs/-/tree/master
