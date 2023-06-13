OPAM Package for Topiary
========================

This repository provides an [OPAM] package for [Topiary]. In particular, issues
with Topiary itself should be reported [there][topiary-github-issues] while
issues with Topiary's packaging in OPAM should be reported [here][issues].

[opam]: https://opam.ocaml.org/
[topiary]: https://topiary.tweag.io/
[topiary-github-issues]: https://github.com/tweag/topiary/issues
[issues]: https://github.com/tweag/topiary-opam/issues

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

Availability
------------

The compilation of Topiary depends on the presence of Rust on the system in a
recent enough version. Currently, Rust 1.65.0 is required. [Our continuous
integration] keeps track of several distributions whose Rust version is listed
here for convenience:

[our continuous integration]: ./.github/workflow/ci.yml

| Distribution    | Rust Version
|-----------------|--------------
| Alpine          | [![version in Alpine 3.18][alpine_3_18]][rust]
| Archlinux       | [![version in Arch][arch]][rust]
| CentOS          |
| Debian          | [![version in Debian 11][debian_11]][rust]
| Debian Testing  | [![version in Debian 12][debian_12]][rust]
| Debian Unstable | [![version in Debian unstable][debian_unstable]][rust]
| Fedora          | [![version in Fedora 38][fedora_38]][rust]
| OracleLinux     |
| openSUSE        | [![version in openSUSE Leap 15.5][opensuse_leap_15_5]][rust]
| Ubuntu          | [![version in Ubuntu 23.04][ubuntu_23_04]][rust]
| Ubuntu LTS      | [![version in Ubuntu 23.04][ubuntu_23_04]][rust]

[alpine_3_18]: https://repology.org/badge/version-for-repo/alpine_3_18/rust.svg?header=&minversion=1.65.0
[arch]: https://repology.org/badge/version-for-repo/arch/rust.svg?header=&minversion=1.65.0
[debian_11]: https://repology.org/badge/version-for-repo/debian_11/rust.svg?header=&minversion=1.65.0
[debian_12]: https://repology.org/badge/version-for-repo/debian_12/rust.svg?header=&minversion=1.65.0
[debian_unstable]: https://repology.org/badge/version-for-repo/debian_unstable/rust.svg?header=&minversion=1.65.0
[fedora_38]: https://repology.org/badge/version-for-repo/fedora_38/rust.svg?header=&minversion=1.65.0
[opensuse_leap_15_5]: https://repology.org/badge/version-for-repo/opensuse_leap_15_5/rust.svg?header=&minversion=1.65.0
[ubuntu_23_04]: https://repology.org/badge/version-for-repo/ubuntu_23_04/rust.svg?header=&minversion=1.65.0
[rust]: https://repology.org/project/rust/versions

How this works
--------------

This repository is made of the following building blocks:

- `topiary/` is a [Git submodule] pointing to [Topiary's Git][topiary-github] at
  a certain commit (in general at a certain tag). The file `.gitmodules`
  declares the submodule to Git.

- `Cargo.toml` and `Cargo.lock` define a Cargo workspace containing only
  Topiary. This allows working with Cargo to vendor dependencies without
  changing the content of the `topiary/` directory.

- `vendor/` is a directory filled automatically by Cargo and containing all the
  dependencies of Topiary, vendored. This is a way to provide all the
  dependencies at upfront and to avoid having Cargo download them during the
  installation. The file `.cargo/config.toml` contains a configuration that
  tells Cargo to look into `vendor/` instead of its usual sources.

- `make-topiary-wrapper.sh` is a Shell script creating a wrapper around Topiary
  that provides it with the right environment. In particular, Topiary needs to
  be made aware of where it can find its “language files”.

- `topiary.opam` contains the definition of the OPAM package for Topiary.

[topiary-github]: https://github.com/tweag/topiary
[git submodule]: https://git-scm.com/book/en/v2/Git-Tools-Submodules

How to update
-------------

- Update the [Git submodule] containing Topiary. Make sure it is checked out at
  a tag of your choosing:
  ```console
  $ git submodule update --remote
  Submodule path 'topiary': checked out 'f99bcd59e2a247e04b31b16fc9214460012e3713'
  ```

- Make sure the Git submodule is checked out at a tag of your choosing:
  ```console
  $ cd topiary
  $ git fetch
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

- Remove the previous local Cargo configuration and vendoring:
  ```console
  $ rm .cargo/config.toml
  $ rm -R vendor/
  ```
  Otherwise, the next step will yield an error, something in the lines of:
  “failed to get `<whatever>` as a dependency of package `topiary`”.

- Update the `Cargo.toml` file. This usually consists in copying the content of
  `topiary/Cargo.toml` file, except for the `workspace.members` attribute. This
  step should include bumping the version number.

- Refresh the `Cargo.lock` file:
  ```console
  $ cargo update
      Updating crates.io index
      Updating git repository `https://github.com/tree-sitter/tree-sitter-bash`
      Updating git submodule `https://git.savannah.gnu.org/git/bash.git`
  [...]
      Updating git repository `https://github.com/nvim-treesitter/tree-sitter-query`
  ```
  This may take a couple of minutes.

- Regenerate the `vendor/` directory:
  ```console
  $ cargo vendor
    Downloaded hermit-abi v0.1.19
    Downloaded is-terminal v0.4.7
    Downloaded instant v0.1.12
  [...]
     Vendoring indexmap v1.9.3 (~/.cargo/registry/src/github.com-1ecc6299db9ec823/indexmap-1.9.3) to vendor/indexmap
     Vendoring instant v0.1.12 (~/.cargo/registry/src/github.com-1ecc6299db9ec823/instant-0.1.12) to vendor/instant
     Vendoring io-lifetimes v1.0.10 (~/.cargo/registry/src/github.com-1ecc6299db9ec823/io-lifetimes-1.0.10) to vendor/io-lifetimes
     Vendoring is-terminal v0.4.7 (~/.cargo/registry/src/github.com-1ecc6299db9ec823/is-terminal-0.4.7) to vendor/is-terminal
  [...]
  To use vendored sources, add this to your .cargo/config.toml for this project:

  [source.crates-io]
  replace-with = "vendored-sources"

  [...]

  [source.vendored-sources]
  directory = "vendor"
  ```
  Note in particular the last lines about adding something to the local Cargo
  configuration.

- Recreate the local Cargo configuration with the content given by `cargo
  vendor`.
  ```console
  $ nano .cargo/config.toml
  ```

- Commit this update:
  ```console
  $ git add Cargo.toml Cargo.lock vendor/ .cargo/
  $ git commit -m 'Update Cargo dependencies'
  [main 95d67dc] Update Cargo dependencies
   8 files changed, 125 insertions(+), 57 deletions(-)
  ```

- Adapt the OPAM package or the other files if necessary and commit the changes.

- Open a [new pull request] and check that the continuous integration is happy
  with the current status of things. Merge the pull request in question.

- Add a tag mimmicking that of Topiary (eg. `v0.1.0` for Topiary's `v0.1.0`).

- Create an archive containing all the content of this repository at that tag:
  ```
  $ git-archive-all source-code-with-submodules.tar.xz
  ```
  This will be necessary to provide a downloadable archive that contains the
  files from all the submodules.

- Create a release for the tag in question. Link to the corresponding release in
  Topiary. Attach the archive.

- Compute the MD5 and SHA512 sums of the archive in question:
  ```
  $ md5sum source-code-with-submodules.tar.xz
  cd825a17db25cb94fd876eef055090e4  source-code-with-submodules.tar.xz
  $ sha512sum source-code-with-submodules.tar.xz
  ae6946aaba0f784773cca71019f73aa62d9b976646ea25e451c220f45da49e6c7e4147e2dd57e3c4764a9038946c38b9de33ce5d463c46ea3f3271d5b98dd46f  source-code-with-submodules.tar.xz
  ```

- Send the new package to the [OPAM repository]. The `src` field of the `url`
  object should be:
  ```
  https://github.com/tweag/topiary-opam/releases/download/<tag>/source-code-with-submodules.tar.xz
  ```

[new pull request]: https://github.com/tweag/topiary-opam/compare
[opam repository]: https://github.com/ocaml/opam-repository
