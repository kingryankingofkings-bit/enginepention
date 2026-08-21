# Pention Engine - jobs module
# Requirement: PN-PLT-013, PN-PLT-014
# Decision:    ADR-0004, ADR-0008

pn_add_module(jobs
    SOURCES src/job_system.cpp
    PUBLIC_DEPS pn::platform)

pn_add_test(jobs_deque      SOURCES tests/deque_test.cpp      DEPS pn::jobs)
pn_add_test(jobs_job_system SOURCES tests/job_system_test.cpp DEPS pn::jobs)
