#include "hello.h"
#include <iostream>

/// Prints a greeting to the console using C++ standard I/O.
void hello(const std::string& name) {
    std::cout << "Hello, " << name << " from C++!" << std::endl;
}
