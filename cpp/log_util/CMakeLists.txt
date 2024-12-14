add_library(log STATIC log.cpp)
target_include_directories(log PUBLIC ${CMAKE_CURRENT_SOURCE_DIR}/include)
