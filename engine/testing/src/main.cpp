// Pention Engine -  testing/main.cpp
// Requirement: PN-PLT-026 -  in-project unit test framework
// Decision:    ADR-0001
//
// Default entry point linked into every test executable.

#include "pn/testing/test.hpp"

int main(int argc, char** argv) {
    return pn::testing::run_all(argc, argv);
}
