// Pention Engine - platform/tests/clock_test.cpp
// Requirement: PN-PLT-003, PN-PHY-016
// Decision:    docs/conventions.md, "Time"
//
// FixedTimestep never reads the clock, so every one of these tests is exact and
// deterministic. Pacing bugs are subtle and timing-dependent tests are flaky;
// making the logic a pure function of its inputs is what lets it be tested at
// all.

#include "pn/platform/clock.hpp"
#include "pn/testing/test.hpp"

namespace {

using pn::platform::Clock;
using pn::platform::FixedTimestep;
using pn::platform::FrameCounter;
using pn::platform::Nanoseconds;
using pn::platform::from_hz;
using pn::platform::from_seconds;
using pn::platform::to_seconds;

}  // namespace

PN_TEST(clock, conversions_round_trip) {
    PN_CHECK_NEAR(to_seconds(from_seconds(0.25)), 0.25, 1e-9);
    PN_CHECK_NEAR(to_seconds(from_hz(60.0)), 1.0 / 60.0, 1e-6);
    PN_CHECK_NEAR(to_seconds(from_hz(120.0)), 1.0 / 120.0, 1e-6);
    // A non-positive rate must not divide by zero.
    PN_CHECK_EQ(from_hz(0.0).count(), 0);
    PN_CHECK_EQ(from_hz(-5.0).count(), 0);
}

PN_TEST(clock, monotonic_source_never_goes_backwards) {
    const auto first = Clock::now();
    for (int i = 0; i < 1000; ++i) {
        const auto later = Clock::now();
        PN_REQUIRE(later >= first);
    }
}

PN_TEST(fixed_timestep, exact_step_yields_exactly_one_step) {
    FixedTimestep timestep{from_hz(60.0)};
    const auto update = timestep.advance(timestep.step());
    PN_CHECK_EQ(update.steps, 1);
    PN_CHECK_NEAR(update.alpha, 0.0f, 1e-5f);
    PN_CHECK(!update.clamped);
}

PN_TEST(fixed_timestep, sub_step_delta_runs_nothing_and_accumulates) {
    FixedTimestep timestep{Nanoseconds{1000}};
    const auto first = timestep.advance(Nanoseconds{400});
    PN_CHECK_EQ(first.steps, 0);
    PN_CHECK_NEAR(first.alpha, 0.4f, 1e-5f);

    const auto second = timestep.advance(Nanoseconds{400});
    PN_CHECK_EQ(second.steps, 0);
    PN_CHECK_NEAR(second.alpha, 0.8f, 1e-5f);

    // The third crosses the boundary.
    const auto third = timestep.advance(Nanoseconds{400});
    PN_CHECK_EQ(third.steps, 1);
    PN_CHECK_NEAR(third.alpha, 0.2f, 1e-5f);
}

PN_TEST(fixed_timestep, a_long_frame_runs_several_steps) {
    FixedTimestep timestep{Nanoseconds{1000}};
    const auto update = timestep.advance(Nanoseconds{3500});
    PN_CHECK_EQ(update.steps, 3);
    PN_CHECK_NEAR(update.alpha, 0.5f, 1e-5f);
}

PN_TEST(fixed_timestep, simulation_time_is_conserved_across_many_frames) {
    // The property that actually matters: over a long run, the number of steps
    // taken must match the elapsed time divided by the step, with at most one
    // step of remainder. Drift here becomes desynchronised physics and replay.
    constexpr Nanoseconds step{1'000'000};
    FixedTimestep timestep{step, Nanoseconds{1'000'000'000}, 1000};

    // Deliberately uneven deltas, none an exact multiple of the step.
    constexpr Nanoseconds deltas[] = {
        Nanoseconds{700'000}, Nanoseconds{1'300'000}, Nanoseconds{2'100'000},
        Nanoseconds{450'000}, Nanoseconds{3'900'000},
    };

    long long total_steps = 0;
    Nanoseconds total_time{0};
    for (int repeat = 0; repeat < 200; ++repeat) {
        for (const Nanoseconds delta : deltas) {
            total_time += delta;
            total_steps += timestep.advance(delta).steps;
        }
    }

    const long long expected = total_time.count() / step.count();
    PN_CHECK_EQ(total_steps, expected);
}

PN_TEST(fixed_timestep, oversized_delta_is_clamped) {
    // A breakpoint, a stall, or a minimised window delivers a delta the
    // simulation must not try to catch up on all at once.
    constexpr Nanoseconds step{1'000'000};
    constexpr Nanoseconds max_delta{10'000'000};
    FixedTimestep timestep{step, max_delta, 1000};

    const auto update = timestep.advance(Nanoseconds{60'000'000'000});  // 60 seconds
    PN_CHECK(update.clamped);
    // At most max_delta worth of simulation, not 60 seconds worth.
    PN_CHECK_LE(update.steps, 10);
}

PN_TEST(fixed_timestep, step_count_is_capped_to_prevent_a_spiral) {
    // If simulation is persistently slower than real time, an uncapped
    // accumulator asks for more steps every frame than the last, and the frame
    // rate collapses to a freeze. Capping trades an honest slowdown for that.
    constexpr Nanoseconds step{1'000'000};
    FixedTimestep timestep{step, Nanoseconds{1'000'000'000}, 4};

    const auto update = timestep.advance(Nanoseconds{100'000'000});
    PN_CHECK_EQ(update.steps, 4);
    PN_CHECK(update.clamped);

    // The backlog is discarded rather than carried, so the next frame starts
    // clean instead of compounding.
    const auto next = timestep.advance(step);
    PN_CHECK_EQ(next.steps, 1);
    PN_CHECK(!next.clamped);
}

PN_TEST(fixed_timestep, negative_delta_is_treated_as_zero) {
    // A time source that moves backwards must not drive the accumulator
    // negative; that is how a simulation ends up running steps in reverse.
    FixedTimestep timestep{Nanoseconds{1000}};
    PN_REQUIRE(timestep.advance(Nanoseconds{500}).steps == 0);

    const auto update = timestep.advance(Nanoseconds{-100'000});
    PN_CHECK_EQ(update.steps, 0);
    PN_CHECK_NEAR(update.alpha, 0.5f, 1e-5f);
    PN_CHECK(timestep.accumulated().count() >= 0);
}

PN_TEST(fixed_timestep, alpha_stays_within_the_unit_interval) {
    FixedTimestep timestep{Nanoseconds{1000}, Nanoseconds{1'000'000}, 1000};
    for (int i = 1; i <= 500; ++i) {
        const auto update = timestep.advance(Nanoseconds{i * 7});
        PN_REQUIRE(update.alpha >= 0.0f);
        PN_REQUIRE(update.alpha < 1.0f);
    }
}

PN_TEST(fixed_timestep, reset_clears_the_accumulator) {
    FixedTimestep timestep{Nanoseconds{1000}};
    static_cast<void>(timestep.advance(Nanoseconds{700}));
    PN_CHECK_GT(timestep.accumulated().count(), 0);
    timestep.reset();
    PN_CHECK_EQ(timestep.accumulated().count(), 0);
}

PN_TEST(fixed_timestep, degenerate_construction_is_corrected) {
    // A zero step would divide by zero computing alpha; a zero step cap would
    // never advance. Both are corrected rather than trusted.
    FixedTimestep zero_step{Nanoseconds{0}};
    PN_CHECK_GT(zero_step.step().count(), 0);

    FixedTimestep zero_cap{Nanoseconds{1000}, Nanoseconds{100'000}, 0};
    PN_CHECK_GE(zero_cap.max_steps(), 1);
    PN_CHECK_EQ(zero_cap.advance(Nanoseconds{1000}).steps, 1);
}

PN_TEST(frame_counter, advances_monotonically_from_zero) {
    FrameCounter counter;
    PN_CHECK_EQ(counter.index(), 0u);
    PN_CHECK_EQ(counter.advance(), 1u);
    PN_CHECK_EQ(counter.advance(), 2u);
    PN_CHECK_EQ(counter.index(), 2u);
}
