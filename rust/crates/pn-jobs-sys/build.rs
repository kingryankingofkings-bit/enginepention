// Pention Engine - pn-jobs-sys/build.rs
// Requirement: PN-OBJ-017
// Decision:    ADR-0009
//
// Locates the C++ static libraries that CMake built and tells cargo how to link
// them. CMake is authoritative: it builds the native side first and passes the
// output directory in PN_NATIVE_LIB_DIR. Running cargo directly without that
// variable set is a configuration error and says so, rather than failing later
// with an unresolved symbol.

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-env-changed=PN_NATIVE_LIB_DIR");

    let Ok(lib_dir) = env::var("PN_NATIVE_LIB_DIR") else {
        panic!(
            "PN_NATIVE_LIB_DIR is not set.\n\
             The Rust crates link against C++ static libraries that CMake builds, so cargo \
             must be invoked through the CMake build rather than directly:\n\
             \n    cmake --build <build-dir>\n\
             \n\
             See BUILD.md and ADR-0009."
        );
    };

    let root = PathBuf::from(&lib_dir);
    for subdirectory in ["engine/jobs", "engine/platform", "engine/core"] {
        println!("cargo:rustc-link-search=native={}", root.join(subdirectory).display());
    }

    // Static archives do not carry their dependencies, so every layer is listed
    // explicitly and in dependency order: a symbol needed by pn_jobs must be
    // resolvable by an archive appearing after it.
    for library in ["pn_jobs", "pn_platform", "pn_core"] {
        println!("cargo:rustc-link-lib=static={library}");
        println!("cargo:rerun-if-changed={}", root.join(format!("lib{library}.a")).display());
    }

    // The C++ standard library and threads, which those archives need.
    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=dylib=stdc++");
        println!("cargo:rustc-link-lib=dylib=pthread");
    } else if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=dylib=c++");
    }
}
