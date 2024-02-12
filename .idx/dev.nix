{ pkgs, ... }: {

  # Which nixpkgs channel to use.
  channel = "unstable"; # or "stable-23.05"

  # Use https://search.nixos.org/packages to find packages
  packages = [
    pkgs.rustup
    pkgs.cargo
    pkgs.rustc
    pkgs.rustfmt
    pkgs.rust-analyzer
    pkgs.clippy
  ];

  # Sets environment variables in the workspace
  env = {};

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
  ];

  # Enable previews and customize configuration
  idx.previews = {
    enable = true;
   };
}