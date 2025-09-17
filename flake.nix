{
  description = "Virtual shop backend";

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
    devShells."x86_64-linux".default = pkgs.mkShell {
      inputsFrom = [
        self.packages."x86_64-linux".default
      ];

      buildInputs = with pkgs; [
        jq
        sqlx-cli
      ];

      # For sqlx compile-time checks
      shellHook = ''
        if [ -f ./config.json ]; then
          export DATABASE_URL=$(jq -r '"\(.db_client)://\(.db_user):\(.db_password)@\(.db_host):\(.db_port)/\(.db_name)"' ./config.json)
        else
          echo "Warning: config.json is missing"
        fi
      '';
    };

    packages."x86_64-linux".default = let
      meta = lib.importTOML ./Cargo.toml;
    in
      pkgs.rustPlatform.buildRustPackage {
        pname = meta.package.name;
        version = meta.package.version;

        src = lib.fileset.toSource {
          root = ./.;
          fileset = lib.fileset.unions [
            # ./.sqlx
            ./src
            ./Cargo.lock
            ./Cargo.toml
          ];
        };

        cargoLock.lockFile = ./Cargo.lock;

        nativeBuildInputs = with pkgs; [pkg-config];
        buildInputs = with pkgs; [openssl];
      };
  };
}
