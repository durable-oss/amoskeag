{ pkgs, lib, config, inputs, ... }:

{
  # https://devenv.sh/packages/
  packages = with pkgs; [ git libyaml openssl lld llvm_18 ];

  languages.rust.enable = true;


  env.LLVM_SYS_180_PREFIX = "${pkgs.llvm_18.dev}";

  enterShell = ''

  '';

}
