{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    nixpkgs-risc0.url = "github:cameronfyfe/nixpkgs/add-cargo-risc0";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs @ { self, ... }:
    (inputs.flake-utils.lib.eachSystem [ "x86_64-linux" ] (system:
      let

        pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [ inputs.rust-overlay.overlays.default ];
        };

        cargo-risczero = (import inputs.nixpkgs-risc0 {
          inherit system;
        }).cargo-risczero;

        rust-config = {
          extensions = [ "rust-src" ];
          targets = [ "wasm32-unknown-unknown" ];
        };

        rust = (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain).override rust-config;

        # rust-nightly used for advanced options in rustfmt
        rust-nightly = pkgs.rust-bin.nightly.latest.default.override rust-config;

        eqty.pkgs = inputs.eqty-nix.packages.${system};

        shellPkgs = [
          cargo-risczero
          rust
        ] ++ (with pkgs; [
          bc
          cargo-audit
          just
          nixpkgs-fmt
          openssl
          pkg-config
        ]);

      in
      rec {

        devShells = {
          default = pkgs.mkShell {
            nativeBuildInputs = shellPkgs;
          };
          rust-nightly = pkgs.mkShell {
            nativeBuildInputs = [ rust-nightly ];
          };
        };

      }));
}
