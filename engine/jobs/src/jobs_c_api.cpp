// Pention Engine - jobs/jobs_c_api.cpp
// Requirement: PN-PLT-013, PN-OBJ-017
// Decision:    ADR-0009

#include "pn/jobs/jobs_c_api.h"

#include "pn/jobs/job_system.hpp"

#include <memory>
#include <new>

namespace {

/// The opaque handle's real type.
///
/// A wrapper rather than JobSystem itself, so the C ABI stays stable if the C++
/// type gains members, and so construction failure can be reported rather than
/// leaving a half-started object reachable.
struct JobSystemHandle {
    pn::jobs::JobSystem system;
};

}  // namespace

extern "C" {

PnJobSystem* pn_job_system_create(uint32_t worker_count) {
    // Nothing here may throw across the boundary. new(std::nothrow) reports
    // allocation failure as a null pointer, which is what the C caller expects.
    auto* handle = new (std::nothrow) JobSystemHandle{};
    if (handle == nullptr) {
        return nullptr;
    }
    if (!handle->system.start(static_cast<std::size_t>(worker_count)).has_value()) {
        delete handle;
        return nullptr;
    }
    return reinterpret_cast<PnJobSystem*>(handle);
}

void pn_job_system_destroy(PnJobSystem* system) {
    if (system == nullptr) {
        return;
    }
    auto* handle = reinterpret_cast<JobSystemHandle*>(system);
    handle->system.stop();
    delete handle;
}

uint32_t pn_job_system_worker_count(const PnJobSystem* system) {
    if (system == nullptr) {
        return 0;
    }
    const auto* handle = reinterpret_cast<const JobSystemHandle*>(system);
    return static_cast<uint32_t>(handle->system.worker_count());
}

PnJobStatus pn_job_system_parallel_for(PnJobSystem* system,
                                       uint64_t count,
                                       uint64_t grain,
                                       PnParallelForBody body,
                                       void* user_data) {
    if (system == nullptr || body == nullptr) {
        return PN_JOB_ERROR_NULL_ARGUMENT;
    }
    if (count == 0) {
        return PN_JOB_OK;  // nothing to do is not an error
    }

    auto* handle = reinterpret_cast<JobSystemHandle*>(system);

    // The C++ side owns the fan-out. `body` is invoked once per chunk, which is
    // the coarse crossing ADR-0009 requires: the boundary is paid once per
    // chunk, never once per index.
    handle->system.parallel_for(
        static_cast<std::size_t>(count),
        static_cast<std::size_t>(grain),
        [body, user_data](std::size_t begin, std::size_t end) noexcept {
            body(user_data, static_cast<uint64_t>(begin), static_cast<uint64_t>(end));
        });

    return PN_JOB_OK;
}

PnJobStatus pn_job_run_serial(uint64_t count, PnParallelForBody body, void* user_data) {
    if (body == nullptr) {
        return PN_JOB_ERROR_NULL_ARGUMENT;
    }
    if (count == 0) {
        return PN_JOB_OK;
    }
    body(user_data, 0, count);
    return PN_JOB_OK;
}

}  // extern "C"
