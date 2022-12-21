cmake_minimum_required(VERSION 3.1...3.15)
project(day-20)
set(BINARY day-20)

# set standards
set(CMAKE_CXX_STANDARD 20)
set(CMAKE_C_STANDARD 99) 

add_executable(${BINARY}
   main.cpp
)

target_compile_options(${BINARY} PRIVATE -O0 -g3)