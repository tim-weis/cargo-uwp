[![crates.io](https://img.shields.io/crates/v/cargo-uwp.svg)](https://crates.io/crates/cargo-uwp)
![platform-support](https://img.shields.io/badge/platform-windows--only-critical?logo=windows)

# `cargo uwp` <!-- omit in toc -->

A custom Cargo command to create, manage, and package UWP applications.

- [Introduction](#introduction)
- [Getting started](#getting-started)
  - [Installation](#installation)
  - [First project](#first-project)
  - [Launching the application](#launching-the-application)
- [Debugging](#debugging)
- [What next](#what-next)
- [Future work](#future-work)

## Introduction

Windows developers can choose to write against several different application models. Targeting the Universal Windows Platform (UWP) poses the unique challenge of requiring a lot of transformations and infrastructure artifacts to turn source code into an executable or deployable application package.

`cargo uwp` aims to make this process less tedious, reducing opportunities for mistakes, and catching errors early. It can generate a Cargo package suitably configured for the UWP, alongside a Cargo [build script](https://doc.rust-lang.org/cargo/reference/build-scripts.html) that manages build artifacts required for packaging and deployment.

The generated starter package depends on the [windows-rs](https://crates.io/crates/windows) crate as a Windows Runtime projection for Rust. This is not a hard requirement at this time, but that is likely to change [over time](#future-work).

## Getting started

### Installation

With this being a custom Cargo command, it's reasonable to assume that Cargo is already installed and accessible. Let's move ahead and run

```none
cargo install cargo-uwp
```

from a command prompt to download, compile, and install the binary. To make sure that `cargo uwp` successfully installed, run the following

```none
cargo uwp --version
```

This should print out the version of the installed binary.

### First project

With everything set up, it's time to generate a new UWP Cargo package. Following Cargo's lead the subcommand to do so is unsurprisingly named `new`. The following

```none
cargo uwp new uwp-rs
```

will set up a new Cargo package called *uwp-rs*. It invokes `cargo new` underneath, and makes some modifications to get the package UWP-ready. You'd be tempted to `cd` into the directory and `cargo build` straight away. Previously, this failed due to missing metadata. Starting with version 0.2.0 the build will succeed out-of-the-box. Instead of failing, the build script will now issue warnings when it encounters default values in the `[package.metadata.appxmanifest]` table.

Those warnings would need to be addressed prior to packaging or deploying an application, but for local testing things can remain as is.

```none
cargo build
```

will now produce a binary called *uwp-rs.exe* into *target\\x86_64-uwp-windows-msvc\\debug*, ready to be launched.

### Launching the application

Naturally, you'd wish to head right in and do that, just to be presented with an error dialog. Like anything UWP, launching an application is neither simple nor obvious. To do that, the application needs to be registered first.

At this time, there's still a manual step required here, namely copying the *Assets* folder to the target directory (*target\\x86_64-uwp-windows-msvc\\debug*). Open a command prompt and navigate to the output directory and do the following:

```none
mkdir Assets
copy ..\..\..\Assets\*.* Assets\
```

With the command prompt still open, everything is now in place for the grand finale:

```none
powershell -command "Add-AppxPackage -Register AppxManifest.xml"
```

If that all went well, you should now see your UWP application in the Start menu, ready to be launched. This time for real.

> *Hint: Should you ever lose track of where your UWP application went, just open a command prompt and type*
> 
> ```none
> %windir%\explorer.exe shell:appsFolder
> ```
> 
> *to find it, and pin it to your Start menu again.*

## Debugging

Debugging a Rust application from Visual Studio Code has never been much of a fun experience. You might be pleasantly surprised to find out, that Visual Studio does a far better job, even without any Rust support.

To [debug an installed UWP app package](https://docs.microsoft.com/en-us/visualstudio/debugger/debug-installed-app-package) launch Visual Studio (2017 or later), open the *Debug* menu, expand the *Other Debug Targets* item, and select *Debug Installed App Package...*. Once you've found your new application, you can hit *Start*, and off you go.

With the application launched go to *File*, *Open*, and selected *File...*. Navigate to the source corresponding to the application, and load up *main.rs*. You can now set breakpoints, e.g. on the `button.Click` handler, single-step through the code, inspect local variables. And memory.

Variable display is still brutally close to how the linker left the code, given that there no visualizers akin to [.natvis](https://docs.microsoft.com/en-us/visualstudio/debugger/create-custom-views-of-native-objects) available. Maybe someone with experience can see whether this situation can be improved.

## What next

Getting all the way here was quite a bit of work. Surely, you haven't gone through this for giggles. After all, you will want to share your work, and package your UWP application for deployment.

First, though, you will have to go back and provide meaningful values in the `[package.metadata.appxmanifest]` table. Going forward with the default values is either going to fail, or have unintended consequences when deploying the application. With that out of the way, there's nothing keeping you from [packaging, bundling, and .appxupload](docs/appx/Packaging.md)-ing your UWP application to the Store. Sadly, none of that [has found](#future-work) its way into `cargo uwp`. This isn't quite over yet.

## Future work

It's still early days, and a lot of features and tool support are still missing. In its current state, `cargo uwp` can be used to produce artifacts ready to be packaged, signed, bundled, and deployed through Microsoft's Store. Future work will address these shortcomings, namely

* Support easier registration for testing
* Streamline .appx package generation
* Allow for .appxbundle bundling
* Implement .appxsym support for better diagnostics
* Package signing and self-signed certificate generation
* ... and more

If you have problems using this tool, suggestions, or feature requests, make sure to [file an issue](https://github.com/tim-weis/cargo-uwp/issues).
