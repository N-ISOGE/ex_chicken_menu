{ pkgs, ... }:
let
  rpkgs = pkgs.extend (import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz"));
in
{

  # Which nixpkgs channel to use.
  channel = "unstable"; # or "stable-23.05"

  # Use https://search.nixos.org/packages to find packages
  packages = with rpkgs; [
    (rust-bin.fromRustupToolchainFile ../rust-toolchain.toml)
    stdenv.cc
    pkgs.libiconv
    pkgs.pkg-config
    pkgs.openssl
    pkgs.gnupg
    pkgs.pinentry-curses
    pkgs.coreutils-full
  ];

  # Sets environment variables in the workspace
  env = { };

  # Search for the extensions you want on https://open-vsx.org/ and use "publisher.id"
  idx.extensions = [
    # rust
    "rust-lang.rust-analyzer"
    "serayuzgur.crates"
    "vadimcn.vscode-lldb"
    "tamasfe.even-better-toml"
    # nix
    "arrterian.nix-env-selector"
    # git
    "sugatoray.vscode-git-extension-pack"
    # comment, markdown
    "DavidAnson.vscode-markdownlint"
    "edwinhuish.better-comments-next"
  ];

  # Enable previews and customize configuration
  idx.previews = {
    enable = true;
  };
}
