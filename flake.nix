{
    description = "A flake to install tools required for working on keenawa-co/moss project";

    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
        flake-utils.url = "github:numtide/flake-utils";
        rust-overlay = {
            url = "github:oxalica/rust-overlay";
            inputs.nixpkgs.follows = "nixpkgs";
        };
    };

    outputs = { self, nixpkgs, flake-utils, rust-overlay }:
        flake-utils.lib.eachDefaultSystem (system:
            let
                pkgs = import nixpkgs {
                    inherit system;
                    overlays = [ (import rust-overlay) ];
                    config = {
                        allowUnfree = true;    # Enable unfree packages
                    };
                };

                surrealdbVersion = pkgs.surrealdb.overrideAttrs (old: {
                    version = "1.5.4";   # Pin SurrealDB to version 1.5.4
                });

                pnpmVersion = pkgs.pnpm.overrideAttrs (old: {
                    version = "9.8.0";   # Pin pnpm to version 9.8.0
                });

                rustToolchain = pkgs.rust-bin.stable."1.80.1".default;  # Pin Rust toolchain (rustc and cargo) to version 1.80.1

                # Required tools and packages
                packages = with pkgs; [
                    webkitgtk
                    gtk3
                    cairo
                    gdk-pixbuf
                    glib
                    dbus
                    openssl_3
                    librsvg
                    curl
                    wget
                    pkg-config
                    surrealdbVersion
                    cloc           
                    pnpmVersion             
                    rustToolchain    
                    libsoup
                ];
            in
            {
                devShells = {
                    default = pkgs.mkShell {
                        buildInputs = packages;
                    };
                };

            }
        );
}
