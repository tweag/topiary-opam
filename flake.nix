{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";

    pre-commit-hooks = {
      url = "github:cachix/pre-commit-hooks.nix";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.nixpkgs-stable.follows = "";
    };
  };

  outputs = inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" ];

      imports = [ inputs.pre-commit-hooks.flakeModule ];

      perSystem = { config, pkgs, ... }: {
        formatter = pkgs.nixfmt;

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [ cargo rustc git-archive-all ];
          shellHook = config.pre-commit.installationScript;
        };

        pre-commit.settings.hooks = {
          nixfmt = {
            enable = true;
            excludes = [ "vendor/.*" ];
          };
          deadnix = {
            enable = true;
            excludes = [ "vendor/.*" ];
          };
          opam-lint.enable = true;
        };
      };
    };
}
