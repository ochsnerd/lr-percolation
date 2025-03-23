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
      ))
      ++ [
        # TODO: This has the be the same python as in python.nix:9
        # how to enforce that?
        pkgs.python313
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
