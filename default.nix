{
  pkgs ? import <nixpkgs> {}
}:
pkgs.mkShell {
  name="dev-environment";
  buildInputs = with pkgs; [
    cargo rustup
#    diesel-cli
  ];
}