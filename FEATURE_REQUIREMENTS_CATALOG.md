# Feature Requirements Catalog

Normalized requirements with unique IDs, priority tier, rationale, dependencies,
and acceptance criteria, per Master Execution Prompt §5.3 and §7.

## Identifier scheme

`PN-<DOMAIN>-<NNN>`, where domain is one of:

| Code | Domain | §7 section |
|---|---|---|
| `PLT` | Platform, foundation, runtime core | 7.1 |
| `OBJ` | Object, entity, scene, gameplay architecture | 7.2 |
| `RND` | Rendering hardware interface and renderer | 7.3 |
| `WLD` | Large-world sandbox systems | 7.4 |
| `PHY` | Physics and simulation | 7.5 |
| `ANM` | Animation, deformation, cinematics | 7.6 |
| `AUD` | Audio | 7.7 |
| `AIN` | AI, crowds, navigation | 7.8 |
| `NET` | Networking, multiplayer, replay, persistence | 7.9 |
| `UIX` | UI, localization, accessibility | 7.10 |
| `EDT` | Integrated editor and creator workflow | 7.11 |
| `AST` | Asset pipeline and formats | 7.12 |
| `OPS` | Build, deployment, diagnostics, security, maintainability | 7.13 |

## Feature states

Exactly one state per requirement, per §4:

`NOT_STARTED` · `RESEARCHED` · `DESIGNED` · `IMPLEMENTED_UNVERIFIED` ·
`VERIFIED` · `BLOCKED` · `DEFERRED_BY_SCOPE`

`VERIFIED` requires executable behaviour **plus** the acceptance evidence assigned
to that requirement — never an interface, a stub, or a passing compile.
`DEFERRED_BY_SCOPE` is used only after the full inventory records why the item
sits outside the agreed release tier, and never to hide skipped work.

**Current status: 5 `VERIFIED`, 14 `IMPLEMENTED_UNVERIFIED`, 18 `BLOCKED`, 177
`NOT_STARTED`.** This catalog is the plan of record; the evidence behind each
non-`NOT_STARTED` state is in
[FEATURE_TRACEABILITY_MATRIX.md](FEATURE_TRACEABILITY_MATRIX.md). Where the two
disagree, the traceability matrix is authoritative, because it cites the
evidence.

## Tier meanings

`A` deterministic foundation · `B` playable sandbox production slice ·
`C` high-end production engine · `D` competitive next-generation suite.
Tiers are cumulative (§6).

---

## 7.1 Platform, foundation, and runtime core

| ID | Requirement | Tier | Depends on | Acceptance / test method | State |
|---|---|---|---|---|---|
| PN-PLT-001 | Application lifecycle and windowing, display modes, high-DPI | A | — | Windowed/fullscreen/borderless transitions, DPI change at runtime, no leak on shutdown | NOT_STARTED |
| PN-PLT-002 | Raw keyboard, mouse, controller input; mapping, rebinding, haptics, hot-plug | A | PN-PLT-001 | Synthetic input injection tests; device add/remove during play | NOT_STARTED |
| PN-PLT-003 | Monotonic time, fixed and variable tick, frame pacing, pause/step, time scaling | A | — | Fixed-step accumulator determinism test across variable frame times | IMPLEMENTED_UNVERIFIED |
| PN-PLT-004 | Deterministic seeded RNG with explicit streams | A | — | Same seed reproduces identical sequence across platforms and compilers | IMPLEMENTED_UNVERIFIED |
| PN-PLT-005 | Virtual memory reservation/commit abstraction | A | — | Reserve-then-commit growth test; commit failure handled without crash | IMPLEMENTED_UNVERIFIED |
| PN-PLT-006 | Tagged allocators, arenas, pools, alignment guarantees | A | PN-PLT-005 | Alignment assertions; arena reset; over-alignment types | VERIFIED |
| PN-PLT-007 | Memory tracking, leak detection, guard pages in diagnostic builds | A | PN-PLT-006 | Deliberate leak detected and attributed to allocation site | IMPLEMENTED_UNVERIFIED |
| PN-PLT-008 | Out-of-memory handling with defined recovery boundaries | A | PN-PLT-006 | Injected allocation failure; engine reports rather than faults | VERIFIED |
| PN-PLT-009 | Cache-aware containers authored in-project | A | PN-PLT-006 | Unit + property tests; bounds-checked in debug | VERIFIED |
| PN-PLT-010 | String, interned name, and handle systems | A | PN-PLT-009 | Interning collision and rehash tests; handle staleness detection | VERIFIED |
| PN-PLT-011 | Generational handles that detect stale access | A | PN-PLT-010 | Freed-then-reused slot yields a detectable stale handle, not an alias | VERIFIED |
| PN-PLT-012 | Thread abstraction, affinity, naming | A | — | Thread lifecycle test; names visible in diagnostics | IMPLEMENTED_UNVERIFIED |
| PN-PLT-013 | Work-stealing job graph with priorities and dependencies | A | PN-PLT-012 | Scales with core count; randomized scheduling stress finds no deadlock | IMPLEMENTED_UNVERIFIED |
| PN-PLT-014 | Synchronization primitives, cancellation, deadlock diagnostics | A | PN-PLT-013 | TSan clean; cancellation propagates without leaking jobs | IMPLEMENTED_UNVERIFIED |
| PN-PLT-015 | Asynchronous I/O with cancellation and backpressure | A | PN-PLT-013 | Concurrent read stress; cancel mid-flight leaves no dangling buffer | NOT_STARTED |
| PN-PLT-016 | Logging, structured events, assertions with categories and levels | A | — | Log capture in tests; assertion fires with source location | VERIFIED |
| PN-PLT-017 | Crash capture, minidump integration, error reporting, recovery boundary | A | PN-PLT-016 | Induced fault produces a usable dump on the target platform | BLOCKED (`BLOCK-001` for the Windows dump path) |
| PN-PLT-018 | Module boundaries, stable internal APIs, versioning, capability discovery | A | — | CI dependency-direction check fails on a violating include | IMPLEMENTED_UNVERIFIED |
| PN-PLT-019 | Configuration, console variables, command execution | A | PN-PLT-010 | CVar set/get/persist round-trip; command parse errors are actionable | VERIFIED |
| PN-PLT-020 | Deterministic serialization with schema migration | A | PN-PLT-009 | Golden-data test; old-version payload migrates and round-trips | VERIFIED |
| PN-PLT-021 | Object identity and reference repair across load | A | PN-PLT-020 | Cross-referencing objects survive save/load and rename | NOT_STARTED |
| PN-PLT-022 | Undo/redo transactions and snapshots | A | PN-PLT-020 | Randomized op/undo/redo sequence returns to the exact initial state | NOT_STARTED |
| PN-PLT-023 | Save-game foundations built on the serialization core | B | PN-PLT-021 | Save/reload across a world mutation preserves state | NOT_STARTED |
| PN-PLT-024 | In-project reflection and metadata generation | A | PN-PLT-010 | Reflected type round-trips through serializer and inspector without hand-written glue | NOT_STARTED |
| PN-PLT-025 | Reproducible builds, build manifests, version stamping | A | — | Two clean builds of one commit produce identical artefact hashes | NOT_STARTED |
| PN-PLT-026 | In-project unit test framework (no third-party framework permitted) | A | — | Framework runs its own self-tests; failure reporting verified by deliberate failures | VERIFIED |
| PN-PLT-027 | `Expected`-based fallible-return convention, exception-independent | A | — | Compiles and passes under GCC and Clang, and under `-fno-exceptions` | VERIFIED |

## 7.2 Object, entity, scene, and gameplay architecture

| ID | Requirement | Tier | Depends on | Acceptance / test method | State |
|---|---|---|---|---|---|
| PN-OBJ-001 | Data-oriented entity/component storage with stable handles | A | PN-PLT-011 | Add/remove/query under randomized churn; handles never silently alias | VERIFIED |
| PN-OBJ-002 | Archetype or equivalent query acceleration | A | PN-OBJ-001 | Query cost independent of total entity count for a fixed match set | NOT_STARTED |
| PN-OBJ-003 | Component lifecycle events | A | PN-OBJ-001 | Construction/destruction ordering test | NOT_STARTED |
| PN-OBJ-004 | Multithread-safe system scheduling from declared component access | A | PN-OBJ-002, PN-PLT-013 | Conflicting writers serialized; TSan clean under parallel systems | NOT_STARTED |
| PN-OBJ-005 | Hierarchical transforms with attachment | A | PN-OBJ-001 | Deep hierarchy update correctness; reparenting preserves world transform | NOT_STARTED |
| PN-OBJ-006 | Activation, tags, layers, spatial bounds | A | PN-OBJ-001 | Filtered queries return exactly the expected set | NOT_STARTED |
| PN-OBJ-007 | Scenes, subscenes, reusable entity templates, variants, overrides | B | PN-OBJ-005, PN-PLT-020 | Template edit propagates; per-instance override survives round-trip | NOT_STARTED |
| PN-OBJ-008 | Separation of editor state, authoring data, cooked data, runtime state, replicated state | A | PN-PLT-020 | Type-level separation enforced; CI check on illegal cross-references | NOT_STARTED |
| PN-OBJ-009 | Event/message system, gameplay tags, commands, timers | B | PN-OBJ-001 | Ordering and delivery guarantees under load | NOT_STARTED |
| PN-OBJ-010 | State machines, reusable abilities/effects, attributes | B | PN-OBJ-009 | Deterministic transition tests | NOT_STARTED |
| PN-OBJ-011 | Inventory, interaction, camera framework, possession/control | B | PN-OBJ-010 | Sample gameplay exercises each in the demonstration project | NOT_STARTED |
| PN-OBJ-012 | Custom text scripting language: parser, compiler, bytecode VM | B | PN-PLT-024 | Language conformance suite; fuzzed source never escapes the sandbox | NOT_STARTED |
| PN-OBJ-013 | Script debugger hooks, sandboxing, deterministic subset, hot reload | B | PN-OBJ-012 | Breakpoint/step tests; sandbox escape attempts rejected | NOT_STARTED |
| PN-OBJ-014 | Script reflection bridge to engine types | B | PN-OBJ-012, PN-PLT-024 | Round-trip calls in both directions with correct lifetimes | NOT_STARTED |
| PN-OBJ-015 | Node-based visual scripting compiling to the same IR as text scripts | C | PN-OBJ-012 | Identical IR and behaviour from equivalent text and graph programs | NOT_STARTED |
| PN-OBJ-016 | Visual script validation, breakpoints, search, comments, reroute, version migration, source mapping | C | PN-OBJ-015 | Graph authored in an older version opens and runs after migration | NOT_STARTED |
| PN-OBJ-017 | Native-code extension boundary that cannot corrupt the editor across ABI | C | PN-PLT-018 | Deliberately mismatched extension is rejected, not loaded | NOT_STARTED |

## 7.3 Rendering hardware interface and renderer

Requirements marked `BLOCKED` cannot be compiled or executed in the current
environment (`BLOCK-001`..`BLOCK-004`). They are blocked, never deferred.

[ADR-0009](docs/adr/ADR-0009-hybrid-rust-cpp.md) moved the first backend from
Direct3D 12 to Vulkan, so the graphics rows below cite `BLOCK-002` (no GPU)
rather than `BLOCK-001` (no Windows SDK). `BLOCK-001` still stands where the
requirement is genuinely Windows-specific.

| ID | Requirement | Tier | Depends on | Acceptance / test method | State |
|---|---|---|---|---|---|
| PN-RND-001 | Explicit graphics API abstraction: adapter selection, feature queries, queues | A | PN-PLT-018 | Enumerates adapters; capability query drives a fallback decision | BLOCKED (`BLOCK-002`) - type and command surface generated and compiling ([ADR-0010](docs/adr/ADR-0010-vulkan-bindings-from-the-registry.md)); no adapter has been enumerated because no GPU is present |
| PN-RND-002 | Command submission, resource creation, descriptor management | A | PN-RND-001 | Validation layers silent across a full frame | BLOCKED (`BLOCK-002`, `BLOCK-004` for validation layers) |
| PN-RND-003 | Synchronization and barriers as independent sync/access/layout axes | A | PN-RND-001 | Barrier derivation unit-tested on the reference backend without a GPU | IMPLEMENTED_UNVERIFIED |
| PN-RND-004 | Transient resources, upload/readback, device-loss recovery | A | PN-RND-002 | Induced device removal recovers rather than crashes | BLOCKED (`BLOCK-002`) |
| PN-RND-005 | Data-driven render graph with lifetime analysis and hazard validation | A | PN-RND-003 | Hazard cases detected by test; graph emits a pass/resource visualization | IMPLEMENTED_UNVERIFIED |
| PN-RND-006 | Transient memory aliasing in the render graph | A | PN-RND-005 | Total allocation strictly below the naive per-resource sum | NOT_STARTED |
| PN-RND-007 | Pass scheduling, async-compute eligibility, GPU markers | B | PN-RND-005 | Queue assignment respects declared dependencies | NOT_STARTED |
| PN-RND-008 | Forward+/deferred/hybrid path chosen by measured use cases | B | PN-RND-005 | Decision recorded in an ADR citing measurements, not preference | BLOCKED (`BLOCK-002`) |
| PN-RND-009 | Configurable render paths and quality tiers | B | PN-RND-008 | Every tier renders; fallbacks exercised in CI | NOT_STARTED |
| PN-RND-010 | Physically based materials and material instances | B | PN-RND-002 | White furnace test; energy conservation across the roughness range | NOT_STARTED |
| PN-RND-011 | Custom shader/material graph with codegen | C | PN-RND-010 | Graph and hand-written shader produce identical output | NOT_STARTED |
| PN-RND-012 | Shader permutations, offline and incremental compilation, PSO cache, hot reload | B | PN-RND-010 | Cache measurably cuts second-run startup; edit reloads without restart | BLOCKED (`BLOCK-003`) |
| PN-RND-013 | Bindless/descriptor-indexed resource addressing | A | PN-RND-002 | One descriptor set bound per frame; adding a texture rewrites no set | BLOCKED (`BLOCK-002`) |
| PN-RND-014 | GPU-driven visibility, indirect drawing, frustum and occlusion culling | C | PN-RND-013 | CPU frame time flat from 1k to 100k objects | BLOCKED (`BLOCK-002`) |
| PN-RND-015 | LOD selection and instance batching | C | PN-RND-014 | No visible popping; transitions measured | NOT_STARTED |
| PN-RND-016 | Capability-gated mesh/task shader path with raster fallback | C | PN-RND-014 | Both paths produce matching golden images | BLOCKED (`BLOCK-002`) |
| PN-RND-017 | Original cluster-based high-density geometry system with offline clustering, hierarchical bounds, streaming, GPU selection | D | PN-RND-016 | Deterministic clustering; documented deformation limits; no copied implementation | NOT_STARTED |
| PN-RND-018 | Texture streaming with residency feedback, mip prioritization, budgets, diagnostics overlay | C | PN-RND-002 | Residency stays inside a fixed budget during traversal | NOT_STARTED |
| PN-RND-019 | Tiled/sparse resource support where available | D | PN-RND-018 | Capability-gated; fallback verified | BLOCKED (`BLOCK-002`) |
| PN-RND-020 | Original virtual-texture system | D | PN-RND-018 | Page fault resolution latency measured under budget | NOT_STARTED |
| PN-RND-021 | HDR linear pipeline, exposure, tonemapping, colour management | B | PN-RND-010 | sRGB encode occurs exactly once; verified by pixel inspection | NOT_STARTED |
| PN-RND-022 | Wide-gamut output where supported | D | PN-RND-021 | Capability-gated; SDR fallback verified | BLOCKED (`BLOCK-002`) |
| PN-RND-023 | Anti-aliasing, temporal reconstruction, motion vectors, dynamic resolution | C | PN-RND-021 | Every moving vertex path writes correct velocity — enumerated and tested | NOT_STARTED |
| PN-RND-024 | Original spatial/temporal upscaling solution | D | PN-RND-023 | Quality and cost measured against native at matched settings | BLOCKED (`BLOCK-002`) |
| PN-RND-025 | Directional, point, spot, area-approximation, emissive, skylight, IBL | B | PN-RND-010 | Each light type validated against an analytic reference where one exists | NOT_STARTED |
| PN-RND-026 | Cascaded and/or virtualized shadows, contact shadows, translucency shadows | C | PN-RND-025 | Texel snapping verified — no crawl under camera motion; blended cascades | NOT_STARTED |
| PN-RND-027 | Original real-time GI and reflection solution | D | PN-RND-025 | Indirect light responds to a moving sun; no leaking through thin geometry | NOT_STARTED |
| PN-RND-028 | Hardware ray tracing with software fallback and acceleration-structure management | D | PN-RND-013 | Both paths produce matching images within tolerance | BLOCKED (`BLOCK-002`) |
| PN-RND-029 | In-project denoising | D | PN-RND-028 | Contact detail preserved; temporal stability measured | NOT_STARTED |
| PN-RND-030 | Optional path-traced reference mode | D | PN-RND-028 | Converges to a reference used as the golden-image ground truth | BLOCKED (`BLOCK-002`) |
| PN-RND-031 | Atmosphere, sky, sun/moon, volumetric clouds, height and local fog, volumetric lighting | C | PN-RND-021 | 24-hour cycle with no discontinuity at sunrise or sunset | NOT_STARTED |
| PN-RND-032 | Weather transitions and precipitation | C | PN-RND-031 | Transition is continuous; no popping between states | NOT_STARTED |
| PN-RND-033 | Post stack: bloom, DOF, motion blur, colour grading, lens effects | C | PN-RND-021 | Energy-conserving bloom; documented pass ordering | NOT_STARTED |
| PN-RND-034 | Debug visualization for every major pass | B | PN-RND-005 | Each view renders and is covered by a golden image | NOT_STARTED |
| PN-RND-035 | Material models: opaque, masked, transparent, refractive, subsurface, skin, hair, eye, cloth, decal, landscape, water, unlit | C | PN-RND-010 | Each has a golden-image test | NOT_STARTED |
| PN-RND-036 | GPU particles with CPU fallback: emitters, forces, collisions, ribbons, mesh particles, events, stages | C | PN-RND-013 | Deterministic seeding reproduces a frame exactly | NOT_STARTED |
| PN-RND-037 | Particle node editor | C | PN-RND-036, PN-EDT-002 | Authored effect round-trips and plays identically in the runtime | NOT_STARTED |
| PN-RND-038 | Water bodies, shore interaction, buoyancy hooks, reflection/refraction, underwater, waves | C | PN-RND-035 | Scalable tiers each validated; buoyancy agrees with the physics solver | NOT_STARTED |
| PN-RND-039 | Stereo/VR rendering architecture and OpenXR research | D | PN-RND-005 | Architecture documented; headset validation is hardware-blocked | BLOCKED (`BLOCK-002`) |

## 7.4 Large-world sandbox systems

| ID | Requirement | Tier | Depends on | Acceptance / test method | State |
|---|---|---|---|---|---|
| PN-WLD-001 | Double-precision or partition-relative world coordinates | A | PN-PLT-009 | No vertex jitter or shadow crawl at 500 km from origin | IMPLEMENTED_UNVERIFIED |
| PN-WLD-002 | Stable physics/render coordinate conversion | A | PN-WLD-001 | Round-trip conversion error bounded and asserted | IMPLEMENTED_UNVERIFIED |
| PN-WLD-003 | Multiplayer-safe spatial addressing | C | PN-WLD-001 | Two clients agree on cell identity for the same world point | NOT_STARTED |
| PN-WLD-004 | Editable world partition grid or hierarchical spatial index | B | PN-WLD-001 | Query correctness under randomized insert/remove | NOT_STARTED |
| PN-WLD-005 | Async cell streaming with prefetch, prioritization, cancellation | B | PN-WLD-004, PN-PLT-015 | 100 km traversal with no frame spike above budget | NOT_STARTED |
| PN-WLD-006 | Fixed streaming memory budgets and diagnostics | B | PN-WLD-005 | Pool never grows unbounded; eviction observable | NOT_STARTED |
| PN-WLD-007 | Data layers, runtime layers, world variants, subscenes | C | PN-WLD-004 | Layer toggling is transactional and reversible | NOT_STARTED |
| PN-WLD-008 | Cross-cell references with safe authoring transactions | C | PN-WLD-007, PN-PLT-021 | Reference across an unloaded cell resolves or fails explicitly | NOT_STARTED |
| PN-WLD-009 | Hierarchical LOD, proxy generation, impostors, cook-time optimization | C | PN-WLD-005 | HLOD transitions do not pop visibly | NOT_STARTED |
| PN-WLD-010 | Terrain sculpting, stamping, layers, splines, erosion/noise tools, holes | B | PN-WLD-004 | Edit → collision → navigation update in the same frame | NOT_STARTED |
| PN-WLD-011 | Terrain collision, navigation integration, material blending, runtime streaming | B | PN-WLD-010 | Collision matches visual surface within tolerance after an edit | NOT_STARTED |
| PN-WLD-012 | Foliage painting and procedural scattering with slope/height/biome filters | C | PN-WLD-010 | Deterministic placement from a seed; density honoured | NOT_STARTED |
| PN-WLD-013 | Foliage clustering, wind, interaction, seasonal state, harvesting hooks | C | PN-WLD-012 | Wind motion vectors correct — verified under TAA | NOT_STARTED |
| PN-WLD-014 | Biomes, ecosystems, spawn populations, resource distribution, significance tiers | C | PN-WLD-012 | Distance-tiered update rates observable and budgeted | NOT_STARTED |
| PN-WLD-015 | Road, river, trail, fence, utility, settlement spline tools with terrain conform | C | PN-WLD-010 | Intersection handling produces manifold geometry | NOT_STARTED |
| PN-WLD-016 | Deterministic procedural-content graph with seeds, filters, grammars, dependency tracking, baking, CLI generation | C | PN-WLD-014 | Same seed and graph reproduce byte-identical output | NOT_STARTED |
| PN-WLD-017 | Day/night, calendar, weather, temperature, wind, wetness, snow accumulation | C | PN-RND-031 | Environment parameters drive both material and gameplay state | NOT_STARTED |
| PN-WLD-018 | World-origin-safe navigation, audio, particles, AI, saves, networking, gizmos, debug draw | B | PN-WLD-001 | Each subsystem re-tested at 500 km from origin | NOT_STARTED |
| PN-WLD-019 | Sandbox persistence for changed, destroyed, and constructed objects, containers, vegetation, spawned actors, quests | B | PN-PLT-023 | Save at distance, reload, verify every mutation survived | NOT_STARTED |
| PN-WLD-020 | Persistence version migration | C | PN-WLD-019, PN-PLT-020 | Old save loads into a newer build with documented semantics | NOT_STARTED |

## 7.5 Physics and simulation

| ID | Requirement | Tier | Depends on | Acceptance / test method | State |
|---|---|---|---|---|---|
| PN-PHY-001 | Original broad phase | B | PN-OBJ-006 | Pair set matches a brute-force reference exactly | NOT_STARTED |
| PN-PHY-002 | Narrow phase, collision shapes, contact manifolds | B | PN-PHY-001 | Analytic cases verified; manifold stability under jitter | NOT_STARTED |
| PN-PHY-003 | Continuous collision detection | B | PN-PHY-002 | Fast body against thin wall does not tunnel | NOT_STARTED |
| PN-PHY-004 | Material friction and restitution | B | PN-PHY-002 | Ramp/bounce cases match closed-form prediction within tolerance | NOT_STARTED |
| PN-PHY-005 | Rigid-body integration, sleeping, islands | B | PN-PHY-002 | Box stack rests without vibration; islands sleep and wake correctly | NOT_STARTED |
| PN-PHY-006 | Queries, triggers, filtering | B | PN-PHY-001 | Raycast/sweep/overlap against a reference implementation | NOT_STARTED |
| PN-PHY-007 | Deterministic diagnostic replay | B | PN-PHY-005, PN-PLT-004 | Identical input reproduces an identical trajectory bit-for-bit | NOT_STARTED |
| PN-PHY-008 | Constraints and joints, ragdolls | B | PN-PHY-005 | Joint limits respected under high load; no explosion | NOT_STARTED |
| PN-PHY-009 | Character controller, moving platforms, kinematic bodies | B | PN-PHY-006 | Steps, slopes, and platforms behave per a written spec | NOT_STARTED |
| PN-PHY-010 | Compound shapes, heightfields, triangle meshes, collision cooking | B | PN-PHY-002 | Cooked collision is deterministic and matches source geometry | NOT_STARTED |
| PN-PHY-011 | Vehicle framework: suspension, tires, drivetrain, steering, braking, traction, surfaces, damage hooks | C | PN-PHY-008 | Drivable in the demonstration project; behaviour documented | NOT_STARTED |
| PN-PHY-012 | Destruction: fracture data, constraint breaking, damage propagation, debris budget, persistence | C | PN-PHY-008, PN-WLD-019 | Debris count stays within budget; destruction survives save/reload | NOT_STARTED |
| PN-PHY-013 | Cloth, rope, soft body with stability tests and scalable solver budgets | D | PN-PHY-008 | Solver remains stable at the documented iteration budget | NOT_STARTED |
| PN-PHY-014 | Buoyancy and water interaction | C | PN-RND-038 | Floating body equilibrium matches the analytic waterline | NOT_STARTED |
| PN-PHY-015 | Fluids, gated on a feasibility and performance review | D | PN-PHY-014 | Gate documented before any implementation begins | NOT_STARTED |
| PN-PHY-016 | Fixed-step policy with render interpolation/extrapolation | A | PN-PLT-003 | No visible stutter at non-multiple frame rates | IMPLEMENTED_UNVERIFIED |
| PN-PHY-017 | Physics debug drawing, determinism limits documented, replay capture, stress benchmarks | B | PN-PHY-007 | Limits stated explicitly rather than implied | NOT_STARTED |

## 7.6 Character animation, deformation, and cinematics

| ID | Requirement | Tier | Depends on | Acceptance / test method | State |
|---|---|---|---|---|---|
| PN-ANM-001 | Skeletons, skinning, blend shapes | B | PN-OBJ-005 | Skinned output matches a reference pose within tolerance | NOT_STARTED |
| PN-ANM-002 | Clips, compression, root motion, events/notifies | B | PN-ANM-001 | Compression error bounded and measured; notify timing exact | NOT_STARTED |
| PN-ANM-003 | Layered blending, masks, additive animation, sync groups | B | PN-ANM-002 | Blend weights verified analytically | NOT_STARTED |
| PN-ANM-004 | Animation graphs, state machines, blend spaces | B | PN-ANM-003 | Deterministic transitions under identical input | NOT_STARTED |
| PN-ANM-005 | Inertial transitions or an original equivalent | C | PN-ANM-004 | Transition is C1-continuous; no pose pop | NOT_STARTED |
| PN-ANM-006 | Pose caching, animation budgets, LOD, multithreaded evaluation | C | PN-ANM-004, PN-PLT-013 | Scales with cores; TSan clean | NOT_STARTED |
| PN-ANM-007 | FK/IK, aim and look constraints, foot placement, ground alignment, reach | B | PN-ANM-001 | No foot sliding on slopes or stairs — measured, not eyeballed | NOT_STARTED |
| PN-ANM-008 | Ragdoll blending and procedural secondary motion | C | PN-ANM-007, PN-PHY-008 | Animation→ragdoll transition has no discontinuity | NOT_STARTED |
| PN-ANM-009 | Retargeting across compatible skeletons with pose correction and scale handling | C | PN-ANM-001 | Batch retarget validated against source proportions | NOT_STARTED |
| PN-ANM-010 | Motion matching or an original data-driven pose selection with trajectory prediction | D | PN-ANM-004 | Deterministic fallback exists and is tested | NOT_STARTED |
| PN-ANM-011 | Facial rigging, curves, visemes, gaze, expressions, layered facial animation | D | PN-ANM-003 | Authored facial performance round-trips | NOT_STARTED |
| PN-ANM-012 | Lip-sync research without third-party models or services | D | PN-ANM-011 | Approach documented; no external service dependency | NOT_STARTED |
| PN-ANM-013 | Animation editor, curve editor, dope sheet, skeleton inspection, retarget tools, graph debugger, import diagnostics | C | PN-EDT-002 | An animator can author without editing engine source | NOT_STARTED |
| PN-ANM-014 | Camera rigs, rails, shakes, DOF focus, cuts, blends | C | PN-OBJ-011 | Deterministic playback from a seeded start | NOT_STARTED |
| PN-ANM-015 | Non-linear sequencer with shot, event, and audio tracks, keyframes, preview, render/export | D | PN-ANM-014 | Exported sequence is frame-exact and reproducible | NOT_STARTED |

## 7.7 Audio

| ID | Requirement | Tier | Depends on | Acceptance / test method | State |
|---|---|---|---|---|---|
| PN-AUD-001 | Custom low-latency mixer over OS audio-device APIs | B | PN-PLT-012 | No underruns under load; latency measured | NOT_STARTED |
| PN-AUD-002 | Voices, buses, sends, priorities, virtual voices, streaming, looping, seeking | B | PN-AUD-001 | Voice-stealing policy verified at the voice cap | NOT_STARTED |
| PN-AUD-003 | Pitch, time, fades, sample-rate conversion | B | PN-AUD-002 | SRC quality measured against an analytic signal | NOT_STARTED |
| PN-AUD-004 | 3D attenuation, panning, Doppler, listener management | B | PN-AUD-002, PN-WLD-018 | Positional accuracy verified at world-scale distances | NOT_STARTED |
| PN-AUD-005 | Obstruction, occlusion, reverb zones, environment sends, portals, spatial debugging | C | PN-AUD-004 | Occlusion transitions are continuous, not stepped | NOT_STARTED |
| PN-AUD-006 | Original DSP blocks: filters, EQ, compressor, limiter, delay, reverb, distortion, modulation, metering | C | PN-AUD-001 | Each block validated against its analytic transfer function | NOT_STARTED |
| PN-AUD-007 | HRTF/spatial audio using legally usable data or a documented original dataset strategy | D | PN-AUD-004 | Data provenance recorded in the provenance ledger before use | NOT_STARTED |
| PN-AUD-008 | Interactive music graph: layers, transitions, tempo/bar sync, stingers, states | C | PN-AUD-002 | Transitions land on the correct musical boundary | NOT_STARTED |
| PN-AUD-009 | Audio profiling, clipping protection, device-change recovery, accessibility hooks | B | PN-AUD-001 | Device removed mid-playback recovers without a crash | NOT_STARTED |

## 7.8 AI, crowds, and navigation

| ID | Requirement | Tier | Depends on | Acceptance / test method | State |
|---|---|---|---|---|---|
| PN-AIN-001 | Custom navigation mesh or hierarchical navigation generation | B | PN-WLD-011 | Generated mesh covers walkable surface; validated against ground truth | NOT_STARTED |
| PN-AIN-002 | Tiled rebuilds and navigation streaming | B | PN-AIN-001 | Terrain edit triggers a bounded, correct partial rebuild | NOT_STARTED |
| PN-AIN-003 | Pathfinding, dynamic obstacles, links, areas and costs, queries, debugging | B | PN-AIN-001 | Paths match a reference search; costs respected | NOT_STARTED |
| PN-AIN-004 | Agent steering, avoidance, local movement, formations, locomotion requests | B | PN-AIN-003, PN-ANM-004 | No agent interpenetration under crowd stress | NOT_STARTED |
| PN-AIN-005 | Behavior trees and/or hierarchical state machines with a visual editor | C | PN-OBJ-015 | Deterministic tests over authored trees | NOT_STARTED |
| PN-AIN-006 | Blackboard/state data, services, decorators, conditions, debugging | C | PN-AIN-005 | Decision trace reproducible from recorded input | NOT_STARTED |
| PN-AIN-007 | Spatial/environment queries, perception, sight, hearing, memory, threat, cover, tactical positions | C | PN-AIN-003 | Sense configuration produces the predicted detection set | NOT_STARTED |
| PN-AIN-008 | Goal-oriented planning, utility scoring, smart objects, schedules, faction/reputation | D | PN-AIN-006 | Planner reaches the goal or reports failure explicitly | NOT_STARTED |
| PN-AIN-009 | Crowd significance, pooled agents, simulation LOD, background simulation, world-partition integration, save/load, network relevance | C | PN-AIN-004, PN-WLD-014 | Distance tiers observable; population survives save/reload | NOT_STARTED |
| PN-AIN-010 | AI recording and replay that exposes decisions without fabricating natural-language explanations | C | PN-AIN-006 | Replay shows actual decision inputs, not a generated narrative | NOT_STARTED |

## 7.9 Networking, multiplayer, replay, and persistence

| ID | Requirement | Tier | Depends on | Acceptance / test method | State |
|---|---|---|---|---|---|
| PN-NET-001 | Custom transport over OS sockets with protocol framing and connection lifecycle | C | PN-PLT-015 | Framing fuzzed; malformed input rejected without crash or leak | NOT_STARTED |
| PN-NET-002 | Encryption strategy | C | PN-NET-001 | Threat model documented; no hand-rolled primitive without review | NOT_STARTED |
| PN-NET-003 | Congestion and backpressure policy, packet statistics, DoS boundaries | C | PN-NET-001 | Behaviour measured under bandwidth caps and loss | NOT_STARTED |
| PN-NET-004 | Authoritative server and dedicated server target | C | PN-NET-001 | Server runs headless without editor or renderer | NOT_STARTED |
| PN-NET-005 | Replication schema, delta encoding, snapshots, RPC/event model | C | PN-NET-004, PN-PLT-024 | Delta correctness verified by full-state comparison | NOT_STARTED |
| PN-NET-006 | Relevance and interest management, dormancy, prioritization, bandwidth budgets, large-world addressing | C | PN-NET-005, PN-WLD-003 | Bandwidth stays within budget at target player count | NOT_STARTED |
| PN-NET-007 | Client prediction, server reconciliation, interpolation, lag compensation, input buffering, clock sync | C | PN-NET-005 | Correction magnitude measured under defined latency profiles | NOT_STARTED |
| PN-NET-008 | Rollback where suitable, with the suitability criteria documented | D | PN-NET-007 | Criteria written before implementation, not after | NOT_STARTED |
| PN-NET-009 | Networked physics and vehicles with explicit authority and correction behaviour | D | PN-NET-007, PN-PHY-011 | Authority model documented; corrections bounded | NOT_STARTED |
| PN-NET-010 | Session/lobby abstraction operable locally without a proprietary online service | C | PN-NET-004 | Full local multiplayer session with no external service | NOT_STARTED |
| PN-NET-011 | Join-in-progress, disconnect/reconnect, version and content compatibility, save ownership | C | PN-NET-010 | Version mismatch refused with an actionable message | NOT_STARTED |
| PN-NET-012 | Host migration research and anti-cheat trust boundaries | D | PN-NET-010 | Trust boundary documented; no client-authoritative gameplay state | NOT_STARTED |
| PN-NET-013 | Deterministic or snapshot replay with seeking, bookmarks, spectator cameras, debug overlays | C | PN-NET-005, PN-PHY-007 | Replay reproduces the recorded session | NOT_STARTED |
| PN-NET-014 | Parser and protocol fuzzing; soak under latency, jitter, reordering, duplication, loss, caps, malicious input | C | PN-NET-001 | Fuzz corpus runs clean under ASan/UBSan | NOT_STARTED |

## 7.10 UI, localization, and accessibility

| ID | Requirement | Tier | Depends on | Acceptance / test method | State |
|---|---|---|---|---|---|
| PN-UIX-001 | Custom retained/immediate hybrid UI runtime and editor toolkit | B | PN-RND-002 | Layout engine unit-tested against expected geometry | NOT_STARTED |
| PN-UIX-002 | Layout, docking, focus, navigation, clipping, styles, animation, data binding, virtualization | B | PN-UIX-001 | Virtualized list of 100k rows stays within frame budget | NOT_STARTED |
| PN-UIX-003 | Accessibility semantics exposed by the UI runtime | C | PN-UIX-001 | Semantic tree inspectable and correct for sample screens | NOT_STARTED |
| PN-UIX-004 | In-world and screen-space UI, anchors, safe areas, DPI scaling, resolution independence | B | PN-UIX-002 | Golden images across several resolutions and DPI settings | NOT_STARTED |
| PN-UIX-005 | Controller, keyboard, and mouse navigation | B | PN-UIX-002, PN-PLT-002 | Every interactive element reachable by each input method | NOT_STARTED |
| PN-UIX-006 | Custom font/text pipeline: shaping strategy, fallback, glyph atlas, line breaking | B | PN-UIX-001 | Rendered text compared against reference metrics | NOT_STARTED |
| PN-UIX-007 | Bidirectional text research | D | PN-UIX-006 | Approach and current limits documented honestly | NOT_STARTED |
| PN-UIX-008 | Localization tables, plural/gender handling, pseudo-localization, string validation, culture-aware formatting | C | PN-UIX-006 | Pseudo-loc pass finds unlocalized strings and overflow | NOT_STARTED |
| PN-UIX-009 | Subtitle system and dialogue-caption metadata | C | PN-UIX-008, PN-AUD-002 | Captions synchronized to audio within a stated tolerance | NOT_STARTED |
| PN-UIX-010 | Colour-vision modes, remapping, hold/toggle, motion reduction, subtitle controls, readable scaling, contrast checks, audio cues | C | PN-UIX-004 | Contrast measured programmatically; checklist executed | NOT_STARTED |
| PN-UIX-011 | Screen-reader feasibility assessment and accessibility test checklist | D | PN-UIX-003 | Assessment recorded; unsupported cases stated, not hidden | NOT_STARTED |

## 7.11 Integrated editor and creator workflow

| ID | Requirement | Tier | Depends on | Acceptance / test method | State |
|---|---|---|---|---|---|
| PN-EDT-001 | Project browser, creation, recent projects, recovery, version display, safe upgrade | B | PN-AST-001 | Project created from a template opens and runs | NOT_STARTED |
| PN-EDT-002 | Dockable windows, menus, commands, shortcuts, layouts, themes, high-DPI, command search, crash-safe layout recovery | B | PN-UIX-002 | Layout survives an induced crash | NOT_STARTED |
| PN-EDT-003 | 3D viewport: selection, marquee, gizmos, snapping, coordinate spaces, pivot modes, camera navigation, view modes, isolation, focus, overlays, stats | B | PN-EDT-002, PN-RND-034 | Picking accuracy verified against a known scene | NOT_STARTED |
| PN-EDT-004 | Scene hierarchy/outliner, inspector, multi-edit, search, component add/remove, overrides, diffing, copy/paste, undo/redo, validation | B | PN-PLT-024, PN-PLT-022 | Randomized edit/undo/redo returns to the exact initial state | NOT_STARTED |
| PN-EDT-005 | Content browser, asset registry, thumbnails, tags, collections, dependencies, rename/move repair, duplicate detection, source tracking, bulk actions | B | PN-AST-002 | Rename repairs every referrer; verified by dependency scan | NOT_STARTED |
| PN-EDT-006 | Specialized editors: terrain, foliage, procedural, material, shader, particle, animation, audio, navigation, behavior, visual scripting, UI, cinematic, world partition | C | PN-EDT-004 | Each authors real data consumed unchanged by the runtime | NOT_STARTED |
| PN-EDT-007 | Play in editor, simulate, pause, single-step, eject/possess, multiplayer instances, standalone launch, device profiles, live property editing with explicit persistence rules | B | PN-EDT-003 | Live edit persistence rule is explicit and tested both ways | NOT_STARTED |
| PN-EDT-008 | Integrated log, console, debugger, call stack, memory view, CPU/GPU profiler, task graph, render graph, network profiler, asset audit, streaming view, capture comparison | C | PN-OPS-006 | Each view shows real captured data, never placeholder | NOT_STARTED |
| PN-EDT-009 | Autosave, recovery, transaction journaling, corrupted-asset isolation, external file change handling, source-control abstraction | B | PN-PLT-022 | Induced corruption isolates one asset without losing the project | NOT_STARTED |
| PN-EDT-010 | Headless CLI tools: import, validate, cook, package, procedural generation, tests, benchmarks, server | B | PN-AST-003 | Every tool runs without a display or an editor process | NOT_STARTED |

## 7.12 Asset pipeline and formats

| ID | Requirement | Tier | Depends on | Acceptance / test method | State |
|---|---|---|---|---|---|
| PN-AST-001 | Source/cooked asset separation with stable 128-bit GUIDs | A | PN-PLT-020 | GUID stable across rename and move; never regenerated | NOT_STARTED |
| PN-AST-002 | Content-addressed cache, dependency graph, incremental rebuild, parallel cooking, invalidation | A | PN-AST-001, PN-PLT-013 | Touching one source rebuilds exactly its dependents, no more | NOT_STARTED |
| PN-AST-003 | Deterministic cooked output and schema versioning | A | PN-AST-002 | Two cooks of one input produce byte-identical output | NOT_STARTED |
| PN-AST-004 | Original container formats with documented schemas, bounds checking, checksums, endian/version policy, migration, partial loading, corruption handling | A | PN-AST-003 | Fuzzed container never reads out of bounds; corruption reported | NOT_STARTED |
| PN-AST-005 | glTF 2.0 import implemented from the public specification | B | PN-AST-004 | Import validated against the specification's own sample models | NOT_STARTED |
| PN-AST-006 | Independently implemented image and audio decoders for the supported formats | B | PN-AST-004 | Decoded output compared against reference values; fuzzed | NOT_STARTED |
| PN-AST-007 | Mesh processing: topology validation, normals/tangents, vertex/index optimization, LOD generation, cluster building, collision generation, skin validation, compression | B | PN-AST-005 | Deterministic output; error bounds measured | NOT_STARTED |
| PN-AST-008 | Texture processing: colour-space metadata, mips, normal handling, channel packing, platform compression, streaming tiles, preview | B | PN-AST-006 | Colour space correct end to end; verified by pixel inspection | NOT_STARTED |
| PN-AST-009 | Animation compression, audio conversion and streaming preparation | B | PN-AST-007 | Compression error bounded and measured | NOT_STARTED |
| PN-AST-010 | Shader compilation and cache | B | PN-RND-012 | Cache hit measurably cuts build and startup time | BLOCKED (`BLOCK-003`) |
| PN-AST-011 | Terrain tiles, navigation tiles, procedural bake products | C | PN-WLD-016 | Bakes are reproducible from seed and inputs | NOT_STARTED |
| PN-AST-012 | DCC export plug-ins written from scratch against documented DCC extension APIs | D | PN-AST-005 | No DCC implementation code ships in the engine | NOT_STARTED |
| PN-AST-013 | Actionable diagnostics for every failed import or cook | A | PN-AST-002 | Every failure path emits an actionable message — tested, not assumed | NOT_STARTED |

## 7.13 Build, deployment, diagnostics, security, maintainability

| ID | Requirement | Tier | Depends on | Acceptance / test method | State |
|---|---|---|---|---|---|
| PN-OPS-001 | Editor, game, server, development, test, profile, shipping configurations | A | — | Each configuration builds and runs in CI | IMPLEMENTED_UNVERIFIED |
| PN-OPS-002 | Incremental and clean builds, generated-code tracking, toolchain lockfile, build cache policy | A | PN-OPS-001 | Clean build from a fresh checkout succeeds | IMPLEMENTED_UNVERIFIED |
| PN-OPS-003 | Warning discipline, static analysis, sanitizers | A | PN-OPS-001 | Warnings-as-errors; ASan/UBSan/TSan configurations green | IMPLEMENTED_UNVERIFIED |
| PN-OPS-004 | Reproducibility checks | A | PN-PLT-025 | Two clean builds of one commit produce identical hashes | NOT_STARTED |
| PN-OPS-005 | Cook, stage, package, patch, manifest, version, uninstall for the first target | B | PN-AST-003 | Packaged build runs without the source asset tree present | BLOCKED (`BLOCK-001`) |
| PN-OPS-006 | CPU/GPU/memory/IO/network budgets, capture format, baselines, regression thresholds, hardware metadata | B | PN-OPS-003 | Every capture carries full hardware and settings context | NOT_STARTED |
| PN-OPS-007 | Unit, property, integration, golden-image, scene, performance, soak, fuzz, network-chaos, recovery, end-to-end tests | A | PN-PLT-026 | Each category present and running in CI | IMPLEMENTED_UNVERIFIED - unit, integration and golden-image present; performance, soak, fuzz, network-chaos and recovery absent |
| PN-OPS-008 | Secure parsing, bounds checks, integer-overflow handling, untrusted-asset policy, script sandbox, network validation, path-traversal prevention, decompression limits, secrets policy | A | PN-AST-004 | Each control has a negative test that fails without it | NOT_STARTED |
| PN-OPS-009 | Dependency and provenance audit, threat model, secure defaults | A | — | Audit runs and its result recorded, including failures | NOT_STARTED |
| PN-OPS-010 | Crash recovery, backup/migration tests, data-loss testing | B | PN-EDT-009 | Induced crash mid-save never destroys the prior good file | NOT_STARTED |
| PN-OPS-011 | API reference, architecture guide, subsystem guides, tutorials, samples, troubleshooting, contribution rules, code style, release notes | B | — | An unfamiliar developer completes the tutorial unaided | NOT_STARTED |
| PN-OPS-012 | Signed-release readiness without pretending a certificate exists | D | PN-OPS-005 | Signing step marked `USER_ACTION_REQUIRED`, never simulated | BLOCKED (`BLOCK-005`) |

---

## Summary

| Domain | Requirements |
|---|---|
| 7.1 Platform / foundation | 27 |
| 7.2 Object / entity / gameplay | 17 |
| 7.3 Rendering | 39 |
| 7.4 Large world | 20 |
| 7.5 Physics | 17 |
| 7.6 Animation / cinematics | 15 |
| 7.7 Audio | 9 |
| 7.8 AI / navigation | 10 |
| 7.9 Networking | 14 |
| 7.10 UI / localization / accessibility | 11 |
| 7.11 Editor | 10 |
| 7.12 Asset pipeline | 13 |
| 7.13 Build / ops / security | 12 |
| **Total** | **214** |

Requirements currently `BLOCKED` by the environment: 18 — two by `BLOCK-001`
(no Windows SDK), thirteen by `BLOCK-002` (no GPU), two by `BLOCK-003` (no
shader compiler), one by `BLOCK-004` (no validation layers), one by `BLOCK-005`
(no signing certificate). One row cites two blockers, so the citations sum to
nineteen. Every one names the blocker that holds it. None is marked
`DEFERRED_BY_SCOPE`; nothing here has been scoped away.
