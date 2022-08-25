{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-22.05";
    crane.url = "github:ipetkov/crane";
    crane.inputs.nixpkgs.follows = "nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, crane, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };

        craneLib = crane.lib.${system};
        desktop_item = pkgs.makeDesktopItem {
          name = "flintstone";
          desktopName = "FlintStone";
          exec = "flintstone %u";
          mimeTypes = [ "text/markdown" ];
        };

        commonArgs = {
          src = ./.;
          buildInputs = with pkgs; [
            openssl
          ];

          nativeBuildInputs = with pkgs; [
            pkg-config
            copyDesktopItems
          ];


          desktopItems = [ desktop_item ];
        };

        # Build *just* the cargo dependencies, so we can reuse
        # all of that work (e.g. via cachix) when running in CI
        cargoArtifacts = craneLib.buildDepsOnly (pkgs.lib.recursiveUpdate
          commonArgs
          {
            pname = "flintstone";
          }
        );

        flintstoneClippy = craneLib.cargoClippy (pkgs.lib.recursiveUpdate commonArgs {
          inherit cargoArtifacts;
          cargoClippyExtraArgs = "--all-targets -- --deny warnings";
        });

        flintstone = craneLib.buildPackage (pkgs.lib.recursiveUpdate
          commonArgs
          {
            inherit cargoArtifacts;
          }
        );

      in
      {
        packages.default = flintstone;
        devShells.default = pkgs.mkShell (pkgs.lib.recursiveUpdate commonArgs {
          shellHook = ''
            git config core.hooksPath .githooks
            cargo-about generate about.hbs | sed "s/&quot;/'/g;s/&lt;/</g;s/&gt;/>/g;s/&#x27;/'/g" > thirdparty/THIRDPARTY
          '';

          nativeBuildInputs = with pkgs; [
            cargo-about
            convco
          ];

          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
        });
      });
}
