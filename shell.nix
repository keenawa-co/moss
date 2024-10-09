{ pkgs ? import <nixpkgs> { config = { allowUnfree = true; }; } }:

pkgs.mkShell {
  buildInputs = [
    pkgs.rustc               
    pkgs.cargo               
    pkgs.pnpm                
    pkgs.surrealdb           
    pkgs.cargo-tauri.hook    
  ];

  # Optional shell hook: run commands when the shell starts
  shellHook = ''
    echo "Development environment ready with SurrealDB, Tauri, PNPM, and Rust"
  '';
}