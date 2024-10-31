{
  inputs = {
    unstablePkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    surrealNixpkgs.url = "github:NixOS/nixpkgs/73b9e297fae10023e18162f0381330a1bd728037";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "unstablePkgs";
      };
    };
  };
  outputs = { self, unstablePkgs, surrealNixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          overlays = [ (import rust-overlay) ];
          unstable = import unstablePkgs {
            inherit system overlays;
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

            pnpmVersion = unstable.pnpm.overrideAttrs (old: {
                version = "9.8.0";
                src = unstable.fetchurl { # had to add this because of hash missmatches
                    url = "https://registry.npmjs.org/pnpm/-/pnpm-9.8.0.tgz";
                    sha256 = "Vqnna1F5bKf3O4XkTPg3EoYgkfTUmMDOTVt+zca6GPc=";
                }; 
            });

            rustToolchain = unstable.rust-bin.stable."1.80.1".default;

          common = with unstable; [
            gtk3
            dbus
            glib
            glib-networking
            openssl_3
            librsvg
            gettext
            libiconv
            libsoup
            libsoup_3
            webkitgtk
            webkitgtk_4_1
            nodejs_20
            corepack_20
            locale
          ];

          # runtime Deps
          libraries = with unstable;[
            cairo
            pango
            harfbuzz
            gdk-pixbuf
            surrealdbVersion
            # libcanberra
            # libcanberra-gtk2
            libcanberra-gtk3
          ] ++ common;

          # compile-time deps
          packages = with unstable; [
            pnpmVersion
            curl
            wget
            pkg-config
            rustToolchain
          ] ++ common;
        in
        {
          devShells.default = unstable.mkShell {
            nativeBuildInputs = packages;
            buildInputs = libraries;
            shellHook = ''
              export LD_LIBRARY_PATH=${unstable.lib.makeLibraryPath libraries}:$LD_LIBRARY_PATH
              export XDG_DATA_DIRS=${unstable.gsettings-desktop-schemas}/share/gsettings-schemas/${unstable.gsettings-desktop-schemas.name}:${unstable.gtk3}/share/gsettings-schemas/${unstable.gtk3.name}:$XDG_DATA_DIRS
              export GIO_MODULE_DIR="${unstable.glib}/lib/gio/modules/"
              export GIO_EXTRA_MODULES=$GIO_MODULE_DIR
              export GTK_PATH="${unstable.libcanberra-gtk3}/lib/gtk-3.0"
              export LC_ALL="en.utf-8"
            '';
          };
        }
      );
}