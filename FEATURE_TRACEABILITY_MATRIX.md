# Feature Traceability Matrix

Maps each requirement that has left `NOT_STARTED` to its implementation, its
tests, and the evidence supporting its current state.

**A requirement is `VERIFIED` only when executable behaviour plus its assigned
acceptance evidence both exist.** An interface, a stub, a passing compile, or a
design document is not evidence. Every other requirement in
[FEATURE_REQUIREMENTS_CATALOG.md](FEATURE_REQUIREMENTS_CATALOG.md) - 208 of the
214 - remains `NOT_STARTED` or `BLOCKED` and is absent from this file, which is
the intended reading: this table is short because the project is young.

## Test totals, as counted from the built binaries

62 registered tests across 7 binaries, executing 274 assertions, all passing in
each of 10 build configurations. Counted by running each binary rather than by
reading the source, because a test that fails to register is invisible to a
source count.

| Binary | Tests | Checks |
|---|---|---|
| `pn_test_testing_self` | 9 | 26 |
| `pn_test_core_expected` | 17 | 51 |
| `pn_test_core_handle` | 10 | 105 |
| `pn_test_math_vector` | 7 | 18 |
| `pn_test_math_matrix` | 7 | 32 |
| `pn_test_math_convention` | 6 | 13 |
| `pn_test_math_precision` | 6 | 29 |
| **Total** | **62** | **274** |

## Verified

| ID | Requirement | Implementation | Tests | Evidence |
|---|---|---|---|---|
| PN-PLT-026 | In-project unit test framework | `engine/testing/` | `engine/testing/tests/self_test.cpp` (9 tests) | Framework self-tests assert that failing checks are recorded, that a failed `PN_REQUIRE` aborts the body while a failed `PN_CHECK` does not, and that a genuine failure exits non-zero. `docs/evidence/build-matrix.txt` |
| PN-PLT-027 | `Expected`-based fallible-return convention, exception-independent | `engine/core/include/pn/core/{error,expected}.hpp` | `engine/core/tests/expected_test.cpp` (17 tests) | Passes under GCC and Clang, Debug and Release, **and with `-fno-exceptions`** under both compilers, and under ASan+UBSan and TSan. `static_assert` pins trivial destructibility for trivial payloads. `docs/evidence/build-matrix.txt` |
| PN-PLT-011 | Generational handles that detect stale access | `engine/core/include/pn/core/handle.hpp` | `engine/core/tests/handle_test.cpp` (10 tests) | A recycled slot is proven to invalidate the prior handle rather than alias it; double-free is reported rather than ignored; never-issued, out-of-range, freed, and stale are distinguished by error category |

## Partially verified

| ID | Requirement | State | What is done | What is not |
|---|---|---|---|---|
| PN-WLD-001 | Double-precision world coordinates | `IMPLEMENTED_UNVERIFIED` | `f64` world space and camera-relative `f32` render space exist as **distinct types**, so mixing them is a compile error. Precision at 500 km is asserted, and a negative-control test proves the naive `f32` approach fails there. `engine/math/tests/precision_test.cpp` | The acceptance criterion is stated in terms of *rendered* output - no vertex jitter or shadow crawl. That requires a renderer and a GPU (`BLOCK-002`). The arithmetic is verified; the rendering claim is not |
| PN-WLD-002 | Stable physics/render coordinate conversion | `IMPLEMENTED_UNVERIFIED` | Round-trip conversion is bounded and asserted; the view matrix is proven to carry no translation. `engine/math/tests/matrix_test.cpp` | No physics engine exists yet, so the physics half of the contract is untested |
| PN-OPS-003 | Warning discipline, static analysis, sanitizers | `IMPLEMENTED_UNVERIFIED` | Warnings-as-errors across a broad set under both compilers; ASan+UBSan and TSan configurations exist and pass. `docs/evidence/build-matrix.txt` | Static analysis (clang-tidy or equivalent) is not yet configured |
| PN-PLT-018 | Module boundaries and dependency direction | `IMPLEMENTED_UNVERIFIED` | `build_scripts/check_layering.py` enforces the dependency graph and the single-graphics-API rule, and is **proven to fail** on deliberate violations. `docs/evidence/enforcement-checks.txt` | Only three modules exist, so the graph is barely exercised. The rule matters most at modules that do not yet exist |
| PN-OPS-001 | Build configurations | `IMPLEMENTED_UNVERIFIED` | Debug and Release under both compilers, plus no-exceptions and two sanitizer configurations - 10 in total, all green | The editor, server, profile, and shipping configurations do not exist because the code they would configure does not exist |
| PN-OPS-007 | Test categories | `IMPLEMENTED_UNVERIFIED` | Unit tests exist and run in CI | Property, integration, golden-image, scene, performance, soak, fuzz, network-chaos, recovery, and end-to-end categories do not exist yet |

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

## Blocked

The 18 requirements blocked by this environment are listed with their blockers
in [FEATURE_REQUIREMENTS_CATALOG.md](FEATURE_REQUIREMENTS_CATALOG.md) and
[docs/ENVIRONMENT_BASELINE.md](docs/ENVIRONMENT_BASELINE.md) section 3. None has
been reclassified as `DEFERRED_BY_SCOPE`.
