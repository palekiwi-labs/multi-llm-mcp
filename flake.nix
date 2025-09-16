{
  description = "A Rust flake for test-runner-mcp";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    # agents = {
    #   url = "path:/home/pl/code/palekiwi-labs/agents/60d61319e96e82b09aa294ba03ec91816ae13f3e";
    #   inputs.nixpkgs.follows = "nixpkgs";
    # };
  };

  outputs = { nixpkgs, fenix, ... }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      rustToolchain = fenix.packages.${system}.stable.toolchain;

      # Common build dependencies
      commonNativeBuildInputs = [ ];
      commonBuildInputs = [ ];
    in
    {
      packages.${system}.default = pkgs.rustPlatform.buildRustPackage {
        pname = "test-runner-mcp";
        version = "0.1.0";
        src = ./.;

        cargoHash = "sha256-2oKq9byTo2+RcqpOdL3mQgcZtSeTgTYu2KK68JYEJpg=";

        nativeBuildInputs = commonNativeBuildInputs;
        buildInputs = commonBuildInputs;

        meta = with pkgs.lib; {
          description = "A Rust flake for test-runner-mcp";
          license = licenses.mit;
          maintainers = [ ];
        };
      };

      devShells.${system}.default = pkgs.mkShell
        {
          buildInputs = [
            rustToolchain
            pkgs.rust-analyzer
            pkgs.cargo-expand
            pkgs.cargo-watch
            pkgs.cargo-edit
          ] ++ commonNativeBuildInputs ++ commonBuildInputs;

          shellHook = ''
            echo "Rust development environment ready!"
            echo "Rust version: $(rustc --version)"
          '';
        };
    };
}
