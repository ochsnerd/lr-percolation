# Long-Range Percolation Core

## Build

Either (with flakes)
```bash
nix build
```
or manually
```bash
cargo build
```

## Use

Create `flake.nix` with the following content

```nix
{
  description = "Percolation experiments";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs?ref=nixpkgs-unstable";
    utils.url = "github:numtide/flake-utils";
    lr-percolation.url = "github:ochsnerd/lr-percolation";
  };
  outputs = { self, nixpkgs, utils, lr-percolation }: (utils.lib.eachSystem ["x86_64-linux" ] (system:
    let
      pkgs = nixpkgs.legacyPackages.${system};
      lrPython = lr-percolation.outputs.packages.${system}.lr_percolation-python-env;
      envPython = pkgs.python310.withPackages(ps: with ps; [
        numpy
        pandas
        matplotlib
        # add other python packages here
      ]);
    in rec {
      packages = {
        pythonEnv = envPython;
        lrPercolationEnv = lrPython;
      };

      defaultPackage = packages.pythonEnv;

      devShell = pkgs.mkShell {
        buildInputs = [
          envPython
          lrPython
        ];

        shellHook = ''
          export PYTHONPATH="${lrPython}/${lrPython.sitePackages}:${envPython}/${envPython.sitePackages}:$PYTHONPATH"
        '';
      };
    }
  ));
}
```

Then run `nix develop` to get a shell with the python environment.
