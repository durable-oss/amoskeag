require 'mkmf'
require 'fileutils'

# Build the Rust static library first
def build_rust_library
  puts "Building Amoskeag Rust library..."

  # Defensive: Validate environment
  ext_dir = File.dirname(__FILE__)
  unless File.directory?(ext_dir)
    abort "Extension directory does not exist: #{ext_dir}"
  end

  cargo_dir = ext_dir
  cargo_toml_path = File.join(cargo_dir, 'Cargo.toml')

  # Defensive: Check if Cargo.toml exists
  unless File.exist?(cargo_toml_path)
    abort "Cargo.toml not found at #{cargo_toml_path}. Cannot build Rust library."
  end

  # Defensive: Check if cargo is available
  unless system("which cargo > /dev/null 2>&1")
    abort "Cargo (Rust build tool) not found in PATH. Please install Rust from https://rustup.rs/"
  end

  # Defensive: Check cargo version
  cargo_version = `cargo --version 2>&1`.strip
  puts "Using #{cargo_version}"

  # Build in release mode for performance
  cargo_cmd = "cargo build --release --manifest-path=#{cargo_toml_path}"

  puts "Running: #{cargo_cmd}"
  unless system(cargo_cmd)
    abort "Failed to build Rust library. Check the error messages above."
  end

  # Return the path to the built library
  # Defensive: Handle multiple possible target locations
  possible_target_dirs = [
    File.join(File.dirname(cargo_dir), "target", "release"),
    File.join(cargo_dir, "target", "release"),
    File.join(Dir.pwd, "target", "release"),
    # Also check in the workspace root (3 levels up from cargo_dir)
    File.join(File.expand_path("../..", File.dirname(cargo_dir)), "target", "release")
  ]

  lib_name = "libamoskeag_ffi.a"
  lib_path = nil

  puts "Searching for #{lib_name} in:"
  possible_target_dirs.each do |target_dir|
    candidate = File.join(target_dir, lib_name)
    puts "  Checking: #{candidate}"
    if File.exist?(candidate)
      lib_path = candidate
      puts "  Found!"
      break
    end
  end

  unless lib_path && File.exist?(lib_path)
    abort "Rust library not found. Searched in:\n" +
          possible_target_dirs.map { |d| "  - #{File.join(d, lib_name)}" }.join("\n")
  end

  # Defensive: Verify the library is a valid file
  unless File.file?(lib_path)
    abort "Rust library exists but is not a regular file: #{lib_path}"
  end

  # Defensive: Verify library has reasonable size (not empty or corrupted)
  file_size = File.size(lib_path)
  if file_size < 1024  # Less than 1KB is suspicious
    abort "Rust library seems invalid (too small: #{file_size} bytes): #{lib_path}"
  end

  puts "Rust library built successfully at #{lib_path} (#{file_size} bytes)"
  lib_path
end

# Build the Rust library
rust_lib_path = build_rust_library

# Get the directory containing the static library
rust_lib_dir = File.dirname(rust_lib_path)

# Defensive: Verify rust_lib_dir is valid
unless File.directory?(rust_lib_dir)
  abort "Rust library directory does not exist: #{rust_lib_dir}"
end

# Defensive: Escape paths to handle spaces and special characters
rust_lib_dir_escaped = rust_lib_dir.shellescape

# Add the Rust library directory to the library search path
$LDFLAGS << " -L#{rust_lib_dir}"

# Link against the Rust static library
$LDFLAGS << " -lamoskeag_ffi"

# Add required system libraries for Rust
# These are needed for the Rust standard library
$LDFLAGS << " -lpthread -ldl -lm"

# On some systems, we may need to link against additional libraries
host_os = RbConfig::CONFIG['host_os']
puts "Building for OS: #{host_os}"

if host_os =~ /darwin|mac os/i
  $LDFLAGS << " -framework Security"
  puts "Added macOS-specific linker flags"
elsif host_os =~ /linux/i
  $LDFLAGS << " -lrt"
  puts "Added Linux-specific linker flags"
elsif host_os =~ /mswin|mingw|cygwin/i
  $LDFLAGS << " -lws2_32 -luserenv"
  puts "Added Windows-specific linker flags"
else
  puts "Warning: Unknown OS '#{host_os}', using default linker flags"
end

# Check for Ruby header
puts "Checking for ruby.h..."
unless have_header('ruby.h')
  abort "ruby.h not found. Make sure Ruby development headers are installed."
end

puts "All checks passed. Creating Makefile..."

# Create the Makefile
create_makefile('amoskeag_native')

# Add custom clean task to also clean Rust artifacts
makefile_content = File.read('Makefile')
makefile_content << "\n\nclean-rust:\n"
makefile_content << "\tcd #{File.dirname(__FILE__)} && cargo clean\n"
makefile_content << "\nclean: clean-rust\n"

File.write('Makefile', makefile_content)

puts "extconf.rb completed successfully"
