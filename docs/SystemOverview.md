# Table of contents <!-- omit in toc -->

- [Introduction](#introduction)
- [Cargo package creation](#cargo-package-creation)
  - [Default package contents](#default-package-contents)
- [Build process](#build-process)
- [Customization](#customization)

## Introduction

Broadly speaking, `cargo uwp` consists of two components:

* A custom Cargo command (*cargo-uwp.exe*)
* A Cargo build script (*.build/main.rs*)

## Cargo package creation

After running `cargo uwp new <path>` a new Cargo package is set up. It contains well known items as well as several new ones. The generated package is a fair bit more complex than a default Cargo-generated package. The directory tree looks like this:

```none
📂<path>
├ 📂.build
│ ├ 📄 main.rs
│ └ 📄 shared.rs
├ 📂.cargo
│ └ 📄 config.toml
├ 📂Assets
│ ├ 📄 SplashScreen.scale-200.png
│ ├ 📄 Square150x150Logo.scale-200.png
│ ├ 📄 Square44x44Logo.scale-200.png
│ └ 📄 StoreLogo.png
├ 📂bindings
│ ├ 📂src
│ │ └ 📄 lib.rs
│ ├ 📄 build.rs
│ └ 📄 Cargo.toml
├ 📂src
│ └ 📄 main.rs
├ 📂templates
│ ├ 📄 AppxManifest.xml
│ └ 📄 FileMapping.ini
├ 📄 Cargo.toml
└ 📄 rust-toolchain.toml
```

Some of those files can be changed by clients to customize the build process or artifacts contained inside AppX packages, while others are fixed. The following explains what the individual files are and how they fit into the overall build system.

### Default package contents

Core to the build configuration are the files *rust-toolchain.toml* and *.cargo/config.toml*. At this point, the build system requires use of the nightly channel. With the *\*-uwp-windows-*\* toolchains in [Tier 3](https://doc.rust-lang.org/stable/rustc/platform-support.html#tier-3) support, there is no Standard Library immediately available, and it needs to be built instead.

For that to work without additional manual work, the *rust-toolchain.toml* file specifies that the nightly channel be used, and designates the compononets required (`rust-src`).

The remainder of the nightly toolchain configuration is encoded inside *.cargo/config.toml*, which describes the unstable features required (`std`), and how the Standard Library should be built (`panic_abort`) \[**TODO:** Research, why `panic_abort` appears to be required.\]

## Build process

The build process itself is fairly straight forward. Invoking `cargo build` (with or without an explicit `--target` argument) first executes a custom build script (*.build/main.rs*), followed by the standard Cargo build. The build script is responsible for generating artifacts required for packaging, currently an *AppxManifest.xml* (describing the application package) and *FileMapping.ini* file (naming the assets contained in the final application package).

## Customization

Several aspects of the build process are customizable:

* \[TODO\]

Client code can change the `channel` from `nightly` to a more specific release. This is useful in pinning the compiler to a particular nightly channel release, if necessary.
