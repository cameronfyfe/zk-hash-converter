{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    nixpkgs-risc0.url = "github:cameronfyfe/nixpkgs/add-cargo-risc0";
    nixpkgs-present.url = "github:cameronfyfe/nixpkgs/add-present-cli";
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

        present-cli = (import inputs.nixpkgs-present {
          inherit system;
        }).present-cli;

        rust-config = {
          extensions = [ "rust-src" ];
          targets = [ "wasm32-unknown-unknown" ];
        };

        rust = (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain).override rust-config;

        eqty.pkgs = inputs.eqty-nix.packages.${system};

        shellPkgs = [
          cargo-risczero
          present-cli
          rust
        ] ++ (with pkgs; [
          bc
          cargo-audit
          ets
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
        };

      }));
}
