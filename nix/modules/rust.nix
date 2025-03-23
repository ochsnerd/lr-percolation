{ inputs, ... }:
{
  imports = [
    inputs.rust-flake.flakeModules.default
    inputs.rust-flake.flakeModules.nixpkgs
  ];
  perSystem = { config, self', pkgs, lib, ... }: {
    rust-project.crates."lr-percolation".crane.args = {
      buildInputs = (lib.optionals pkgs.stdenv.isDarwin (
        with pkgs.darwin.apple_sdk.frameworks; [
          IOKit
        ]
      )) ++ [
        pkgs.python3
      ];

      # pyO3 wants a python executable
      env = {
        PYTHONPATH = "${pkgs.python3}/lib";
        PYO3_PYTHON = "${pkgs.python3}/bin/python";
      };
    };
    packages.default = self'.packages.lr-percolation;
  };
}
