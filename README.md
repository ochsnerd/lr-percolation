# Long-Range Percolation Core

## Build

Either (with flakes)

```bash
nix build  # builds the rust crate
nix shell .#lr_interactions-python-env # rebuilds and enters a shell with a python knowing lr_percolation
```

or manually (needs rust and python with maturin installed) (untested, see `nix/modules/python.nix`)

```bash
cargo build  # builds the rust crate
maturin build --release --features python-bindings
```

## Use

Create `flake.nix` with the following content (TODO: Update)

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
      envPython = pkgs.python313.withPackages(ps: with ps; [
        numpy
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

An example usage could look like this:
```python
import lr_interactions import percolation as lrp
import numpy as np
import matplotlib.pyplot as plt


def avg_qs_size(obs):
    return np.average([o.size_spread for o in obs])


def main():
    ls = [50, 100, 200]
    alpha = 10
    num_samples = 100
    seed = 42

    plt.figure(figsize=(10, 6))
    colors = [
        "blue",
        "red",
        "green",
        "purple",
        "orange",
    ]

    betas = list(np.arange(0.01, 0.4, 0.01))

    for i, (l, c) in enumerate(zip(ls, colors)):
        print(f"Processing L = {l}...")

        sizes = [
            avg_qs_size(
                lrp.simulate(
                    lrp.Norm.LInf,
                    l,
                    alpha,
                    beta,
                    num_samples,
                    seed + 1,
                )
            )
            for beta in betas
        ]
        plt.plot(betas, sizes, color=c, linestyle="-", linewidth=1)
        plt.scatter(betas, sizes, s=10, color=c, label=f"l = {l}")

    plt.legend(loc="best")
    plt.xlabel("Beta")
    plt.ylabel("S")
    plt.title(f"Results for Different L Values (Seed: {seed}), alpha={alpha}")
    plt.grid(True, alpha=0.3)

    plt.savefig(f"percolation_results_alpha_{alpha}.png")
    plt.show()


if __name__ == "__main__":
    main()
```

This takes 2 minutes on my machine and results in

![](data/percolation_results_sigma_10.png)

## Benchmark

Benchmark code can be found in `benches/`, and executed with `cargo bench`.
After executing `nix-shell -p gnuplot --run "cargo bench"`, results are in `target/criterion/report/index.html`.

# TODO

- finish migration to cargo workspace (atm the python package cannot be imported)
- update readme (nix shell command for ex, and check example)

1. figure out if what we're doing in the interface is grossly inefficient
   (we pass a list of custom objects, instead of an np-array of primitives)
2. How much work is it to implement LERW in rust?
