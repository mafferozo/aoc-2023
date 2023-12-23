{
  inputs = {
    naersk.url = "github:nix-community/naersk/master";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils, naersk }:
    utils.lib.eachDefaultSystem 
    (system:
        let
          pkgs = import nixpkgs { inherit system; };
          naersk-lib = pkgs.callPackage naersk { };
          nativeBuildInputs = with pkgs; [ pkg-config ];
          buildInputs = with pkgs; [ openssl ];
        in
        {
          packages.default = naersk-lib.buildPackage { src = ./.; inherit nativeBuildInputs; inherit buildInputs; };
          devShells.default = with pkgs; mkShell {
            inherit nativeBuildInputs;
            buildInputs = [ cargo rustc rustfmt rustPackages.clippy ] ++ buildInputs;
            RUST_SRC_PATH = rustPlatform.rustLibSrc;
          };
        }
    );
}
