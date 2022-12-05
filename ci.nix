{ config, channels, pkgs, lib, ... }: with pkgs; with lib; let
  inherit (import ./. { inherit pkgs; }) checks;
in {
  name = "ddc-i2c-rs";
  ci.gh-actions.enable = true;
  cache.cachix = {
    ci.signingKey = "";
    arc.enable = true;
  };
  channels = {
    nixpkgs = "22.11";
  };
  tasks = {
    build.inputs = singleton checks.test;
  };
  jobs = {
    nixos = {
      tasks.rustfmt.inputs = singleton checks.rustfmt;
      tasks.version.inputs = singleton checks.version;
      # TODO: cross-windows build
    };
    macos.system = "x86_64-darwin";
  };
}
