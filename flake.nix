{
  description = "learn-rust – Gamified Rust Learning Campaign";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      supportedSystems = [
        "aarch64-darwin"
        "x86_64-darwin"
        "aarch64-linux"
        "x86_64-linux"
      ];
      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
      pkgsFor = system: nixpkgs.legacyPackages.${system};
    in
    {
      # ── Dev shell ─────────────────────────────────────────────────────
      # Provides a pinned Rust toolchain.  No rustup, Homebrew, or host
      # Rust installation required.
      devShells = forAllSystems (system:
        let pkgs = pkgsFor system; in {
          default = pkgs.mkShell {
            packages = with pkgs; [
              cargo
              rustc
              rustfmt
              clippy
              rust-analyzer
            ];
            # Helps rust-analyzer and IDEs locate standard-library source.
            RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
          };
        }
      );

      # ── Packages ──────────────────────────────────────────────────────
      # `nix build path:.` produces the xtask runner binary.
      packages = forAllSystems (system:
        let pkgs = pkgsFor system; in {
          default = pkgs.rustPlatform.buildRustPackage {
            pname = "xtask";
            version = "0.1.0";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
            # Build only xtask; exercise stubs are intentionally todo!().
            cargoBuildFlags = [ "--package" "xtask" ];
            # Exercise tests use todo!() stubs and must not run here.
            doCheck = false;
            meta = with pkgs.lib; {
              description = "Runner for the learn-rust gamified Rust campaign";
              license = licenses.mit;
              mainProgram = "xtask";
            };
          };
        }
      );

      # ── Apps ──────────────────────────────────────────────────────────
      # `nix run path:.` invokes the xtask binary directly.
      apps = forAllSystems (system: {
        default = {
          type = "app";
          program = "${self.packages.${system}.default}/bin/xtask";
        };
      });

      # ── Checks ────────────────────────────────────────────────────────
      # `nix flake check path:.` runs workspace compilation and xtask
      # tests.  Exercise tests (which use todo!() stubs) are excluded.
      checks = forAllSystems (system:
        let pkgs = pkgsFor system; in {

          # Verify the entire workspace compiles with todo!() stubs intact.
          workspace-check = pkgs.rustPlatform.buildRustPackage {
            pname = "learn-rust-workspace-check";
            version = "0.1.0";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
            buildPhase = ''
              runHook preBuild
              cargo check --workspace
              runHook postBuild
            '';
            installPhase = "mkdir -p $out";
            doCheck = false;
          };

          # Run xtask's own unit tests (pure logic; no exercise tests).
          xtask-tests = pkgs.rustPlatform.buildRustPackage {
            pname = "learn-rust-xtask-tests";
            version = "0.1.0";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
            buildPhase = ''
              runHook preBuild
              cargo build --package xtask
              runHook postBuild
            '';
            checkPhase = ''
              runHook preCheck
              cargo test --package xtask
              runHook postCheck
            '';
            installPhase = "mkdir -p $out";
            doCheck = true;
          };
        }
      );
    };
}
