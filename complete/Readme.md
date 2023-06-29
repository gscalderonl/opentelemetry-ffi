Prerequisite:

Install CMake  and a C++ compiler 
Install the Rust programming language and Cargo
Install openTelemetry-cpp

- To Run the application, follow this instructions: 
cargo build --release      # Build the Rust application
mkdir build && cd build    # Create a build directory and navigate into it
cmake ..                   # Generate the build files using CMake
cmake --build .            # Build the C++ application

# Run the C++ application
./opentelemetry_cpp_rust_integration
