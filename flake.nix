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
        in
        {
          packages.default = naersk-lib.buildPackage ./.;
          devShells.default = with pkgs; mkShell {
            buildInputs = [ cargo rustc rustfmt rustPackages.clippy pkg-config openssl ];
            RUST_SRC_PATH = rustPlatform.rustLibSrc;
          };
        }
    );
}
