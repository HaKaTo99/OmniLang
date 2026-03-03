# CMakeLists.txt for HarmonyOS Native C++ Bridge (OmniLang)
cmake_minimum_required(VERSION 3.4.1)
project(omnilang_harmonyos)

set(NATIVERENDER_ROOT_PATH ${CMAKE_CURRENT_SOURCE_DIR})

# Assume that libomnilang_core.so or .a is compiled and placed in the appropriate jniLibs or libs directory architecture
# For simplicity in this CMake, we assume dynamic linking to the Rust-generated .so
# find_library(OMNILANG_CORE omnilang_core HINTS ${CMAKE_CURRENT_SOURCE_DIR}/../../../target/aarch64-linux-ohos/release)

include_directories(${NATIVERENDER_ROOT_PATH}
                    ${NATIVERENDER_ROOT_PATH}/include)

add_library(omnilang_napi SHARED napi_init.cpp)

# Link NAPI (Node API for HarmonyOS) libraries
# In actual DevEco Studio, "libace_napi.z.so" is the standard library for ArkTS NAPI
target_link_libraries(omnilang_napi PUBLIC libace_napi.z.so)

# If linking Rust statically/dynamically, add here:
# target_link_libraries(omnilang_napi PUBLIC ${OMNILANG_CORE})
