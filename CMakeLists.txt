cmake_minimum_required(VERSION 3.20)

set(CMAKE_CXX_STANDARD 17)


project(read_model)

find_package(OpenCV REQUIRED)
find_package(Torch REQUIRED PATHS "/usr/local/opt/libtorch")

add_executable(digit digit.cpp)

target_link_libraries(digit ${TORCH_LIBRARIES})
target_link_libraries(digit ${OpenCV_LIBS})