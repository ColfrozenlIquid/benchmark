cmake_minimum_required(VERSION 3.10)
project(BenchmarkTest)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED True)

find_package(benchmark REQUIRED)

add_executable(benchmark_test main.cpp)
target_link_libraries(benchmark_test PRIVATE benchmark::benchmark)
