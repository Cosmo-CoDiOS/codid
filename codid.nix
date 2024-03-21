{ lib
, pkgs ? import <nixpkgs>
, rustPlatform
,
}:
rustPlatform.buildRustPackage {
  name = "codid";

  src = lib.cleanSource ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
    allowBuiltinFetchGit = true;
  };

  cargoBuildFlags = [
    "--features=stock-codi,codios-codi,nixos"
  ];

  nativeBuildInputs = with pkgs; [ pkg-config protobuf ];
  buildInputs = with pkgs; [ systemd.dev ];

  meta = with lib; {
    description = "";
    homepage = "https://github.com/Cosmo-CoDiOS/codid";
    license = licenses.mit;
  };
}
