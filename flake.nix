{
  description = "LiveJuke monorepo dev shells (Rust API + Expo RN)";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };

        rustToolchain = with pkgs.rust-bin; [
          (stable.latest.minimal.override {
            extensions = [
              "clippy"
              "rust-src"
            ];
          })

          nightly.latest.rustfmt
          nightly.latest.rust-analyzer
        ];

        common = with pkgs; [
          git
          curl
          jq
          just
        ];

        apiPkgs =
          (with pkgs; [
            pkg-config
            postgresql
          ])
          ++ rustToolchain;

      in
      {
        devShells.default = pkgs.mkShell {
          packages = common ++ apiPkgs;
        };
      }
    );
}
