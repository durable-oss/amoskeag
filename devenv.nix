{ pkgs, lib, config, inputs, ... }:

{
  # https://devenv.sh/packages/
  packages = with pkgs; [ git libyaml openssl lld llvm_18 zig ];

  languages.rust = {
    enable = true;
    channel = "stable";
    components = [ "rustc" "cargo" "rust-std" ];
    targets = [ "x86_64-unknown-linux-musl" "aarch64-unknown-linux-musl" ];
  };


  env.LLVM_SYS_180_PREFIX = "${pkgs.llvm_18.dev}";

  enterShell = ''

  '';

}
