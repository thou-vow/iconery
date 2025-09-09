{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = {
    self,
    nixpkgs,
  }: let
    pkgs = nixpkgs.legacyPackages."x86_64-linux";
    inherit (pkgs) lib;
  in {
    devShells."x86_64-linux".api = pkgs.mkShell {
      inputsFrom = [
        self.packages."x86_64-linux".api
      ];

      buildInputs = with pkgs; [
        sqlx-cli
      ];
    };

    packages."x86_64-linux".api = let
      deserializedManifest = lib.importTOML ./api/Cargo.toml;
    in
      pkgs.rustPlatform.buildRustPackage {
        pname = deserializedManifest.package.name;
        version = deserializedManifest.package.version;

        src = lib.fileset.toSource {
          root = ./.;
          fileset = lib.fileset.unions [
            # ./.sqlx
            ./api
            ./Cargo.lock
            ./Cargo.toml
          ];
        };

        cargoLock.lockFile = ./Cargo.lock;
        buildAndTestSubdir = "api";
      };
  };
}
