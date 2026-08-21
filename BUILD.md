# Building Pention

## Prerequisites

| Tool | Minimum | Notes |
|---|---|---|
| CMake | 3.24 | Build orchestration only; not an engine component |
| Ninja | 1.11 | Or any CMake generator |
| GCC | 13 | Must support `-std=c++23` |
| Clang | 18 | Second supported compiler; both must build every commit |
| Python | 3.11 | Repository tooling only. No Python is invoked by the engine at any point |
| Rust | 1.75 | Second implementation language ([ADR-0009](docs/adr/ADR-0009-hybrid-rust-cpp.md)). Optional: CMake reports `rust crates : OFF` and skips them if cargo is absent |

For the sanitizer configurations under Clang you also need Clang's sanitizer
runtime archives, packaged separately on most distributions:

```sh
sudo apt-get install -y libclang-rt-18-dev
```

Without it the Clang sanitizer builds fail at **link** time, not compile time.
Worth knowing, because the error names missing `.a` files and reads like a
project misconfiguration rather than a missing package.

## Build and test

```sh
cmake -S . -B build -G Ninja -DCMAKE_BUILD_TYPE=Debug
cmake --build build
ctest --test-dir build --output-on-failure
```

## Options

| Option | Default | Purpose |
|---|---|---|
| `PN_BUILD_TESTS` | `ON` | Build and register the test suite |
| `PN_WARNINGS_AS_ERRORS` | `ON` | Warnings are errors. Leave on |
| `PN_NO_EXCEPTIONS` | `OFF` | Compile with `-fno-exceptions`, proving ADR-0002's independence claim |
| `PN_SANITIZE_ADDRESS` | `OFF` | AddressSanitizer plus UndefinedBehaviorSanitizer |
| `PN_SANITIZE_THREAD` | `OFF` | ThreadSanitizer. Mutually exclusive with the above |

## The configurations CI runs

Every commit is built in all ten. A compiler-specific failure is a build break,
not a warning - the two supported compilers have already disagreed once, over
`std::expected` availability, which is why both build from the first commit
rather than one being added later.

```sh
for compiler in g++ clang++; do
  for build_type in Debug Release; do
    cmake -S . -B "build-$compiler-$build_type" -G Ninja \
      -DCMAKE_BUILD_TYPE="$build_type" -DCMAKE_CXX_COMPILER="$compiler"
    cmake --build "build-$compiler-$build_type"
    ctest --test-dir "build-$compiler-$build_type" --output-on-failure
  done
done
```

Plus, under each compiler: `-DPN_NO_EXCEPTIONS=ON`, `-DPN_SANITIZE_ADDRESS=ON`,
and `-DPN_SANITIZE_THREAD=ON`.

## The Rust half

`cargo` is driven by CMake, never run directly. The crates link against static
libraries CMake produces, and `build.rs` reads their location from
`PN_NATIVE_LIB_DIR`; running `cargo` by hand fails with an explanation rather
than an unresolved symbol.

The workspace declares **zero** external dependencies, so cargo runs `--offline`
and never touches the network. That is load-bearing rather than a convenience:
an attempted download is itself a boundary violation and should fail the build.

Rust is skipped automatically in the four sanitizer configurations. Instrumented
C++ archives cannot be linked safely into an uninstrumented Rust binary, and
stable Rust has no matching sanitizer support. The C++ side keeps full coverage;
the FFI boundary under a sanitizer is a stated gap.

## Regenerating the Vulkan bindings

`rust/crates/pn-vulkan-sys/src/generated.rs` is committed, so an ordinary build
needs neither the registry nor a network. Regenerate it only when the pin
changes or the generator does:

```sh
python3 build_scripts/fetch_vulkan_registry.py
cargo run -p vkgen --manifest-path rust/Cargo.toml -- \
    third_party/vk.xml --emit rust/crates/pn-vulkan-sys/src/generated.rs
```

`vk.xml` downloads to `third_party/`, which is git-ignored: it is another
party's authored file and is deliberately not vendored. What is committed is
`build_scripts/vulkan_registry_pin.txt` - the URL, the SHA-256, and the header
version - which the fetcher reads and `vkgen` compiles in, so the two cannot
disagree about what is pinned. A hash mismatch is a hard failure; adopting a new
registry means editing the pin and reviewing the regenerated diff.

Running `vkgen` without `--emit` prints what it parsed and exits, which is a
quick way to see what a registry bump changed.

## Repository checks

These enforce rules that erode instantly if left to convention. Both run in CI
and both are proven to fail on deliberate violations - see
`docs/evidence/enforcement-checks.txt`.

```sh
python3 build_scripts/check_authorship.py            # every source file names its requirement and ADR
python3 build_scripts/check_layering.py              # dependency direction; graphics API confined to one module
python3 build_scripts/check_no_crate_dependencies.py # no crates.io, git, or registry dependencies
python3 build_scripts/check_generated_bindings.py    # committed Vulkan bindings match the pinned registry
```

The last one re-fetches `vk.xml`, regenerates, and diffs. It needs network
access; unlike the other three it fails rather than passes when it cannot reach
the registry, because an unverifiable check is not a passing one.

## Running a subset of tests

Each test binary takes `--list` and `--filter=<substring>`:

```sh
./build/engine/core/pn_test_core_handle --list
./build/engine/math/pn_test_math_precision --filter=500km
```

## What does not build here

The Direct3D 12 backend, the Vulkan backend, and anything requiring a shader
compiler cannot be built on a Linux host without the Windows SDK, a GPU, or
`dxc`. Those are tracked as `BLOCK-001` through `BLOCK-004` in
[docs/ENVIRONMENT_BASELINE.md](docs/ENVIRONMENT_BASELINE.md). None of that code
exists yet.
