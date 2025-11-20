require 'mkmf'
require 'fileutils'

# Build the Rust static library first
def build_rust_library
  puts "Building Amoskeag Rust library..."

  ext_dir = File.dirname(__FILE__)
  cargo_dir = ext_dir

  # Build in release mode for performance
  cargo_cmd = "cargo build --release --manifest-path=#{cargo_dir}/Cargo.toml"

  unless system(cargo_cmd)
    abort "Failed to build Rust library. Make sure Rust and Cargo are installed."
  end

  # Return the path to the built library
  target_dir = File.join(File.dirname(cargo_dir), "target", "release")
  lib_path = File.join(target_dir, "libamoskeag_ffi.a")

  unless File.exist?(lib_path)
    abort "Rust library not found at #{lib_path}"
  end

  puts "Rust library built successfully at #{lib_path}"
  lib_path
end

# Build the Rust library
rust_lib_path = build_rust_library

# Get the directory containing the static library
rust_lib_dir = File.dirname(rust_lib_path)

# Add the Rust library directory to the library search path
$LDFLAGS << " -L#{rust_lib_dir}"

# Link against the Rust static library
$LDFLAGS << " -lamoskeag_ffi"

# Add required system libraries for Rust
# These are needed for the Rust standard library
$LDFLAGS << " -lpthread -ldl -lm"

# On some systems, we may need to link against additional libraries
if RbConfig::CONFIG['host_os'] =~ /darwin|mac os/
  $LDFLAGS << " -framework Security"
elsif RbConfig::CONFIG['host_os'] =~ /linux/
  $LDFLAGS << " -lrt"
end

# Check for Ruby header
have_header('ruby.h') or abort "ruby.h not found"

# Create the Makefile
create_makefile('amoskeag_native')

# Add custom clean task to also clean Rust artifacts
makefile_content = File.read('Makefile')
makefile_content << "\n\nclean-rust:\n"
makefile_content << "\tcd #{File.dirname(__FILE__)} && cargo clean\n"
makefile_content << "\nclean: clean-rust\n"

File.write('Makefile', makefile_content)

puts "extconf.rb completed successfully"
