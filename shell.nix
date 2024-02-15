{ pkgs ? import <nixpkgs> {}
}: pkgs.mkShell {
  nativeBuildInputs = with pkgs.buildPackages; [
    cargo
    rustc
    rustfmt
    rust-analyzer
    clippy
    gnupg
    pinentry-curses
    coreutils-full
    git
  ];
}