{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";

    nixpkgs-mozilla = {
      url = "github:mozilla/nixpkgs-mozilla";
      flake = false;
    };
  };

  outputs = { self, flake-utils, naersk, nixpkgs, nixpkgs-mozilla }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = (import nixpkgs) {
          inherit system;

          overlays = [
            (import nixpkgs-mozilla)
          ];
        };

        toolchain = (pkgs.rustChannelOf {
          rustToolchain = ./rust-toolchain;
          sha256 = "sha256-PjvuouwTsYfNKW5Vi5Ye7y+lL7SsWGBxCtBOOm2z14c=";
        }).rust;

        naersk' = pkgs.callPackage naersk {
          cargo = toolchain;
          rustc = toolchain;
        };

      in
      {
        # For `nix build` & `nix run`:
        packages.codid = naersk'.buildPackage {
          src = ./.;
          nativeBuildInputs = with pkgs; [ pkg-config cmake ];
          buildInputs = with pkgs; [ systemd.dev dbus.dev protobuf protobufc ];
        };

        packages.default = self.outputs.packages.${system}.codid;

        # For `nix develop` (optional, can be skipped):
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = [ toolchain ] ++ (with pkgs; [ rustc cargo pkg-config cmake ]);
          buildInputs = with pkgs; [ systemd.dev dbus.dev protobuf protobufc ];
        };
        overlays.default = final: prev: {
          inherit (self.packages.${final.system}) codid;
        };
      }
    );
}
