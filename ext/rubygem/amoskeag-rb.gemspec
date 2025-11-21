require_relative 'lib/amoskeag-rb/version'

Gem::Specification.new do |spec|
  spec.name          = "amoskeag-rb"
  spec.version       = Amoskeag::VERSION
  spec.authors       = ["Durable Programming"]
  spec.email         = ["contact@example.com"]

  spec.summary       = "Ruby bindings for Amoskeag - a secure, functional DSL for business rules"
  spec.description   = <<~DESC
    Amoskeag is a purely functional, statically-validated Domain-Specific Language (DSL)
    designed for high-security, sandboxed evaluation. It's perfect for:

    - Business Rules Engines (insurance underwriting, loan approval)
    - Template Engines (secure alternative to ERB with more power than Liquid)
    - Spreadsheet Formula Engines (Excel-like calculations)

    This gem provides native Ruby bindings to the Amoskeag library, compiled from Rust
    for maximum performance and security.
  DESC

  spec.homepage      = "https://github.com/durable-oss/amoskeag"
  spec.license       = "MIT"
  spec.required_ruby_version = ">= 2.7.0"

  spec.metadata["homepage_uri"] = spec.homepage
  spec.metadata["source_code_uri"] = spec.homepage
  spec.metadata["changelog_uri"] = "#{spec.homepage}/blob/main/CHANGELOG.md"

  # Specify which files should be added to the gem when it is released.
  spec.files = Dir[
    "lib/**/*.rb",
    "ext/**/*.{c,h,rb,rs}",
    "ext/**/Cargo.toml",
    "README.md",
    "LICENSE",
    "CHANGELOG.md"
  ].select { |f| File.file?(f) }

  spec.require_paths = ["lib"]
  spec.extensions    = ["ext/amoskeag/extconf.rb"]

  # Runtime dependencies
  spec.add_runtime_dependency "json", "~> 2.0"

  # Development dependencies
  spec.add_development_dependency "rake", "~> 13.0"
  spec.add_development_dependency "rake-compiler", "~> 1.2"
  spec.add_development_dependency "minitest", "~> 5.0"
  spec.add_development_dependency "yard", "~> 0.9"

  # Post-install message
  spec.post_install_message = <<~MSG

    Thank you for installing amoskeag-rb!

    Amoskeag is a secure, purely functional DSL for business rules, templates,
    and spreadsheet formulas. It's designed to be immune to code injection
    attacks while providing powerful expression evaluation capabilities.

    Get started:
      require 'amoskeag-rb'
      result = Amoskeag.eval("2 + 2", {})  # => 4.0

    Documentation: https://github.com/durable-oss/amoskeag

  MSG
end
