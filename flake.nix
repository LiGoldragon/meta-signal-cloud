{
  description = "meta-signal-cloud — owner cloud provider Signal contract";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      fenix,
      crane,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
        toolchain = fenix.packages.${system}.stable.withComponents [
          "cargo"
          "rustc"
          "rustfmt"
          "clippy"
          "rust-analyzer"
          "rust-src"
        ];
        craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;
        schemaFilter = path: type: type == "regular" && pkgs.lib.hasSuffix ".schema" path;
        sourceFilter = path: type: (craneLib.filterCargoSources path type) || (schemaFilter path type);
        src = pkgs.lib.cleanSourceWith {
          src = ./.;
          filter = sourceFilter;
          name = "source";
        };
        commonArgs = {
          inherit src;
          strictDeps = true;
        };
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
      in
      {
        packages.default = craneLib.buildPackage (
          commonArgs
          // {
            inherit cargoArtifacts;
          }
        );

        checks = {
          build = craneLib.cargoBuild (
            commonArgs
            // {
              inherit cargoArtifacts;
            }
          );

          test = craneLib.cargoTest (
            commonArgs
            // {
              inherit cargoArtifacts;
            }
          );

          test-nota-text = craneLib.cargoTest (
            commonArgs
            // {
              inherit cargoArtifacts;
              cargoTestExtraArgs = "--features nota-text --all-targets";
            }
          );

          clippy-nota-text = craneLib.cargoClippy (
            commonArgs
            // {
              inherit cargoArtifacts;
              cargoClippyExtraArgs = "--features nota-text --all-targets -- -D warnings";
            }
          );

          fmt = craneLib.cargoFmt {
            inherit src;
          };
        };

        devShells.default = pkgs.mkShell {
          name = "meta-signal-cloud";
          packages = [
            pkgs.jujutsu
            toolchain
          ];
        };
      }
    );
}
