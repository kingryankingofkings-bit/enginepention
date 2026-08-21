# Feature Traceability Matrix

Maps each requirement that has left `NOT_STARTED` to its implementation, its
tests, and the evidence supporting its current state.

**A requirement is `VERIFIED` only when executable behaviour plus its assigned
acceptance evidence both exist.** An interface, a stub, a passing compile, or a
design document is not evidence.

Current totals: **5 `VERIFIED`**, 14 `IMPLEMENTED_UNVERIFIED`, 18 `BLOCKED`,
177 `NOT_STARTED`, of 214. Nothing is `DEFERRED_BY_SCOPE`.

No requirement changed state in the Vulkan binding increment. The bindings are
infrastructure under PN-RND-001, which stays `BLOCKED`: nothing here has
enumerated an adapter, because there is no GPU to enumerate one on.

Several entries below are `IMPLEMENTED_UNVERIFIED` despite substantial passing
tests, because their acceptance criteria are not fully met - most often because
the criterion names something this environment cannot exercise, such as scaling
with core count or the absence of visible stutter. Those gaps are stated per
requirement rather than rounded up.

## Test totals, as counted from the built binaries

162 registered C++ tests across 14 binaries, executing 12,483 assertions, plus
67 Rust tests across 6 suites. All pass. Counted by running each binary rather
than by reading the source, because a test that fails to register is invisible
to a source count.

The C++ figures are re-measured here; an earlier revision of this file said 152
tests across 13 binaries, which had gone stale by one binary and one round of
test fixes.

**Independently reproduced on a clean machine.** GitHub Actions run
[32481092505](https://github.com/kingryankingofkings-bit/enginepention/actions/runs/32481092505)
(push) and
[32481200055](https://github.com/kingryankingofkings-bit/enginepention/actions/runs/32481200055)
(pull request) each executed all 11 CI jobs - the clean-room and layering checks
plus all 10 build configurations - from a fresh checkout on a runner that shares
nothing with the development container. All 22 check runs concluded `success`.

This matters more than the local run it duplicates: it is the first evidence in
this repository that the build reproduces somewhere other than the machine it
was written on.

| C++ binary | Tests | Checks |
|---|---|---|
| `pn_test_testing_self` | 16 | 49 |
| `pn_test_math_vector` | 7 | 18 |
| `pn_test_math_matrix` | 7 | 32 |
| `pn_test_math_convention` | 6 | 29 |
| `pn_test_math_precision` | 6 | 13 |
| `pn_test_core_expected` | 17 | 51 |
| `pn_test_core_handle` | 10 | 105 |
| `pn_test_core_memory` | 30 | 365 |
| `pn_test_platform_virtual_memory` | 12 | 56 |
| `pn_test_platform_clock` | 13 | 2,036 |
| `pn_test_platform_thread` | 4 | 4 |
| `pn_test_jobs_deque` | 10 | 8,154 |
| `pn_test_jobs_job_system` | 17 | 1,548 |
| `pn_test_jobs_c_api` | 7 | 23 |
| **Total** | **162** | **12,483** |

The Rust workspace reports pass/fail per test rather than per assertion, so its
tests are counted separately rather than folded into a total that would mean two
different things:

| Rust suite | Tests | Covers |
|---|---|---|
| `pn-jobs` `boundary.rs` | 9 | The FFI contract: one crossing per operation and per chunk, never per element; a panicking body contained and reported |
| `vkgen` `xml_parser.rs` | 15 | The pull parser, including mixed content in document order |
| `vkgen` `registry_model.rs` | 11 | Extension-enum numbering, core-version layering, funcpointer signatures, `implicitexternsyncparams` exclusion |
| `vkgen` `emit.rs` | 11 | Emitted shape: transparent newtypes, bit 63 unsigned, C array extent order, keyword escaping, dispatch-table classification |
| `vkgen` `sha256.rs` | 6 | FIPS PUB 180-4 published vectors |
| `pn-vulkan-sys` `bindings.rs` | 15 | Constant values and layouts against the Vulkan specification |
| **Total** | **67** | |

Concurrency soak beyond the matrix: 48 further runs of the two jobs suites under
ThreadSanitizer (both compilers) and AddressSanitizer, zero failures and zero
warnings.

## Verified

| ID | Requirement | Implementation | Tests | Evidence |
|---|---|---|---|---|
| PN-PLT-026 | In-project unit test framework | `engine/testing/` | `engine/testing/tests/self_test.cpp` (9 tests) | Framework self-tests assert that failing checks are recorded, that a failed `PN_REQUIRE` aborts the body while a failed `PN_CHECK` does not, and that a genuine failure exits non-zero. `docs/evidence/build-matrix.txt` |
| PN-PLT-027 | `Expected`-based fallible-return convention, exception-independent | `engine/core/include/pn/core/{error,expected}.hpp` | `engine/core/tests/expected_test.cpp` (17 tests) | Passes under GCC and Clang, Debug and Release, **and with `-fno-exceptions`** under both compilers, and under ASan+UBSan and TSan. `static_assert` pins trivial destructibility for trivial payloads. `docs/evidence/build-matrix.txt` |
| PN-PLT-011 | Generational handles that detect stale access | `engine/core/include/pn/core/handle.hpp` | `engine/core/tests/handle_test.cpp` (10 tests) | A recycled slot is proven to invalidate the prior handle rather than alias it; double-free is reported rather than ignored; never-issued, out-of-range, freed, and stale are distinguished by error category |
| PN-PLT-006 | Tagged allocators, arenas, pools, alignment | `engine/core/include/pn/core/memory.hpp` | `engine/core/tests/memory_test.cpp` (30 tests) | Alignment honoured from a deliberately misaligned cursor across five alignments; padding cannot overrun the end; marker rewind exact; pool refuses an interior or foreign pointer rather than corrupting its free list; full allocate/free/reallocate cycle returns every block |
| PN-PLT-008 | Out-of-memory handling with defined recovery | `engine/core/include/pn/core/memory.hpp` | `engine/core/tests/memory_test.cpp` | Exhaustion returns `out_of_memory` rather than aborting, and the allocator remains usable and uncorrupted afterwards - asserted, not assumed |

## Partially verified

| ID | Requirement | State | What is done | What is not |
|---|---|---|---|---|
| PN-WLD-001 | Double-precision world coordinates | `IMPLEMENTED_UNVERIFIED` | `f64` world space and camera-relative `f32` render space exist as **distinct types**, so mixing them is a compile error. Precision at 500 km is asserted, and a negative-control test proves the naive `f32` approach fails there. `engine/math/tests/precision_test.cpp` | The acceptance criterion is stated in terms of *rendered* output - no vertex jitter or shadow crawl. That requires a renderer and a GPU (`BLOCK-002`). The arithmetic is verified; the rendering claim is not |
| PN-WLD-002 | Stable physics/render coordinate conversion | `IMPLEMENTED_UNVERIFIED` | Round-trip conversion is bounded and asserted; the view matrix is proven to carry no translation. `engine/math/tests/matrix_test.cpp` | No physics engine exists yet, so the physics half of the contract is untested |
| PN-OPS-003 | Warning discipline, static analysis, sanitizers | `IMPLEMENTED_UNVERIFIED` | Warnings-as-errors across a broad set under both compilers; ASan+UBSan and TSan configurations exist and pass. `docs/evidence/build-matrix.txt` | Static analysis (clang-tidy or equivalent) is not yet configured |
| PN-PLT-018 | Module boundaries and dependency direction | `IMPLEMENTED_UNVERIFIED` | `build_scripts/check_layering.py` enforces the dependency graph and the single-graphics-API rule, and is **proven to fail** on deliberate violations. `docs/evidence/enforcement-checks.txt` | Only three modules exist, so the graph is barely exercised. The rule matters most at modules that do not yet exist |
| PN-OPS-001 | Build configurations | `IMPLEMENTED_UNVERIFIED` | Debug and Release under both compilers, plus no-exceptions and two sanitizer configurations - 10 in total, all green locally and on clean CI runners | The editor, server, profile, and shipping configurations do not exist because the code they would configure does not exist |
| PN-OPS-002 | Incremental and clean builds, toolchain lockfile, build cache policy | `IMPLEMENTED_UNVERIFIED` | A clean build from a fresh checkout succeeds on CI, which is one of this requirement's four parts | No toolchain lockfile, no declared build cache policy, and no generated-code tracking. Three of four parts are absent, so this is deliberately not `VERIFIED` |
| PN-OPS-007 | Test categories | `IMPLEMENTED_UNVERIFIED` | Unit tests exist and run in CI; a concurrency soak exists for the scheduler | Property, golden-image, scene, performance, fuzz, network-chaos, recovery, and end-to-end categories do not exist yet |
| PN-PLT-013 | Work-stealing job graph | `IMPLEMENTED_UNVERIFIED` | Correct under concurrency: 48 sanitizer runs clean; exact-once delivery asserted across thousands of items; steal counters asserted non-zero so a silent degradation to single-threaded fails | **That it scales is not shown.** The acceptance criterion is scaling with core count, and this host has four cores and no performance budget to measure against (`BLOCK-002`). Scaling is a target, not an achievement |
| PN-PLT-014 | Synchronization, cancellation, deadlock diagnostics | `IMPLEMENTED_UNVERIFIED` | ThreadSanitizer clean under both compilers; cancellation releases waiters; nested waiting does not deadlock | No deadlock *diagnostics* - detection and reporting of a stuck graph does not exist |
| PN-PLT-005 | Virtual memory reservation and commit | `IMPLEMENTED_UNVERIFIED` | POSIX backend verified: reserve without commit, growth preserving both base address and contents, decommit, idempotent release, and composition with a core arena | The **Windows** backend - the declared first target platform - is written but cannot be compiled or run here (`BLOCK-001`) |
| PN-PLT-007 | Memory tracking and leak detection | `IMPLEMENTED_UNVERIFIED` | Per-tag attribution, retained peaks, and a live-allocation check that is the shape of the shutdown leak assertion | Guard pages in diagnostic builds do not exist |
| PN-PLT-003 | Monotonic time, fixed and variable ticks, frame pacing | `IMPLEMENTED_UNVERIFIED` | `FixedTimestep` is a pure function of its deltas, so pacing is tested deterministically: simulation time conserved exactly over 1,000 uneven frames, oversized deltas clamped, step count capped against a spiral, negative deltas neutralised | Simulation pause/step and time scaling do not exist |
| PN-PLT-012 | Thread abstraction, affinity, naming | `IMPLEMENTED_UNVERIFIED` | Hardware thread count never returns zero; naming is best-effort and safe from any thread | Affinity is not implemented |
| PN-PHY-016 | Fixed-step policy with render interpolation | `IMPLEMENTED_UNVERIFIED` | The accumulator and interpolation factor are implemented and tested | Its acceptance criterion is "no visible stutter", which needs a renderer that does not exist |

## Documented conventions with enforcing tests

These are not requirements in their own right; they are the
[conventions](docs/conventions.md) whose violation produces bugs that present as
something else. Each has a test so the convention is enforced rather than merely
written down.

| Convention | Test |
|---|---|
| Right-handed, Y-up, -Z-forward basis | `convention.basis_is_right_handed_y_up` |
| Reversed-Z: near maps to 1.0, far approaches 0.0 | `convention.reversed_z_maps_near_to_one_and_far_toward_zero` |
| Infinite far plane - no far term in the projection | `convention.projection_has_no_far_plane_term` |
| Column-major matrix storage, no hidden transpose | `convention.matrix_storage_is_column_major` |
| Angles are radians internally | `convention.angles_round_trip_between_degrees_and_radians` |
| f32 ULP figures quoted in `docs/conventions.md` | `precision.f32_ulp_table_matches_conventions_document` |
| Camera-relative rendering preserves detail at 500 km | `precision.camera_relative_conversion_preserves_detail_at_500km` |
| Naive f32 world space fails at 500 km (negative control) | `precision.naive_f32_world_space_visibly_fails_at_500km` |
| Comparison macros evaluate each operand exactly once | `framework.check_eq_evaluates_each_operand_exactly_once` and six siblings |
| A deque item is never taken twice, and never lost | `deque.concurrent_drain_takes_every_item_exactly_once` |
| The scheduler genuinely steals rather than degrading to single-threaded | `job_system.stealing_actually_happens_under_load` |

## Blocked

The 18 requirements blocked by this environment are listed with their blockers
in [FEATURE_REQUIREMENTS_CATALOG.md](FEATURE_REQUIREMENTS_CATALOG.md) and
[docs/ENVIRONMENT_BASELINE.md](docs/ENVIRONMENT_BASELINE.md) section 3. None has
been reclassified as `DEFERRED_BY_SCOPE`.

One of them has work behind it that is worth stating precisely, because it is
the kind of thing that gets rounded up:

| ID | State | What exists | What does not |
|---|---|---|---|
| PN-RND-001 | `BLOCKED` (`BLOCK-002`) | `rust/tools/vkgen` and `rust/crates/pn-vulkan-sys`: 563 Vulkan types and 229 commands generated from the pinned Khronos registry, compiling without warnings, with 43 generator tests and 15 binding tests passing. CI re-derives the committed output ([ADR-0010](docs/adr/ADR-0010-vulkan-bindings-from-the-registry.md)) | No adapter has been enumerated, no device created, no Vulkan call made. The layout tests check what C would produce for these declarations; they cannot check what a driver expects, because there is no driver here |
