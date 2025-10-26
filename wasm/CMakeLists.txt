cmake_minimum_required(VERSION 3.20)
project(AchronymeWASM VERSION 0.1.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Source files
set(CORE_SOURCES
    src/core/value.cpp
)

set(PARSER_SOURCES
    src/parser/lexer.cpp
    src/parser/parser.cpp
    src/parser/evaluator.cpp
)

set(BINDINGS_SOURCES
    src/bindings/main.cpp
)

set(ALL_SOURCES
    ${CORE_SOURCES}
    ${PARSER_SOURCES}
    ${BINDINGS_SOURCES}
)

# Emscripten build
if(EMSCRIPTEN)
    message(STATUS "Building for WebAssembly with Emscripten")

    add_executable(achronyme-core ${ALL_SOURCES})

    # Emscripten compiler flags
    target_compile_options(achronyme-core PRIVATE
        -fexceptions
        -O3
    )

    # Emscripten linker flags
    set_target_properties(achronyme-core PROPERTIES
        LINK_FLAGS "\
            -s WASM=1 \
            -s ALLOW_MEMORY_GROWTH=1 \
            -s MODULARIZE=1 \
            -s EXPORT_ES6=1 \
            -s EXPORT_NAME='AchronymeCore' \
            -s USE_ES6_IMPORT_META=0 \
            -s ENVIRONMENT='web,worker' \
            --bind \
            -fexceptions \
            -O3"
    )

    # Optional SIMD support
    if(ENABLE_SIMD)
        message(STATUS "SIMD optimizations enabled")
        target_compile_options(achronyme-core PRIVATE -msimd128)
        set_target_properties(achronyme-core PROPERTIES
            LINK_FLAGS "${LINK_FLAGS} -msimd128"
        )
    endif()

    # Set output names
    set_target_properties(achronyme-core PROPERTIES
        OUTPUT_NAME "achronyme-core"
        SUFFIX ".js"
    )

    # Install to dist/
    install(TARGETS achronyme-core
        RUNTIME DESTINATION ${CMAKE_SOURCE_DIR}/../dist
    )

    # Also copy the .wasm file
    install(FILES ${CMAKE_CURRENT_BINARY_DIR}/achronyme-core.wasm
        DESTINATION ${CMAKE_SOURCE_DIR}/../dist
    )

# Native build (for testing)
else()
    message(STATUS "Building for native platform")

    add_library(achronyme-core-lib STATIC
        ${CORE_SOURCES}
        ${PARSER_SOURCES}
    )

    target_include_directories(achronyme-core-lib PUBLIC
        ${CMAKE_CURRENT_SOURCE_DIR}/src
    )

    # Add tests subdirectory if building natively
    if(BUILD_TESTING)
        enable_testing()
        add_subdirectory(tests)
    endif()
endif()
