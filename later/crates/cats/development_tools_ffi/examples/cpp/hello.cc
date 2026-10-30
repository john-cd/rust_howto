// TODO Reviewed ??-??-2026.

// ANCHOR: example
// With cxx-build, by default your include paths always start with the crate name.
// This applies to both #include within your C++ code,
// and include! in the extern "C++" section of your Rust cxx::bridge.
#include "development_tools_ffi/examples/cpp/hello.h"
#include "development_tools_ffi/examples/cpp/cxx.rs.h"

#include <iostream>
#include <string>
#include <cstdint>

void hello(const std::string& name) {
    std::cout << "Hello, " << name << " from C++!" << std::endl;
}

std::int32_t cpp_multiply(std::int32_t left, std::int32_t right) {
    return left * right;
}

std::int32_t cpp_add_via_rust(std::int32_t left, std::int32_t right) {
    return rust_add(left, right);
}

extern "C" void cxx_link_anchor() {}
// ANCHOR_END: example
