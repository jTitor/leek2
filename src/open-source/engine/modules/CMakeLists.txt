# Enable ExternalProject CMake module
include(ExternalProject)

# Set default ExternalProject root directory
set_directory_properties(PROPERTIES EP_PREFIX ${CMAKE_BINARY_DIR}/rust)

# Add rust_example as a CMake target
ExternalProject_Add(
    rust_example
    DOWNLOAD_COMMAND ""
    CONFIGURE_COMMAND ""
    BUILD_COMMAND cargo build COMMAND cargo build --release
    TEST_COMMAND cargo test COMMAND cargo test --release
    BINARY_DIR "${CMAKE_SOURCE_DIR}/engine/modules"
    INSTALL_COMMAND ""
    LOG_BUILD ON)