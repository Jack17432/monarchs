{
    description = "monarchs flake";

    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
        flake-utils.url = "github:numtide/flake-utils";

        rust-overlay.url = "github:oxalica/rust-overlay";
    };

    outputs = {
        self,
        nixpkgs,
        flake-utils,
        rust-overlay,
        ...
    }:
        flake-utils.lib.eachDefaultSystem (system: let
            overlays = [ (import rust-overlay) ];
            pkgs = import nixpkgs { inherit system overlays; };
        in {
            devShells.default = pkgs.mkShell {
                packages = with pkgs; [
                    just
                ];

                nativeBuildInputs = with pkgs; [
                    (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
                    pkg-config
                ];

                buildInputs = with pkgs; [

                    udev alsa-lib vulkan-loader
                    xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr
                    libxkbcommon wayland
                ];

                LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath (with pkgs; [
                    udev alsa-lib vulkan-loader
                    xorg.libX11 xorg.libXcursor xorg.libXi xorg.libXrandr
                    libxkbcommon wayland
                ]);
            };
        });
}
