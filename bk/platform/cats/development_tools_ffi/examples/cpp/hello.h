// TODO Reviewed ??-??-2026.

// ANCHOR: example
#pragma once
#include <cstdint>
#include <string>

void hello(const std::string& name);
std::int32_t cpp_multiply(std::int32_t left, std::int32_t right);
std::int32_t cpp_add_via_rust(std::int32_t left, std::int32_t right);
extern "C" void cxx_link_anchor();
// ANCHOR_END: example
