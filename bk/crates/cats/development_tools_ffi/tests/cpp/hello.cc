// ANCHOR: example
// With cxx-build, by default your include paths always start with the crate name.
// This applies to both #include within your C++ code,
// and include! in the extern "C++" section of your Rust cxx::bridge.
#include "d/tests/development_tools_ffi/hello.h"

#include <iostream>
#include <string>

void hello(const std::string& name) {
    std::cout << "Hello, " << name << " from C++!" << std::endl;
}
// ANCHOR_END: example
