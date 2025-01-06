// src/hello.cc
#include "hello.h"
#include <iostream>

void hello(const std::string& name) {
    std::cout << "Hello, " << name << " from C++!" << std::endl;
}
