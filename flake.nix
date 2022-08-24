{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-22.05";
    flake-utils.url = "github:numtide/flake-utils";
	flake-compat = {
		url = "github:edolstra/flake-compat";
		flake = false;
    };
  };

  outputs = { self, nixpkgs, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };

        desktop_item = pkgs.makeDesktopItem {
          name = "flintstone";
          desktopName = "FlintStone";
          exec = "flintstone";
          mimeTypes = [ "text/markdown" ];
        };
      in
      {
        defaultPackage = pkgs.rustPlatform.buildRustPackage rec {
          pname = "flintstone";
          version = "0.1";
          src = self;

          nativeBuildInputs = with pkgs; [
            # basic
            rustc
            cargo

            # this hook is needed for desktop items
            copyDesktopItems
          ];

          desktopItems = [ desktop_item ];
          cargoSha256 = "sha256-FgptQpre1ibSSzIlvOD1dPZdB9f8WPnnAVns+C0A/Hc=";
        };

        devShells.default = pkgs.mkShell {
          shellHook = ''
            git config core.hooksPath .githooks
          '';
          buildInputs = with pkgs;
            [
              # basic
              rustc
              cargo

              # for development
              rustfmt

              # custom
              cargo-about
              upx

              # needed for SSL
              openssl
              pkg-config

              git
            ];

          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        };


      }
    );
}
