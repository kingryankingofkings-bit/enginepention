// Pention Engine - jobs/tests/c_api_test.cpp
// Requirement: PN-PLT-013, PN-OBJ-017
// Decision:    ADR-0009
//
// The C ABI is tested from the C++ side too, not only from Rust. If the header
// contract is wrong, this catches it without a second toolchain in the loop.

#include "pn/jobs/jobs_c_api.h"
#include "pn/testing/test.hpp"

#include <atomic>
#include <cstdint>
#include <numeric>
#include <vector>

namespace {

struct SumContext {
    const std::uint64_t* values;
    std::atomic<std::uint64_t> total;
    std::atomic<std::uint64_t> chunk_count;
};

void sum_chunk(void* user_data, std::uint64_t begin, std::uint64_t end) {
    auto* context = static_cast<SumContext*>(user_data);
    std::uint64_t local = 0;
    for (std::uint64_t i = begin; i < end; ++i) {
        local += context->values[i];
    }
    context->total.fetch_add(local, std::memory_order_relaxed);
    context->chunk_count.fetch_add(1, std::memory_order_relaxed);
}

}  // namespace

PN_TEST(c_api, create_and_destroy) {
    PnJobSystem* system = pn_job_system_create(2);
    PN_REQUIRE(system != nullptr);
    PN_CHECK_EQ(pn_job_system_worker_count(system), 2u);
    pn_job_system_destroy(system);
}

PN_TEST(c_api, destroy_tolerates_null) {
    pn_job_system_destroy(nullptr);
    PN_CHECK_EQ(pn_job_system_worker_count(nullptr), 0u);
}

PN_TEST(c_api, null_arguments_are_rejected_not_dereferenced) {
    PN_CHECK_EQ(pn_job_system_parallel_for(nullptr, 10, 1, &sum_chunk, nullptr),
                PN_JOB_ERROR_NULL_ARGUMENT);

    PnJobSystem* system = pn_job_system_create(1);
    PN_REQUIRE(system != nullptr);
    PN_CHECK_EQ(pn_job_system_parallel_for(system, 10, 1, nullptr, nullptr),
                PN_JOB_ERROR_NULL_ARGUMENT);
    pn_job_system_destroy(system);

    PN_CHECK_EQ(pn_job_run_serial(10, nullptr, nullptr), PN_JOB_ERROR_NULL_ARGUMENT);
}

PN_TEST(c_api, zero_count_is_success_not_an_error) {
    PnJobSystem* system = pn_job_system_create(2);
    PN_REQUIRE(system != nullptr);
    SumContext context{nullptr, {0}, {0}};
    PN_CHECK_EQ(pn_job_system_parallel_for(system, 0, 16, &sum_chunk, &context), PN_JOB_OK);
    PN_CHECK_EQ(context.chunk_count.load(), 0u);
    pn_job_system_destroy(system);
}

PN_TEST(c_api, parallel_result_matches_the_serial_reference) {
    constexpr std::uint64_t kCount = 50'000;
    std::vector<std::uint64_t> values(kCount);
    std::iota(values.begin(), values.end(), std::uint64_t{1});

    PnJobSystem* system = pn_job_system_create(3);
    PN_REQUIRE(system != nullptr);

    SumContext parallel{values.data(), {0}, {0}};
    PN_REQUIRE_EQ(pn_job_system_parallel_for(system, kCount, 512, &sum_chunk, &parallel),
                  PN_JOB_OK);

    SumContext serial{values.data(), {0}, {0}};
    PN_REQUIRE_EQ(pn_job_run_serial(kCount, &sum_chunk, &serial), PN_JOB_OK);

    PN_CHECK_EQ(parallel.total.load(), serial.total.load());
    PN_CHECK_EQ(serial.chunk_count.load(), 1u);

    pn_job_system_destroy(system);
}

PN_TEST(c_api, crossing_frequency_is_per_chunk_not_per_element) {
    // The claim the whole hybrid rests on, asserted rather than assumed. If a
    // future change makes the boundary fine-grained, this fails loudly instead
    // of quietly costing an order of magnitude.
    constexpr std::uint64_t kCount = 50'000;
    constexpr std::uint64_t kGrain = 512;

    std::vector<std::uint64_t> values(kCount, 1);

    PnJobSystem* system = pn_job_system_create(3);
    PN_REQUIRE(system != nullptr);

    SumContext context{values.data(), {0}, {0}};
    PN_REQUIRE_EQ(pn_job_system_parallel_for(system, kCount, kGrain, &sum_chunk, &context),
                  PN_JOB_OK);

    // Every element visited exactly once.
    PN_CHECK_EQ(context.total.load(), kCount);

    // ...across about count/grain crossings, not count of them.
    const std::uint64_t crossings = context.chunk_count.load();
    const std::uint64_t expected = (kCount + kGrain - 1) / kGrain;
    PN_CHECK_EQ(crossings, expected);
    PN_CHECK_LT(crossings, kCount / 10);

    pn_job_system_destroy(system);
}

PN_TEST(c_api, nested_parallel_for_does_not_deadlock) {
    // The calling thread participates rather than blocking, so a parallel body
    // may itself call parallel_for. A pool that blocked here would deadlock.
    PnJobSystem* system = pn_job_system_create(2);
    PN_REQUIRE(system != nullptr);

    static PnJobSystem* s_system = system;
    static std::atomic<std::uint64_t> s_inner{0};
    s_inner.store(0);

    struct Outer {
        static void run(void*, std::uint64_t begin, std::uint64_t end) {
            for (std::uint64_t i = begin; i < end; ++i) {
                pn_job_system_parallel_for(
                    s_system, 100, 10,
                    [](void*, std::uint64_t b, std::uint64_t e) {
                        s_inner.fetch_add(e - b, std::memory_order_relaxed);
                    },
                    nullptr);
            }
        }
    };

    PN_REQUIRE_EQ(pn_job_system_parallel_for(system, 8, 1, &Outer::run, nullptr), PN_JOB_OK);
    PN_CHECK_EQ(s_inner.load(), 800u);

    pn_job_system_destroy(system);
}
