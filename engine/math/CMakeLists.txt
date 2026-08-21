# Pention Engine - math module
# Requirement: PN-WLD-001, PN-WLD-002
# Decision:    ADR-0003, ADR-0004 (lowest module; depends on nothing)

pn_add_module(math INTERFACE)

pn_add_test(math_vector     SOURCES tests/vector_test.cpp     DEPS pn::math)
pn_add_test(math_matrix     SOURCES tests/matrix_test.cpp     DEPS pn::math)
pn_add_test(math_convention SOURCES tests/convention_test.cpp DEPS pn::math)
pn_add_test(math_precision  SOURCES tests/precision_test.cpp  DEPS pn::math)
