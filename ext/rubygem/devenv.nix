{ pkgs, lib, config, ... }:

{
  # Enable languages
  languages.ruby = {
    enable = true;
    version = "3.2.2";
  };

  languages.rust = {
    enable = true;
    channel = "stable";
  };

  # System packages needed for Ruby gem development
  packages = with pkgs; [
    # Ruby development dependencies
    ruby
    rubyPackages.rake
    rubyPackages.bundler

    # Build tools
    gcc
    gnumake
    pkg-config

    # For the C extension
    ruby.devEnv

    # Rust tooling
    cargo
    rustc
    rustfmt
    clippy

    # Testing and development
    git
  ];

  # Environment variables
  env = {
    # Ensure Ruby can find the headers
    RUBY_INCLUDE_DIR = "${pkgs.ruby}/include/ruby-3.2.0";

    # Help the build find libraries
    LIBRARY_PATH = lib.makeLibraryPath [
      pkgs.ruby
    ];

    LD_LIBRARY_PATH = lib.makeLibraryPath [
      pkgs.ruby
      pkgs.stdenv.cc.cc.lib
    ];
  };

  # Shell initialization
  enterShell = ''
    echo "Ruby gem development environment"
    echo "Ruby version: $(ruby --version)"
    echo "Cargo version: $(cargo --version)"
    echo "Rake version: $(rake --version)"
    echo ""
    echo "Available commands:"
    echo "  rake compile  - Compile the extension"
    echo "  rake test     - Run all tests"
    echo "  rake clean    - Clean build artifacts"
    echo "  gem build amoskeag-rb.gemspec - Build the gem"
  '';

  # Pre-commit hooks for code quality
  pre-commit.hooks = {
    rustfmt.enable = true;
    clippy.enable = true;
  };
}
