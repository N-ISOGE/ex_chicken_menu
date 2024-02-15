{ pkgs ? import <nixpkgs> {}
}: pkgs.mkShell {
  nativeBuildInputs = with pkgs.buildPackages; [
    cargo
    rustc
    rustfmt
    rust-analyzer
    clippy
    libiconv
    pkg-config
    openssl
    gnupg
    pinentry-curses
    coreutils-full
    git
  ];
}