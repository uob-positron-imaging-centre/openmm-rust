# OpenMM-sys
This crate provides [`bindgen`](https://docs.rs/bindgen/)-generated declarations for the C API wrapper of [OpenMM](http://openmm.org/), optionally building static / dynamic libraries from its source code. The build step is highly configurable, exposing the most important OpenMM CMake options to the library consumer through Cargo features and / or environment variables.

OpenMM is a toolkit for molecular simulation. It can be used either as a stand-alone application for running simulations, or as a library you call from your own code. It provides a combination of extreme flexibility (through custom forces and integrators), openness, and high performance (especially on recent GPUs) that make it truly unique among simulation codes.

## Requirements
This crate needs the following programs installed and available on your system:

- C / C++ compiler (the platform defaults work best: MSVC on Windows, GCC on Linux and Clang on macOS)
- CMake

## Installation

The simplest installation can be achieved by simply adding the following to `Cargo.toml`:

```toml
[dependencies]
openmm-sys = "7.4"
```

That's it! The installation is fully self-contained. However, building the OpenMM libraries can be a bit time-consuming - on the author's machine (mid 2012 MacBook Pro) it takes 3-5 minutes. To see the CMake build steps as it runs, use the `-vv` (very verbose) when building using Cargo:

```
cargo build -vv
```

The good news is you can copy / move the installation folder (printed during compilation) somewhere else and set the `OPENMM_HOME` environment variable to point to its location. Take this terminal output as an example:

```
$> cargo build -vv

<Elided CMake output>

warning: [OpenMM-sys] Dynamically linking against the OpenMM library.
warning: [OpenMM-sys] The OpenMM library was built and installed in `/Users/andreinicusan/openmm-rust/openmm-sys/target/debug/build/openmm-sys-0e32f8f9e5febd89/out`. You can copy / move that directory somewhere else and set the `OPENMM_HOME` environment variable to point to its path. This way, the OpenMM library will no longer require building.

```

Copy the file from the path shown and set `OPENMM_HOME`:

```
$> mkdir ~/openmm

$> cp -r /Users/andreinicusan/openmm-rust/openmm-sys/target/debug/build/openmm-sys-0e32f8f9e5febd89/out ~/openmm

$> export OPENMM_HOME=/Users/andreinicusan/openmm-rust/openmm-sys/target/debug/build/openmm-sys-0e32f8f9e5febd89/out
```

Future Rust crates will use the OpenMM installation from `OPENMM_HOME` instead of compiling its source code - improving compilation times tremendously!

The directory tree pointed at by `OPENMM_HOME` should look something like this:

```
$OPENMM_HOME
├── bin
├── build
├── docs
├── examples
├── include
├── lib
│   ├── libOpenMM.dylib
│   ├── libOpenMMAmoeba.dylib
│   ├── libOpenMMAmoeba_static.a
│   ├── libOpenMMDrude.dylib
│   ├── libOpenMMDrude_static.a
│   ├── libOpenMMRPMD.dylib
│   ├── libOpenMMRPMD_static.a
│   ├── libOpenMM_static.a
│   └── plugins
└── licenses
```

The actual OpenMM libraries reside in `OPENMM_HOME/lib`.

### Alternative Conda-based Installation
OpenMM also provides pre-built binaries for different systems that are easy to install using [`conda`](https://docs.conda.io/en/latest/) (packaged with the recommended [Anaconda Python distribution](https://docs.continuum.io/anaconda/install/)):

```
$> conda install -c omnia -c conda-forge openmm
```

The downside of this approach is that you now need to locate where the OpenMM libraries were installed, so you can set `OPENMM_HOME`. For example, on the author's machine, running `which python` produces:

```
$> which python
/usr/local/anaconda3/envs/openmm_env/bin/python
```

Popping the last two components from that path should yield a location which contains `lib` and the required OpenMM libraries:

```
export OPENMM_HOME=/usr/local/anaconda3/envs/openmm_env/
```

### Basic Configuration
By default, OpenMM's CMake installer will check which platforms are available and build all plugins for those. For example, if CUDA is available on your system and the AMOEBA plugin is going to be built (`OPENMM_BUILD_AMOEBA_PLUGIN = "ON"` in CMake), the CUDA implementation is also included by default (`OPENMM_BUILD_AMOEBA_CUDA_LIB = "ON"`). The most important features available can be cherry-picked from this crate, as shown below.

Besides setting the `OPENMM_HOME` environment variable, there are a few other settings that are easy to configure in `Cargo.toml`:

```toml
[dependencies.openmm-sys]
features = ["static-lib", "cuda"]
```

In this case, OpenMM is built and used as a [static library](https://en.wikipedia.org/wiki/Static_library), including the CUDA platform. Here is the complete list of available features:

- Library type:
  - `shared-lib` - build shared (dynamic) OpenMM libraries.
  - `static-lib` - build static OpenMM libraries and link against them. If both `shared_lib` and `static_lib` are checked, they are built together, but your crate will link against the static library.

- Configuration "bundles":
  - `no-default` - turn off all OpenMM CMake options. You can use this for cherry-picking individual features that you want compiled.
  - `minimal` - does not build any plugins, examples, API docs or Python wrappers. 

- Platforms:
  - `cpu` - build optimized CPU platform.
  - `opencl` - build the OpenMMOpenCL library.
  - `cuda` - build OpenMMCuda library for Nvidia GPUs.

- Plugins:
  - `amoeba` - build Amoeba plugin.
  - `drude` - build Drude plugin.
  - `pme` - build PME plugin.
  - `rpmd` - build RPMD plugin.

- Miscellaneous:
  - `examples` - build included OpenMM examples.
  - `generate-api-docs` - generate Doxygen-based documentation.

- Wrappers:
  - `c-and-fortran-wrappers` - build C and Fortran wrappers. Essential if you want to use this crate, as it uses the C API.

Note: we cannot build the Python wrapper in this crate as packaging the required OpenMM source code exceeds crates.io's limit of 10 MB per crate. If you need the Python wrapper, please use another installation path (`conda`, compiling from source, etc.).

### Advanced Configuration
The build step can be further configured using some environment variables:

- `OPENMM_CMAKE_OPTIONS` - this can contain *space-separated* `key=value` pairs that will be passed down to `CMake` using the `-D` flag (e.g. `OPENMM_CMAKE_OPTIONS` = `OPENMM_BUILD_CUDA_LIB=ON PYTHON_EXECUTABLE=usr/bin/python`). You can run `cmake -LA <openmm_source_dir>` to see the full list of available options.
- `CXXSTDLIB` - sometimes the location of the C++ standard library shared-lib needs to be explicitly given to the linker. For reasons beyond me, the name varies between compilers (e.g. "libc++" for Clang and "libstdc++" for GCC). Set this environment variable to override the default platform-dependent option.

Finally, you can compile OpenMM yourself using CMake. Even more information (and perhaps better explained than me) is provided by OpenMM's authors on their [website](http://docs.openmm.org/latest/userguide/library.html#compiling-openmm-from-source-code) - see the "Compiling OpenMM from Source Code" section.

## Examples
Take a look in the `examples/` folder for some low-level use of OpenMM from Rust. Note that it uses OpenMM's C Wrapper, so all functions calls must be `unsafe` - see the `openmm-rust` crate for a safe, higher-level API.

## Versioning
The first two digits of this crate's version correspond to the latest [minor version](https://semver.org/) of `OpenMM`, while the last digit corresponds to patches to this crate. For example, `openmm-sys` version 7.4.0 corresponds to `OpenMM` 7.4.2 (the latest at the time of writing).

If you are interested in having Rust wrappers for a different `OpenMM` version, by all means contact me at *a.l.nicusan \<at\> bham.ac.uk*.

## Bindgen Usage
The shell script used to generate the Rust bindings to OpenMM's C Wrapper is included in `src/bindings`, along with the wrapper itself.

## License
This crate is MIT-licensed. See the `openmm/docs-source/licenses` directory for OpenMM's licenses.

## Acknowledgement
All credit goes to Dr. Peter Eastman, OpenMM's authors, the Stanford Pande lab and Stanford University for this amazing piece of software. If you used OpenMM in any of your work please cite the following publication:

> P. Eastman, J. Swails, J. D. Chodera, R. T. McGibbon, Y. Zhao, K. A. Beauchamp, L.-P. Wang, A. C. Simmonett, M. P. Harrigan, C. D. Stern, R. P. Wiewiora, B. R. Brooks, and V. S. Pande. “OpenMM 7: Rapid development of high performance algorithms for molecular dynamics.” PLOS Comp. Biol. 13(7): e1005659. (2017)

