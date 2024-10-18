{
    description = "A flake to install tools required for working on keenawa-co/moss project";

    inputs = {
        nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
        # Specific nixpkgs for surrealdb
        surrealNixpkgs.url = "github:NixOS/nixpkgs/73b9e297fae10023e18162f0381330a1bd728037";
        
        flake-utils.url = "github:numtide/flake-utils";
        rust-overlay = {
            url = "github:oxalica/rust-overlay";
            inputs.nixpkgs.follows = "nixpkgs";
        };
    };

    outputs = { self, nixpkgs, surrealNixpkgs, flake-utils, rust-overlay }:
        flake-utils.lib.eachDefaultSystem (system:
            let
                pkgs = import nixpkgs {
                    inherit system;
                    overlays = [ (import rust-overlay) ];
                    config = {
                        allowUnfree = true;
                    };
                };

                # Import the specific nixpkgs for surrealdb
                surrealPkgs = import surrealNixpkgs {
                    inherit system;
                    overlays = [ (import rust-overlay) ];
                    config = {
                        allowUnfree = true;    # Enable unfree packages if needed
                    };
                };

                surrealdbVersion = surrealPkgs.surrealdb;

                pnpmVersion = pkgs.pnpm.overrideAttrs (old: {
                    version = "9.8.0";
                    src = pkgs.fetchurl { # had to add this because of hash missmatches
                        url = "https://registry.npmjs.org/pnpm/-/pnpm-9.8.0.tgz";
                        sha256 = "Vqnna1F5bKf3O4XkTPg3EoYgkfTUmMDOTVt+zca6GPc=";
                    }; 
                });

                rustToolchain = pkgs.rust-bin.stable."1.80.1".default;

                tauriDeps = with pkgs; [
                    at-spi2-atk
                    atkmm
                    cairo
                    gdk-pixbuf
                    glib
                    gobject-introspection
                    gobject-introspection.dev
                    gtk3
                    harfbuzz
                    librsvg
                    libsoup_3
                    pango
                    webkitgtk_4_1
                    webkitgtk_4_1.dev
                ];

                # Required tools and packages
                packages = with pkgs; [
                    dbus
                    openssl_3
                    curl
                    wget
                    pkg-config
                    surrealdbVersion
                    cloc           
                    pnpmVersion             
                    rustToolchain    
                ] ++ tauriDeps;

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
