// Pention Engine - platform/tests/thread_test.cpp
// Requirement: PN-PLT-012
// Decision:    ADR-0004

#include "pn/platform/thread.hpp"
#include "pn/testing/test.hpp"

#include <atomic>
#include <thread>
#include <vector>

PN_TEST(thread, hardware_thread_count_is_never_zero) {
    // std::thread::hardware_concurrency may return 0 when it cannot tell, and
    // a thread pool sized from that has no workers at all.
    PN_CHECK_GE(pn::platform::hardware_thread_count(), 1u);
}

PN_TEST(thread, naming_the_current_thread_does_not_throw_or_crash) {
    // Best-effort by contract. What must hold is that a failure to set a
    // diagnostic label never becomes a failure of the thing being labelled.
    pn::platform::set_current_thread_name("pn-test");
    pn::platform::set_current_thread_name("");
    pn::platform::set_current_thread_name(
        "a-name-far-longer-than-any-platform-permits-for-a-thread-label");
    PN_CHECK(true);
}

PN_TEST(thread, naming_works_from_a_spawned_thread) {
    std::atomic<bool> completed{false};
    std::thread worker{[&completed] {
        pn::platform::set_current_thread_name("pn-worker");
        completed.store(true, std::memory_order_release);
    }};
    worker.join();
    PN_CHECK(completed.load(std::memory_order_acquire));
}

PN_TEST(thread, yield_is_safe_to_call_repeatedly) {
    for (int i = 0; i < 100; ++i) {
        pn::platform::yield_current_thread();
    }
    PN_CHECK(true);
}
