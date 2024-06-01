{
  description = "Pifijs the Discord bot!";

  # Dependencies for building everything in the flake
  inputs = {
    # All packages in the nix repo
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    # Utilities for building nix flakes for multiple architectures
    flake-utils.url = "github:numtide/flake-utils";
    # Helper for creating custom Rust toolchains
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    # Helper for building Rust packages
    naersk.url = "github:nix-community/naersk/master";
    # Helper for caching (avoid re-building by ignoring gitignored files)
    gitignore = {
      url = "github:hercules-ci/gitignore.nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, fenix, naersk, flake-utils, gitignore }:
    # Build outputs for each default system
    # by default that is ["x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin"]
    flake-utils.lib.eachDefaultSystem (system:
      let 
        # Build parameters
        version = "0.1.0";
        cargoBuildType = "debug"; # cargo build --release or --debug

        # System packages
        pkgs = nixpkgs.legacyPackages.${system};

        # Reading source code with gitignore 
        gitignoreSource = gitignore.lib.gitignoreSource;

        # Rust build tools
        cargo = pkgs.cargo;
        rustc = pkgs.cargo;
        fenixSystem = fenix.packages.${system};
        toolchain = fenixSystem.latest.toolchain;
        rustPlatform = pkgs.makeRustPlatform {
          inherit toolchain cargo rustc;
        };
        buildRustPackage = rustPlatform.buildRustPackage;

        defaultBuildDependencies = [
          pkgs.pkg-config
          pkgs.stdenv.cc.cc.lib
          toolchain 
        ];

        # Source: https://github.com/bevyengine/bevy/blob/main/docs/linux_dependencies.md#nix
        plotterLinkedDependencies = [
          pkgs.udev 
          pkgs.alsa-lib 
          pkgs.vulkan-loader
          # X11 support
          pkgs.xorg.libX11 
          pkgs.xorg.libXcursor 
          pkgs.xorg.libXi 
          pkgs.xorg.libXrandr
          # Wayland support
          pkgs.libxkbcommon 
          pkgs.wayland 
        ];
        
        # Plotter build (pure, but broken right now)
        #   Currently broken, because Nix can't handle a git dependency which uses workspaces
        #   Supposedly fixed, but doesn't work for me anyway
        #   Source: https://github.com/rust-lang/cargo/issues/11192
        #   Will be automatically fixed when bevy officially releases 0.14.x
        # cargoBuild = buildRustPackage {
        #   # inherit importCargoLock;
        #   pname = "plotter";
        #   src = gitignoreSource ./.;
        #   extraPrefix = "/bot";
        #   buildType = cargoBuildType;
        #   cargoLock = {
        #     lockFile = ./Cargo.lock;
        #     outputHashes = {
        #       "bevy-0.14.0-dev" = "sha256-uLToacQEmRyGUbDi/Z8gjTS5iRECJmDD6iJ5G+zwvgA=";
        #     };
        #   };
        #   version = version;
        #   # cargoSha256 = "sha256-4AUlVsEhOqzm8oMXNbP2Qs4ZktVuZTw1+W7p0YRCYv8=";
        #   nativeBuildInputs = defaultBuildDependencies;
        #   buildInputs = plotterLinkedDependencies;
        # };

        # Plotter build (impure, requires nix build --option sandbox false)
        cargoBuild = pkgs.stdenv.mkDerivation {
          name = "plotter";
          src = gitignoreSource ./.;
          version = version;
          nativeBuildInputs = defaultBuildDependencies ++ [
            pkgs.autoPatchelfHook
          ];
          buildInputs = plotterLinkedDependencies;
          installPhase = ''
            runHook preInstall
            
            CARGO_HOME=$PWD/cargo cargo build ${if cargoBuildType == "release" then "--release" else ""}

            mkdir $out
            cp -rv ./target/${cargoBuildType}/{pifijs_bot,pifijs_plotter} $out

            runHook postInstall
          '';

          postFixup = ''
            patchelf $out/pifijs_plotter \
              --add-rpath ${lib.makeLibraryPath plotterLinkedDependencies}
          '';
        };
      in {
        packages.pifijs = cargoBuild;
        devShells.default =
          pkgs.mkShell { 
            # Needed by Bevy
            LD_LIBRARY_PATH = "$LD_LIBRARY_PATH:${builtins.toString (pkgs.lib.makeLibraryPath plotterLinkedDependencies)}";
            buildInputs = 
              defaultBuildDependencies ++ 
              plotterLinkedDependencies ++ [
                pkgs.bash
                pkgs.git
                pkgs.vim
              ];
          };
        
        # Default package (result of nix build)
        defaultPackage = cargoBuild;
      });
}
