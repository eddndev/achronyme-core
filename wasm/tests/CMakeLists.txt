# Tests for Achronyme Core

# Find Google Test
find_package(GTest REQUIRED)
include_directories(${GTEST_INCLUDE_DIRS})

# Test executable
add_executable(test_basic test_basic.cpp)

# Link against the library and Google Test
target_link_libraries(test_basic
    achronyme-core-lib
    ${GTEST_LIBRARIES}
    pthread
)

# Add test to CTest
add_test(NAME BasicTests COMMAND test_basic)
