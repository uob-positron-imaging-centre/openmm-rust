/**
 * File   : build.rs
 * License: GNU v3.0
 * Author : Andrei Leonard Nicusan <a.l.nicusan@bham.ac.uk>
 * Date   : 13.08.2020
 */

use std::env;
use std::io::{self, ErrorKind};
use std::fs;
use std::path::Path;
use std::process::Command;

#[allow(non_snake_case)]
struct BuildOptions {
    OPENMM_BUILD_SHARED_LIB: Option<bool>,
    OPENMM_BUILD_STATIC_LIB: Option<bool>,

    OPENMM_BUILD_CPU_LIB: Option<bool>,
    OPENMM_BUILD_OPENCL_LIB: Option<bool>,
    OPENMM_BUILD_CUDA_LIB: Option<bool>,

    OPENMM_BUILD_AMOEBA_PLUGIN: Option<bool>,
    OPENMM_BUILD_DRUDE_PLUGIN: Option<bool>,
    OPENMM_BUILD_PME_PLUGIN: Option<bool>,
    OPENMM_BUILD_RPMD_PLUGIN: Option<bool>,

    OPENMM_BUILD_EXAMPLES: Option<bool>,
    OPENMM_GENERATE_API_DOCS: Option<bool>,

    OPENMM_BUILD_C_AND_FORTRAN_WRAPPERS: Option<bool>,
    OPENMM_BUILD_PYTHON_WRAPPERS: Option<bool>,         // Always false due to crates.io size limit
}

impl BuildOptions {
    fn minimal() -> BuildOptions {
        BuildOptions {
            OPENMM_BUILD_SHARED_LIB: Some(true),

            OPENMM_BUILD_AMOEBA_PLUGIN: Some(false),
            OPENMM_BUILD_DRUDE_PLUGIN: Some(false),
            OPENMM_BUILD_PME_PLUGIN: Some(false),
            OPENMM_BUILD_RPMD_PLUGIN: Some(false),

            OPENMM_BUILD_EXAMPLES: Some(false),
            OPENMM_GENERATE_API_DOCS: Some(false),

            OPENMM_BUILD_C_AND_FORTRAN_WRAPPERS: Some(true),
            OPENMM_BUILD_PYTHON_WRAPPERS: Some(false),

            ..BuildOptions::all_unset()
        }
    }

    fn all_false() -> BuildOptions {
        BuildOptions {
            OPENMM_BUILD_SHARED_LIB: Some(false),
            OPENMM_BUILD_STATIC_LIB: Some(false),

            OPENMM_BUILD_CPU_LIB: Some(false),
            OPENMM_BUILD_OPENCL_LIB: Some(false),
            OPENMM_BUILD_CUDA_LIB: Some(false),

            OPENMM_BUILD_AMOEBA_PLUGIN: Some(false),
            OPENMM_BUILD_DRUDE_PLUGIN: Some(false),
            OPENMM_BUILD_PME_PLUGIN: Some(false),
            OPENMM_BUILD_RPMD_PLUGIN: Some(false),

            OPENMM_BUILD_EXAMPLES: Some(false),
            OPENMM_GENERATE_API_DOCS: Some(false),

            OPENMM_BUILD_C_AND_FORTRAN_WRAPPERS: Some(false),
            OPENMM_BUILD_PYTHON_WRAPPERS: Some(false),
        }
    }

    fn all_unset() -> BuildOptions {
        BuildOptions {
            OPENMM_BUILD_SHARED_LIB: None,
            OPENMM_BUILD_STATIC_LIB: None,

            OPENMM_BUILD_CPU_LIB: None,
            OPENMM_BUILD_OPENCL_LIB: None,
            OPENMM_BUILD_CUDA_LIB: None,

            OPENMM_BUILD_AMOEBA_PLUGIN: None,
            OPENMM_BUILD_DRUDE_PLUGIN: None,
            OPENMM_BUILD_PME_PLUGIN: None,
            OPENMM_BUILD_RPMD_PLUGIN: None,

            OPENMM_BUILD_EXAMPLES: None,
            OPENMM_GENERATE_API_DOCS: None,

            OPENMM_BUILD_C_AND_FORTRAN_WRAPPERS: None,
            OPENMM_BUILD_PYTHON_WRAPPERS: None,
        }
    }
}

fn bool_to_cmake(option: bool) -> &'static str {
    if option { "ON" } else { "OFF" }
}

fn add_build_options(cmaker: &mut cmake::Config, build_options: BuildOptions) {
    if let Some(opt) = build_options.OPENMM_BUILD_SHARED_LIB {
        cmaker.define("OPENMM_BUILD_SHARED_LIB", bool_to_cmake(opt));
    }
    if let Some(opt) = build_options.OPENMM_BUILD_STATIC_LIB {
        cmaker.define("OPENMM_BUILD_STATIC_LIB", bool_to_cmake(opt));
    }

    if let Some(opt) = build_options.OPENMM_BUILD_CPU_LIB {
        cmaker.define("OPENMM_BUILD_CPU_LIB", bool_to_cmake(opt));
    }
    if let Some(opt) = build_options.OPENMM_BUILD_OPENCL_LIB {
        cmaker.define("OPENMM_BUILD_OPENCL_LIB", bool_to_cmake(opt));
    }
    if let Some(opt) = build_options.OPENMM_BUILD_CUDA_LIB {
        cmaker.define("OPENMM_BUILD_CUDA_LIB", bool_to_cmake(opt));
    }

    if let Some(opt) = build_options.OPENMM_BUILD_AMOEBA_PLUGIN {
        cmaker.define("OPENMM_BUILD_AMOEBA_PLUGIN", bool_to_cmake(opt));
    }
    if let Some(opt) = build_options.OPENMM_BUILD_DRUDE_PLUGIN {
        cmaker.define("OPENMM_BUILD_DRUDE_PLUGIN", bool_to_cmake(opt));
    }
    if let Some(opt) = build_options.OPENMM_BUILD_PME_PLUGIN {
        cmaker.define("OPENMM_BUILD_PME_PLUGIN", bool_to_cmake(opt));
    }
    if let Some(opt) = build_options.OPENMM_BUILD_RPMD_PLUGIN {
        cmaker.define("OPENMM_BUILD_RPMD_PLUGIN", bool_to_cmake(opt));
    }

    if let Some(opt) = build_options.OPENMM_BUILD_EXAMPLES {
        cmaker.define("OPENMM_BUILD_EXAMPLES", bool_to_cmake(opt));
    }
    if let Some(opt) = build_options.OPENMM_GENERATE_API_DOCS {
        cmaker.define("OPENMM_GENERATE_API_DOCS", bool_to_cmake(opt));
    }

    if let Some(opt) = build_options.OPENMM_BUILD_C_AND_FORTRAN_WRAPPERS {
        cmaker.define("OPENMM_BUILD_C_AND_FORTRAN_WRAPPERS", bool_to_cmake(opt));
    }
    if let Some(opt) = build_options.OPENMM_BUILD_PYTHON_WRAPPERS {
        cmaker.define("OPENMM_BUILD_PYTHON_WRAPPERS", bool_to_cmake(opt));
    }
}

fn configure_cmake(cmaker: &mut cmake::Config) {
    // Overall (base) config
    let mut build_options = if env::var("CARGO_FEATURE_NO_DEFAULT").is_ok() {
        BuildOptions::all_false()
    } else if env::var("CARGO_FEATURE_MINIMAL").is_ok() {
        BuildOptions::minimal()
    } else {
        BuildOptions::all_unset()
    };

    // Build static / dynamic libraries
    if env::var("CARGO_FEATURE_SHARED_LIB").is_ok() {
        build_options.OPENMM_BUILD_SHARED_LIB = Some(true);
    }
    if env::var("CARGO_FEATURE_STATIC_LIB").is_ok() {
        build_options.OPENMM_BUILD_STATIC_LIB = Some(true);
    }

    // Which platforms to build for
    if env::var("CARGO_FEATURE_CPU").is_ok() {
        build_options.OPENMM_BUILD_CPU_LIB = Some(true);
    }
    if env::var("CARGO_FEATURE_OPENCL").is_ok() {
        build_options.OPENMM_BUILD_OPENCL_LIB = Some(true);
    }
    if env::var("CARGO_FEATURE_CUDA").is_ok() {
        build_options.OPENMM_BUILD_CUDA_LIB = Some(true);
    }

    // Which plugins to build
    if env::var("CARGO_FEATURE_AMOEBA").is_ok() {
        build_options.OPENMM_BUILD_AMOEBA_PLUGIN = Some(true);
    }
    if env::var("CARGO_FEATURE_DRUDE").is_ok() {
        build_options.OPENMM_BUILD_DRUDE_PLUGIN = Some(true);
    }
    if env::var("CARGO_FEATURE_PME").is_ok() {
        build_options.OPENMM_BUILD_PME_PLUGIN = Some(true);
    }
    if env::var("CARGO_FEATURE_RPMD").is_ok() {
        build_options.OPENMM_BUILD_RPMD_PLUGIN = Some(true);
    }

    // Build examples and Doxygen API
    if env::var("CARGO_FEATURE_EXAMPLES").is_ok() {
        build_options.OPENMM_BUILD_EXAMPLES = Some(true);
    }
    if env::var("CARGO_FEATURE_GENERATE_API_DOCS").is_ok() {
        build_options.OPENMM_GENERATE_API_DOCS = Some(true);
    }

    // Build C / Fortran and Python wrappers
    if env::var("CARGO_FEATURE_C_AND_FORTRAN_WRAPPERS").is_ok() {
        build_options.OPENMM_BUILD_C_AND_FORTRAN_WRAPPERS = Some(true);
    }

    // Don't build python wrappers as they are 50 MB in size, way over crates.io's crate source
    // size limit of 10 MB.
    build_options.OPENMM_BUILD_PYTHON_WRAPPERS = Some(false);

    add_build_options(cmaker, build_options);
}

fn link_cxx_stdlib() {
    // If OpenMM is used as a static library, we need to explicitly link the C++ stdlib. On Clang
    // toolchains, it is called "c++", while on GCC toolchains it is "stdc++".
    let target = env::var("TARGET").unwrap();
    let static_lib = env::var("CARGO_FEATURE_STATIC_LIB").is_ok();

    if static_lib && !target.contains("msvc") {
        match env::var("CXXSTDLIB") {
            Ok(stdlib) => {
                println!("cargo:warning=CXXSTDLIB is set to `{}`, overriding the C++ standard \
                    library name for the system linker.", &stdlib);

                println!("cargo:rustc-link-lib={}", stdlib);
            },
            _ => {
                let stdlib = if target.contains("apple") {
                    "c++"
                } else if target.contains("freebsd") {
                    "c++"
                } else if target.contains("openbsd") {
                    "c++"
                } else {
                    "stdc++"
                };

                println!("cargo:rustc-link-lib={}", stdlib);
            },
        }
    }
}

fn copy_openmm_libs(openmm_home: &str) -> io::Result<String> {
    // Copy necessary OpenMM libraries (dynamic and static) from "$OPENMM_HOME/lib" to Rust's
    // output directory ($OUT_DIR/lib). Show extra information for most usual errors.
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_lib = Path::new(&out_dir).join("lib");

    // Create directory "$OUT_DIR/lib". Only propagate error if it is not ErrorKind::AlreadyExists
    if let Err(err) = fs::create_dir(&out_lib) {
        match err.kind() {
            ErrorKind::AlreadyExists => (),
            _ => panic!("Error creating directory `lib` in Cargo's `$OUT_DIR`: {}. Do you have \
                write access to your current directory?", err),
        }
    }

    // Will copy OpenMM libraries from "$OPENMM_HOME/lib"
    let openmm_lib = Path::new(openmm_home).join("lib");

    // Check whether "$OPENMM_HOME/lib" exists
    if let Err(err) = fs::metadata(&openmm_lib) {
        panic!("Filepath `$OPENMM_HOME/lib` (`{}`) does not exist: {}.",
            openmm_lib.display(), err);
    }

    for file in openmm_lib.read_dir()? {
        let f = file?;
        let fname = f.file_name().into_string().expect(&format!(
            "Could not convert file name `{:?}` to UTF-8.", &f
        ));

        if fname.to_uppercase().contains("OPENMM") {
            fs::copy(f.path(), out_lib.join(f.file_name()))?;
        }
    }

    Ok(out_dir)
}

fn main() {
    let static_lib = env::var("CARGO_FEATURE_STATIC_LIB").is_ok();

    // Default to shared (dynamic) library. If a static library is requested, link against it
    // instead of the dylib.
    let lib_name = if static_lib {
        println!("cargo:warning=[OpenMM-sys] Statically linking against the OpenMM library.");
        "OpenMM_static"
    } else {
        println!("cargo:warning=[OpenMM-sys] Dynamically linking against the OpenMM library.");
        "OpenMM"
    };

    let lib_type = if static_lib { "static" } else { "dylib" };

    // If the OPENMM_HOME environment variable is set, copy the OpenMM libraries from
    // "$OPENMM_HOME/lib" to "$OUT_DIR/lib" and link against them.
    if let Ok(openmm_home) = env::var("OPENMM_HOME") {
        let openmm_path = match copy_openmm_libs(&openmm_home) {
            Ok(out_dir) => out_dir,
            Err(err) => panic!("Error accessing / copying OpenMM libraries from \
                `$OPENMM_HOME/lib` to `$CARGO_OUT_DIR/lib`: {}", err),
        };

        let lib_path = Path::new(&openmm_path).join("lib");

        println!("cargo:rustc-link-search={}", lib_path.display());
        println!("cargo:rustc-link-lib={}={}", lib_type, lib_name);

        link_cxx_stdlib();

        println!("cargo:home={}", openmm_path);
        println!("cargo:warning=[OpenMM-sys] The OPENMM_HOME environment variable is set to \
            `{}`. Searching for the library there. Note: OpenMM is not built if OPENMM_HOME is \
            set.", openmm_home);

        println!("cargo:rerun-if-changed=build.rs");
        println!("cargo:rerun-if-env-changed=OPENMM_HOME");
        return;
    }

    // Check if the openmm git submodule is initialised. Otherwise download the OpenMM source code
    // from the https://github.com/anicusan/openmm-mirrors.git repository, branch `7.4.2`
    if !Path::new("openmm/.git").exists() {
        let _ = Command::new("git")
            .args(&["submodule", "update", "--init", "openmm"])
            .status();
    }

    // Build OpenMM using CMake, setting the relevant options
    let mut cmaker = cmake::Config::new("openmm");
    cmaker.profile("Release");
    configure_cmake(&mut cmaker);

    // Pass -D options to cmake (e.g. OPENMM_BUILD_CUDA_LIB) from the "OPENMM_CMAKE_OPTIONS"
    // environment variable, if defined
    if let Ok(cmake_options) = env::var("OPENMM_CMAKE_OPTIONS") {
        for option in cmake_options.split_whitespace() {
            assert!(
                option.contains("="),
                "\n[OpenMM-sys] Error: The OpenMM CMake options defined in the \
                `OPENMM_CMAKE_OPTIONS` environment variable must be space-separated `key=value` \
                pairs (e.g. 'OPENMM_BUILD_CUDA_LIB=ON PYTHON_EXECUTABLE=usr/bin/python'). \
                Found ill-defined option '{}'.",
                option
            );
            let mut key_value = option.split("=");
            cmaker.define(key_value.next().unwrap(), key_value.next().unwrap());
        }
    }

    let install_path = cmaker.build();
    let lib_path = install_path.join("lib");

    // Link against the built OpenMM library and the C++ standard library if needed.
    println!("cargo:rustc-link-search={}", lib_path.display());
    println!("cargo:rustc-link-lib={}={}", lib_type, lib_name);

    link_cxx_stdlib();

    println!("cargo:home={}", install_path.display());
    println!("cargo:warning=[OpenMM-sys] The OpenMM library was built and installed in `{}`. \
        You can copy / move that directory somewhere else and set the `OPENMM_HOME` environment \
        variable to point to its path. This way, the OpenMM library will no longer require \
        building.", install_path.display());

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=OPENMM_HOME");
}
