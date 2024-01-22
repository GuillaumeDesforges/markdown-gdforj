{
  outputs = { flake-utils, nixpkgs, ... }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          devShells.default = import ./devShell.nix
            {
              inherit nixpkgs pkgs;
            };
          packages.default = pkgs.callPackage ./package.nix { };
        }
      );
}
