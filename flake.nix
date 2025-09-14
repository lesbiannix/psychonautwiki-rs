{
  description = "PsychonautWiki Journal TUI";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, rust-overlay }:
    let
      system = "x86_64-linux";
      overlays = [ rust-overlay.overlays.default ];
      pkgs = import nixpkgs { inherit system overlays; };
    in {
      devShells.x86_64-linux.default = pkgs.mkShell {
        buildInputs = [
          pkgs.rust-bin.stable.latest.default
          pkgs.pkg-config
          pkgs.openssl
        ];
        RUST_SRC_PATH = "${pkgs.rust-bin.stable.latest.default}/lib/rustlib/src/rust/library";
      };
      packages.x86_64-linux.default = pkgs.rustPlatform.buildRustPackage {
        pname = "psychonautwiki-rs";
        version = "0.1.0";
        src = ./.;
        cargoLock = {
          lockFile = ./Cargo.lock;
        };
      };
    };
}
