{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };
  outputs = { self, ... } @ inputs:
    inputs.flake-utils.lib.eachDefaultSystem
      (system:
        let
          pkgs = inputs.nixpkgs.outputs.legacyPackages.${system};
        in
        {
          packages.codid = pkgs.callPackage ./codid.nix { };
          packages.default = self.outputs.packages.${system}.codid;

          devShells.default = self.packages.${system}.default.overrideAttrs (super: {
            nativeBuildInputs = with pkgs;
              super.nativeBuildInputs
              ++ [
                clippy
                rustfmt
              ];
            RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
          });
        })
    // {
      overlays.default = final: prev: {
        inherit (self.packages.${final.system}) codid;
      };
    };
}
