# Feature Traceability Matrix

Maps each requirement that has left `NOT_STARTED` to its implementation, its
tests, and the evidence supporting its current state.

**A requirement is `VERIFIED` only when executable behaviour plus its assigned
acceptance evidence both exist.** An interface, a stub, a passing compile, or a
design document is not evidence.

Current totals: **10 `VERIFIED`**, 17 `IMPLEMENTED_UNVERIFIED`, 18 `BLOCKED`,
169 `NOT_STARTED`, of 214. Nothing is `DEFERRED_BY_SCOPE`.

No requirement changed state in the Vulkan binding increment. The bindings are
infrastructure under PN-RND-001, which stays `BLOCKED`: nothing here has
enumerated an adapter, because there is no GPU to enumerate one on.

Several entries below are `IMPLEMENTED_UNVERIFIED` despite substantial passing
tests, because their acceptance criteria are not fully met - most often because
the criterion names something this environment cannot exercise, such as scaling
with core count or the absence of visible stutter. Those gaps are stated per
requirement rather than rounded up.

## Test totals, as counted from the built binaries

314 registered C++ tests across 21 binaries, executing 2,834,472 assertions,
plus 145 Rust tests across 11 suites. All pass.

The assertion count is dominated by three binaries that assert per sample
rather than per case: the generator's statistical tests (884,727) and the two
container suites' property tests against standard-library oracles (504,516 and
1,066,347). A statistical property asserted once is not asserted, and a
container invariant checked on one hand-written sequence is checked on one
hand-written sequence. Counted by running each binary rather
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
| `pn_test_core_random` | 21 | 884,727 |
| `pn_test_core_containers` | 22 | 704,719 |
| `pn_test_core_collections` | 21 | 1,066,347 |
| `pn_test_core_name` | 18 | 44,405 |
| `pn_test_core_serialize` | 23 | 120,089 |
| `pn_test_core_console` | 27 | 1,639 |
| `pn_test_core_log` | 20 | 63 |
| `pn_test_platform_virtual_memory` | 12 | 56 |
| `pn_test_platform_clock` | 13 | 2,036 |
| `pn_test_platform_thread` | 4 | 4 |
| `pn_test_jobs_deque` | 10 | 8,154 |
| `pn_test_jobs_job_system` | 17 | 1,548 |
| `pn_test_jobs_c_api` | 7 | 23 |
| **Total** | **314** | **2,834,472** |

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
| `pn-rhi` `barrier_derivation.rs` | 19 | Hand-computed barrier timelines, and the hazards that are rejected rather than papered over |
| `pn-render-graph` `compilation.rs` | 20 | Ordering semantics, culling to a fixed point, lifetime spans, four classes of graph hazard, and the visualization |
| `pn-render-graph` `end_to_end.rs` | 5 | Whole frames compiled and then validated against the reference backend |
| `pn-rhi-reference` `validation.rs` | 16 | Nine of them break a correct timeline in one specific way and require the backend to say so |
| `pn-rhi-reference` `rasterization.rs` | 18 | Golden images as text: fill rule, perspective correction, reversed-Z ordering, culling by winding, near-plane clipping, sRGB at output |
| **Total** | **145** | |

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
| PN-PLT-009 | Cache-aware containers authored in-project | `engine/core/include/pn/core/{assert,hash,inline_array,hash_map,sparse_set,ring_buffer,bit_set}.hpp` | `containers_test.cpp` (22 tests), `collections_test.cpp` (21 tests) | Five containers, each chosen for a cache property the standard library does not offer: inline storage, open addressing with backward-shift deletion, contiguous iteration by integer key, a masked wrap, and one bit per object. Unit tests plus property tests driving 100,000-200,000 pseudorandom operations against `std::vector`, `std::map` and `std::deque` as oracles, with a full sweep afterwards so an element that became unreachable without changing the count is still caught. `PN_ASSERT` guards every index whenever `NDEBUG` is not defined. Five defects were introduced deliberately and each was caught - `docs/evidence/enforcement-checks.txt`. The map uses Robin Hood ordering after measurement showed plain linear probing reaching a 146-slot run at a hundred thousand keys; the probe test now runs at that scale rather than at 500 |
| PN-PLT-010 | String, interned name, and handle systems | `engine/core/include/pn/core/{name,string,handle}.hpp` | `name_test.cpp` (18 tests), `handle_test.cpp` (10 tests) | All three parts of the criterion. **Interning collisions:** a deliberately colliding hasher is supplied to the table, and 200 distinct strings that all hash to zero stay 200 distinct names, resolvable by text in both directions; equal-length collisions exercise the byte comparison behind the length check. **Rehash:** 20,000 names force many doublings with every name and its text preserved and no duplicates created, and text views captured before the growth remain valid and at the same address afterwards - the property the chunked storage exists for. **Handle staleness:** covered by PN-PLT-011, already verified |
| PN-PLT-020 | Deterministic serialization with schema migration | `engine/core/include/pn/core/{serialize,checksum}.hpp` | `serialize_test.cpp` (23 tests) | Both clauses. **Golden data:** a committed 31-byte blob is asserted to decode, and separately to be reproduced byte for byte by the writer - checked in both directions, so a failure says which side moved. **Migration:** a version-1 payload is read by version-2 code, the field it never carried is filled from a stated default, and the result is written at the current version and read back unchanged. Encoding is canonical - shortest-form varints, with longer encodings of the same value rejected - and the same values re-encode identically over 20,000 pseudorandom rounds. CRC-32 matches the published check value for `123456789`. Three defects were introduced deliberately and each was caught - `docs/evidence/enforcement-checks.txt` |
| PN-PLT-019 | Configuration, console variables, command execution | `engine/core/include/pn/core/console.hpp`, `engine/core/src/console.cpp` | `console_test.cpp` (27 tests) | Both clauses. **Round-trip:** four variables of four types are set, saved, loaded into a fresh console, compared value by value, and saved again to the identical file - so it is a round-trip rather than a one-way conversion. Only variables that differ from their default and are flagged to persist are written, because writing everything would freeze today's defaults into every user's config file. **Actionable errors:** each parse failure names what was wrong and the column it happened at, asserted exactly - an unterminated string reports the opening quote rather than the end of the line, a stray argument suggests quoting, and a bad boolean lists the spellings it would have taken. A bad line does not discard the rest of a config file, and an unknown variable is reported without rejecting the file, because a config outlives the variables in it |
| PN-PLT-016 | Logging, structured events, assertions with categories and levels | `engine/core/include/pn/core/{log,assert}.hpp`, `engine/core/src/log.cpp` | `log_test.cpp` (20 tests) | Both clauses. **Log capture:** a capturing sink receives level, category, message, structured fields and source location, and copies every string out of the record because the views in one point at the caller's stack. Per-category thresholds, records suppressed below a threshold without evaluating their arguments, and 800 records from four threads all arriving with the dispatch TSan-clean. **Assertion fires with source location:** the handler is now replaceable, which is what made this testable at all - an assertion that only ever aborts cannot be observed by the process it aborts - and is what a crash reporter needs anyway. The expression, message, file and line are asserted exactly. Both compile-time gates have a test of their own where they are turned off and the *absence* of an effect is checked |

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

### PN-RND-003 - Synchronization and barriers

| Field | Value |
|---|---|
| State | `IMPLEMENTED_UNVERIFIED` |
| Implementation | `rust/crates/pn-rhi/src/{barrier,derive}.rs` |
| Tests | `rust/crates/pn-rhi/tests/barrier_derivation.rs` (19 tests) |
| What is demonstrated | Sync, access, and arrangement are three independent axes; barriers are derived from declared pass intents rather than written by passes; hand-computed timelines for a four-pass frame are asserted exactly; seven classes of hazard are rejected rather than turned into an over-conservative barrier; a chain of sixteen identical readers costs one barrier, not sixteen |
| Reference-backend evidence | `rust/crates/pn-rhi-reference` exists and validates the derivation independently: it reconstructs each resource's state from the derived barriers alone, then checks the pass intents against it. Nine tests break a correct timeline in one specific way - a deleted barrier, a source scope that names the wrong writer, a transition from an arrangement the resource is not in, an unordered write-after-read - and require it to report exactly that. An empty timeline for a real frame is rejected everywhere, so a clean report is not the vacuous kind. Five whole frames are compiled by the render graph and replayed through it |
| Why not `VERIFIED` | The catalog's criterion is met; [ADR-0006](docs/adr/ADR-0006-rhi-barrier-model.md)'s is not, and it requires the **hardware debug layer with synchronization validation to be silent**. The two check different things: the reference backend establishes that the derivation is self-consistent against an independent model of the same specification, and cannot establish that the model matches a driver - especially as the model was written by the same author as the thing it checks. Recorded as [CONF-004](docs/INSTRUCTION_CONFLICTS.md), resolved toward the stricter bar |
| Defect found | The derivation initially treated read-after-read as always free. It is not: a barrier makes a write visible to the reader's stages, so a second reader in a stage nobody has synchronised for still races the writer even though the arrangement has not changed. Caught by a test written to separate the two cases |

### PN-RND-005 - Render graph

| Field | Value |
|---|---|
| State | `IMPLEMENTED_UNVERIFIED` |
| Implementation | `rust/crates/pn-render-graph/src/{graph,compile,visualize}.rs` |
| Tests | `rust/crates/pn-render-graph/tests/compilation.rs` (20 tests) |
| What is demonstrated | Both halves of the acceptance criterion. Hazards detected by test: reading a transient nothing wrote, using a buffer as a texture, a barrier-derivation rejection surfacing rather than being swallowed, and a write discarded by a later write. Plus culling iterated to a fixed point through a three-pass chain, pinned passes surviving with no consumer, lifetime spans in execution order with an aliasing candidate set, and a DOT visualization asserted byte for byte |
| Why not `VERIFIED` | The requirement says **data-driven**, and this graph is built through a Rust API rather than from data. Pass declarations coming from a config or asset does not exist. Separately, nothing has recorded a compiled graph on hardware |
| Defects found | Two, both mine, both found by tests whose premise turned out to be wrong. `disjoint_from` offered a persistent resource as an aliasing candidate because it filtered on lifetime span alone - the swapchain image is not the graph's memory to reuse. And the ordering semantics were never stated: I had assumed the graph would reorder a consumer declared before its producer, which would mean moving a declaration silently changed which frame's contents a pass sampled. Declaration order versions resources, so that case is a hazard, not a reordering opportunity, and a consequence is that dependency edges always run forward and the cycle check is an invariant guard rather than a feature. Both facts are now written down and asserted |

### PN-PLT-004 - Deterministic seeded RNG

| Field | Value |
|---|---|
| State | `IMPLEMENTED_UNVERIFIED` |
| Implementation | `engine/core/include/pn/core/random.hpp` |
| Tests | `engine/core/tests/random_test.cpp` (21 tests, 884,727 checks) |
| What is demonstrated | Determinism across **compilers**: a recorded eight-value sequence is asserted literally, so a compiler that disagrees fails rather than passing quietly. Confirmed identical under g++ and clang++ at `-O0` and `-O2`, and across all ten CI configurations. Streams of one seed are independent and are not shifted copies of each other at any of sixteen offsets. Seeking ten million draws ahead agrees with iterating. Avalanche measured over 32,768 single-bit perturbations: mean 31-33 output bits flipped, no perturbation below 12 or above 52. The mixer is injective over a million consecutive inputs. Every output bit is set 49-51% of the time over 200,000 draws. `uniform` uses rejection sampling and shows no low-end bias over 1.2 million draws where `%` would |
| Why not `VERIFIED` | The criterion says "across platforms **and** compilers". The compiler half is evidenced; the platform half has one platform. Linux x86-64 is all this environment has (`BLOCK-001`), and an argument that the code contains no implementation-defined behaviour is an argument, not a measurement. The recorded sequence is the artifact that closes this: the same test on a Windows or ARM host either matches it or does not |
| Not claimed | Cryptographic strength, or a pass through any published statistical battery. None has been run here, and repeating a quality claim from another generator's literature would be describing evidence this project does not have. The header says so at the point of use |

### ADR-0005 - Reference backend (not a requirement; recorded because things depend on it)

Both halves described by [ADR-0005](docs/adr/ADR-0005-reference-backend.md) now
exist: the validating command recorder, and correctness-oriented rasterization
sufficient for golden-image tests.

| Field | Value |
|---|---|
| Implementation | `rust/crates/pn-rhi-reference/src/{device,trace,raster}.rs` |
| Tests | 34 - 16 in `validation.rs`, 18 in `rasterization.rs` |
| Golden images | Text, not image files. A failing comparison prints the picture beside the expected one in the test output, so the difference is visible without opening a viewer or trusting an encoder. A golden image nobody can read is a golden image nobody will update correctly |
| Conventions enforced | Reversed-Z with depth clearing to 0.0 and greater-or-equal comparison; the Y flip between NDC and framebuffer rows; counter-clockwise-front under right-handed coordinates; linear colour throughout with sRGB applied exactly once at output (linear 0.5 encodes to 188, not 128) |
| Defect found | Geometry lying exactly on the near plane was clipped away. The near test used a positive epsilon, so `z == w` - which *is* the near plane - fell outside it. This presents as surfaces vanishing the moment the camera reaches them, and it was invisible until a test drew a quad at exactly the near plane |
| Not claimed | Any performance property. There is no tiling, no SIMD, no threading, and no measurement. It is a correctness oracle, and the header says so |


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
