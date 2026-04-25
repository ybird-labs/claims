{
  description = "claims";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane.url = "github:ipetkov/crane";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      crane,
      rust-overlay,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachSystem
      [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ]
      (
        system:
        let
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs {
            inherit system overlays;
          };

          toolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
          craneLib = (crane.mkLib pkgs).overrideToolchain toolchain;
          src = craneLib.cleanCargoSource ./.;
          systemDeps = with pkgs; [  ];

          commonArgs = {
            inherit src;
            pname = "claims";
            version = "0.1.0";
            strictDeps = true;
            buildInputs = systemDeps;
          };

          cargoArtifacts = craneLib.buildDepsOnly commonArgs;
          package = craneLib.buildPackage (
            commonArgs
            // {
              inherit cargoArtifacts;
            }
          );
        in
        {
          packages.default = package;

          checks = {
            default = package;
            fmt = craneLib.cargoFmt {
              inherit src;
              pname = "claims";
              version = "0.1.0";
            };
            clippy = craneLib.cargoClippy (
              commonArgs
              // {
                inherit cargoArtifacts;
                cargoClippyExtraArgs = "--workspace --all-targets -- --deny warnings";
              }
            );
            test = craneLib.cargoTest (
              commonArgs
              // {
                inherit cargoArtifacts;
                cargoTestExtraArgs = "--workspace --all-targets";
              }
            );
          };

          devShells.default = pkgs.mkShell {
            packages = (with pkgs; [
              toolchain
              just
            ]) ++ systemDeps;

            RUST_SRC_PATH = "${toolchain}/lib/rustlib/src/rust/library";
          };

          formatter = pkgs.nixfmt;
        }
      );
}
