{ inputs, ... }: {
  perSystem = { config, self', pkgs, lib, system, ... }:
    let
      # underscore to get a valid python package name
      project_name = config.rust-project.crates."lr-percolation".name or "lr_percolation";
      project_version = config.rust-project.crates."lr-percolation".version or "0.1.0";

      # Python configuration
      python = pkgs.python313;
      wheel_tail = "cp313-cp313-linux_x86_64"; # Change if pythonVersion changes
      wheel_name = "${project_name}-${project_version}-${wheel_tail}.whl";

      # Build a Python wheel using maturin
      crate_wheel = config.packages.lr-percolation.overrideAttrs (old: {
        nativeBuildInputs = old.nativeBuildInputs ++ [
          pkgs.maturin
          (python.withPackages (ps: with ps; [ cffi setuptools pip wheel ]))
        ];
        buildPhase = old.buildPhase + ''
          maturin build --release --offline --target-dir ./target --features python-bindings
        '';
        installPhase = old.installPhase + ''
          ls target/wheels
          cp target/wheels/${wheel_name} $out/
        '';
      });

      # Create a Python package from the wheel
      pythonPackage = python.pkgs.buildPythonPackage {
        pname = project_name;
        version = project_version;
        format = "wheel";
        src = "${crate_wheel}/${wheel_name}";
        propagatedBuildInputs = [ python.pkgs.cffi ];
        doCheck = false;
        pythonImportsCheck = [ "lr_percolation" ];
      };

    in
    {
      # Add the Python wheel and package to the packages set
      packages = {
        "${project_name}-wheel" = crate_wheel;
        "${project_name}-python" = pythonPackage;

        # Create a Python environment with our package installed
        "${project_name}-python-env" = python.withPackages (ps: [
          pythonPackage
        ]);
      };

      # Expose a function for other flakes to use this package
      legacyPackages = {
        pythonModules = {
          "${project_name}" = pythonPackage;
        };
      };
    };
}
