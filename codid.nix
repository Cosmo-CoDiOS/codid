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

  nativeBuildInputs = with pkgs; [ pkg-config ];
  buildInputs = with pkgs; [ systemd.dev ];

  meta = with lib; {
    description = "";
    homepage = "https://github.com/Cosmo-CoDiOS/codid";
    license = licenses.mit;
  };
}
