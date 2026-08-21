// Pention Engine - platform/clock.hpp
// Requirement: PN-PLT-003 (monotonic time, fixed and variable ticks, frame pacing),
//              PN-PHY-016 (fixed-step policy with render interpolation)
// Decision:    docs/conventions.md, "Time"

#ifndef PN_PLATFORM_CLOCK_HPP
#define PN_PLATFORM_CLOCK_HPP

#include <chrono>
#include <cstdint>

namespace pn::platform {

using Nanoseconds = std::chrono::nanoseconds;

/// Monotonic time source.
///
/// Steady, not wall-clock: the system clock can jump backwards when NTP
/// corrects it, and a simulation that reacts to a negative delta is a
/// simulation that occasionally explodes for reasons nobody can reproduce.
class Clock {
public:
    using Impl = std::chrono::steady_clock;
    using TimePoint = Impl::time_point;

    [[nodiscard]] static TimePoint now() noexcept { return Impl::now(); }

    static_assert(Impl::is_steady, "the engine requires a steady clock");
};

/// Seconds as a double, for interfaces where a duration reads better that way.
[[nodiscard]] constexpr double to_seconds(Nanoseconds duration) noexcept {
    return static_cast<double>(duration.count()) / 1'000'000'000.0;
}

[[nodiscard]] constexpr Nanoseconds from_seconds(double seconds) noexcept {
    return Nanoseconds{static_cast<std::int64_t>(seconds * 1'000'000'000.0)};
}

[[nodiscard]] constexpr Nanoseconds from_hz(double hz) noexcept {
    return hz <= 0.0 ? Nanoseconds{0} : from_seconds(1.0 / hz);
}

/// Fixed-timestep accumulator driving simulation, with an interpolation factor
/// for rendering between the two most recent simulation states.
///
/// Deliberately a pure function of the deltas handed to it: it never reads the
/// clock itself. That makes the pacing logic - which is where the subtle bugs
/// live - fully testable with synthetic deltas, deterministically, without any
/// timing flakiness.
///
/// Two protections, both required by docs/conventions.md:
///
///  * The incoming delta is **clamped**, so a stall, a breakpoint, or a
///    minimised window does not deliver a multi-second delta that the
///    simulation then tries to catch up on all at once.
///  * The step count per call is **capped**. If simulation is persistently
///    slower than real time, an uncapped accumulator asks for more steps each
///    frame than the last, which is the classic spiral of death. Capping trades
///    an honest slowdown for a freeze.
class FixedTimestep {
public:
    struct Update {
        /// How many fixed steps to run this frame.
        int steps = 0;
        /// Fraction of a step remaining, in [0, 1). Render state interpolates
        /// between the previous and current simulation states by this amount.
        float alpha = 0.0f;
        /// True when the step cap was hit and simulation time was discarded to
        /// avoid a spiral. Surfaced rather than hidden: silently dropping
        /// simulation time is exactly the kind of thing that should appear in a
        /// diagnostic overlay.
        bool clamped = false;
    };

    static constexpr int kDefaultMaxStepsPerFrame = 8;

    /// @param step        simulation period, 1/60 s by convention
    /// @param max_delta   the largest frame delta accepted before clamping
    /// @param max_steps   the most steps one call may return
    explicit FixedTimestep(Nanoseconds step = from_hz(60.0),
                           Nanoseconds max_delta = Nanoseconds{100'000'000},  // 100 ms
                           int max_steps = kDefaultMaxStepsPerFrame) noexcept
        : step_(step.count() > 0 ? step : from_hz(60.0)),
          max_delta_(max_delta),
          max_steps_(max_steps > 0 ? max_steps : 1) {}

    [[nodiscard]] Update advance(Nanoseconds frame_delta) noexcept {
        Update update;

        // A negative delta means the caller's time source moved backwards.
        // Treat it as zero rather than propagating it into the accumulator.
        if (frame_delta.count() < 0) {
            frame_delta = Nanoseconds{0};
        }
        if (frame_delta > max_delta_) {
            frame_delta = max_delta_;
            update.clamped = true;
        }

        accumulated_ += frame_delta;

        while (accumulated_ >= step_ && update.steps < max_steps_) {
            accumulated_ -= step_;
            ++update.steps;
        }

        if (accumulated_ >= step_) {
            // Still behind after the cap: discard the backlog so the next frame
            // starts fresh instead of compounding.
            accumulated_ = Nanoseconds{0};
            update.clamped = true;
        }

        update.alpha = static_cast<float>(
            static_cast<double>(accumulated_.count()) / static_cast<double>(step_.count()));
        return update;
    }

    void reset() noexcept { accumulated_ = Nanoseconds{0}; }

    [[nodiscard]] Nanoseconds step() const noexcept { return step_; }
    [[nodiscard]] Nanoseconds accumulated() const noexcept { return accumulated_; }
    [[nodiscard]] int max_steps() const noexcept { return max_steps_; }

private:
    Nanoseconds step_;
    Nanoseconds max_delta_;
    Nanoseconds accumulated_{0};
    int max_steps_;
};

/// Monotonic frame counter.
///
/// 64-bit and never reset. Temporal techniques index off it, and a counter that
/// wraps or resets produces flicker that is maddening to trace back to its
/// cause.
class FrameCounter {
public:
    [[nodiscard]] std::uint64_t index() const noexcept { return index_; }
    std::uint64_t advance() noexcept { return ++index_; }

private:
    std::uint64_t index_ = 0;
};

}  // namespace pn::platform

#endif  // PN_PLATFORM_CLOCK_HPP
