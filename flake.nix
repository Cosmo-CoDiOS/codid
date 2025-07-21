{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };
  outputs = inputs: let
    inherit (inputs) self flake-utils nixpkgs;
  in
    flake-utils.lib.eachDefaultSystem
    (system: let
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      packages = {
        codid = pkgs.callPackage ./codid.nix {};
        default = self.packages.${system}.codid;
      };

      devShells.default = pkgs.mkShell {
        inputsFrom = with self.packages.${system}; [codid];
        RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
      };
    })
    // {
      overlays.default = final: prev: {
        inherit (self.packages.${final.system}) codid;
      };
    };
}
