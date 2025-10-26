cmake_minimum_required(VERSION 3.20)
project(AchronymeCore VERSION 0.1.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_CXX_EXTENSIONS OFF)

# Compiler options
if(MSVC)
    add_compile_options(/W4)
else()
    add_compile_options(-Wall -Wextra -Wpedantic)
endif()

# Emscripten detection
if(EMSCRIPTEN)
    message(STATUS "Building for WebAssembly with Emscripten")
    set(CMAKE_EXECUTABLE_SUFFIX ".js")
endif()

# Add WASM subdirectory
add_subdirectory(wasm)

# Enable testing
enable_testing()
