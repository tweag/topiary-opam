[OPAM] Package for [Topiary]
============================

This repository provides an OPAM package for [Topiary]. In particular, issues
with Topiary itself should be reported [there][topiary-github-issues] while
issues with Topiary's packaging in OPAM should be reported [here][issues].

[opam]: https://opam.ocaml.org/
[topiary]: https://topiary.tweag.io/
[topiary-github-issues]: https://github.com/tweag/topiary/issues
[issues]: https://github.com/Niols/topiary-opam/issues

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

How this works
--------------

This repository is made of the following building blocks:

- `topiary/` is a [Git submodule] pointing to [Topiary's Git][topiary-github] at
  a certain commit (in general at a certain tag).

- `Cargo.toml` and `Cargo.lock` define a Cargo workspace containing only
  Topiary. This allows working with Cargo to vendor dependencies without
  changing the content of the `topiary/` directory.

- `vendor/` is a directory filled automatically by Cargo and containing all the
  dependencies of Topiary, vendored. This is a way to provide all the
  dependencies at upfront and to avoid having Cargo download them during the
  installation.

- `make-topiary-wrapper.sh` is a Shell script creating a wrapper around Topiary
  that provides it with the right environment. In particular, Topiary needs to
  be made aware of where it can find its “language files”.

- `topiary.opam` contains the definition of the OPAM package for Topiary.

[topiary-github]: https://github.com/tweag/topiary
[git submodule]: https://git-scm.com/book/en/v2/Git-Tools-Submodules

How to update
-------------

- Update the [Git submodule] containing Topiary:
  ```console
  $ git submodule update --remote
  Submodule path 'topiary': checked out 'f99bcd59e2a247e04b31b16fc9214460012e3713'
  ```

- Make sure the Git submodule is checked out at a tag of your choosing:
  ```console
  $ cd topiary
  $ git checkout v0.1.0
  HEAD is now at c4fe76c GraphViz visualisation support (#326)
  $ cd ..
  ```

- Commit this update:
  ```console
  $ git add topiary/
  $ git commit -m 'Bump submodule to v0.1.0'
  [main 3bb3a28] Bump submodule to v0.1.0
   1 file changed, 1 insertion(+), 1 deletion(-)
  ```

- Refresh the `Cargo.lock` file:
  ```console
  $ cargo update
      Updating crates.io index
      Updating git repository `https://github.com/tree-sitter/tree-sitter-bash`
      Updating git submodule `https://git.savannah.gnu.org/git/bash.git`
  [...]
      Updating git repository `https://github.com/nvim-treesitter/tree-sitter-query`
  ```

- Regenerate the `vendor/` directory:
  ```console
  $ cargo vendor
    Downloaded hermit-abi v0.1.19
    Downloaded is-terminal v0.4.7
    Downloaded instant v0.1.12
  [...]
     Vendoring indexmap v1.9.3 (/home/niols/.cargo/registry/src/github.com-1ecc6299db9ec823/indexmap-1.9.3) to vendor/indexmap
     Vendoring instant v0.1.12 (/home/niols/.cargo/registry/src/github.com-1ecc6299db9ec823/instant-0.1.12) to vendor/instant
     Vendoring io-lifetimes v1.0.10 (/home/niols/.cargo/registry/src/github.com-1ecc6299db9ec823/io-lifetimes-1.0.10) to vendor/io-lifetimes
     Vendoring is-terminal v0.4.7 (/home/niols/.cargo/registry/src/github.com-1ecc6299db9ec823/is-terminal-0.4.7) to vendor/is-terminal
  [...]
  [source.vendored-sources]
  directory = "vendor"
  ```

- Commit this update:
  ```console
  $ git add Cargo.lock vendor/
  $ git commit -m 'Update Cargo dependencies'
  [main 95d67dc] Update Cargo dependencies
   8 files changed, 125 insertions(+), 57 deletions(-)
  ```

- Adapt the OPAM package or the other files if necessary and commit the changes.

- Open a [new pull request] and check that the continuous integration is happy
  with the current status of things.

- Merge the pull request in question and add a tag mimmicking that of Topiary.

- Send the new package to the [OPAM repository].

[new pull request]: https://github.com/Niols/topiary-opam/compare
[opam repository]: https://github.com/ocaml/opam-repository
