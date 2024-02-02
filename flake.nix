{
  description = "Devenv for hoyolab-claim-bot";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    devenv.url = "github:cachix/devenv";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  nixConfig = {
    extra-trusted-public-keys = "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw=";
    extra-substituters = "https://devenv.cachix.org";
  };

  outputs =
    inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [ inputs.devenv.flakeModule ];
      systems = [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" ];

      perSystem = { config, self', inputs', pkgs, system, lib, ... }:
        {
          packages = rec {
            hoyolab-claim-bot = pkgs.callPackage ./default.nix { };
            default = hoyolab-claim-bot;
          };
          devenv.shells.default = let
            libs = with pkgs; [ openssl ];
          in {
            name = "hoyolab-claim-bot";
            env = {
              LD_LIBRARY_PATH = lib.makeLibraryPath libs;
            };
            packages = with pkgs; libs ++ [
              cargo-audit
              cargo-udeps
              cargo-unused-features
              nixfmt-rfc-style
            ] ++ lib.optionals stdenv.isDarwin (
              with pkgs.darwin.apple_sdk; [ frameworks.Security ]
            );
            languages.rust = {
              enable = true;
              channel = "nightly";
              components = [
                "rustc"
                "cargo"
                "clippy"
                "rustfmt"
                "rust-analyzer"
                "rust-docs"
              ];
            };
          };
        };
      flake = { };
    };
}
