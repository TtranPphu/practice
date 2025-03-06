cmake_minimum_required(VERSION 3.14.0 FATAL_ERROR)
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

project(practice VERSION 0.1.0 LANGUAGES CXX)

include(FetchContent)

add_subdirectory(log_util)

FetchContent_Declare(
    googletest
    URL
        https://github.com/google/googletest/archive/b514bdc898e2951020cbdca1304b75f5950d1f59.zip
)
FetchContent_MakeAvailable(googletest)

add_executable(practice main.cpp ${SOURCE_FILES})
target_include_directories(
    practice
    PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/3rd_party/spdlog/include
)
target_link_libraries(practice PUBLIC log)

# GoogleTest
enable_testing()

add_executable(leetcode_test tests/leetcode.cpp)
target_include_directories(
    leetcode_test
    PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/leetcode
)
target_link_libraries(leetcode_test GTest::gtest_main)

add_executable(interview_test tests/interview.cpp)
target_include_directories(
    interview_test
    PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/interview
)
target_link_libraries(interview_test GTest::gtest_main)

include(GoogleTest)
gtest_discover_tests(leetcode_test)
gtest_discover_tests(interview_test)
