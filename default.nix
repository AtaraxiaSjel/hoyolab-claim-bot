{ lib, rustPlatform, openssl, pkg-config }:
rustPlatform.buildRustPackage {
  pname = "hoyolab-claim-bot";
  version = "1.0.0";
  src = lib.cleanSource ./.;
  cargoLock.lockFile = ./Cargo.lock;

  nativeBuildInputs = [ pkg-config ];
  buildInputs = [ openssl ];

  meta = with lib; {
    description = "Hoyolab daily claim bot for Hoyoverse games";
    homepage = "https://github.com/AtaraxiaSjel/hoyolab-claim-bot";
    license = licenses.mit;
    maintainers = with maintainers; [ ataraxiasjel ];
    platforms = [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" ];
  };
}