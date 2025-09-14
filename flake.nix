{
  description = "PsychonautWiki Journal Rust TUI - Nix Flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    crane.url = "github:ipetkov/crane";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, crane }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustToolchain = pkgs.rust-bin.stable.latest.default;
        craneLib = crane.mkLib pkgs;
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = [ rustToolchain pkgs.pkg-config pkgs.openssl ];
          shellHook = ''
            export CARGO_TERM_COLOR=always
          '';
        };
        packages.default = craneLib.buildPackage {
          src = craneLib.cleanCargoSource ./.;
          # Optionally add extra build inputs here
          nativeBuildInputs = [ pkgs.openssl pkgs.pkg-config ];
        };
      }
    );
}
