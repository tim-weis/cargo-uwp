//! Contains constants and data used by this tool to generate the initial cargo package.

/// Version of the *window-rs* crate to be used. This version is used in the
/// `[dependencies]` table of the cargo package, as well as in the *bindings* crate.
///
pub(crate) const WINDOWS_RS_VERSION: &str = "0.18.0";

/// Rust toolchain related assets
pub(crate) const RUST_TOOLCHAIN_TOML_FILENAME: &str = "rust-toolchain.toml";
pub(crate) const RUST_TOOLCHAIN_TOML: &[u8] = include_bytes!("../data/rust-toolchain.toml");

/// Cargo default configuration
pub(crate) const CARGO_CONFIG_DIR: &str = ".cargo";
pub(crate) const CARGO_CONFIG_TOML_FILENAME: &str = "config.toml";
pub(crate) const CARGO_CONFIG_TOML: &[u8] = include_bytes!("../data/.cargo/config.toml");

/// Assets
///
pub(crate) const ASSETS_DIR: &str = "Assets";

pub(crate) const STORE_LOGO_PNG: &[u8] = include_bytes!("../data/Assets/StoreLogo.png");
pub(crate) const STORE_LOGO_PNG_FILENAME: &str = "StoreLogo.png";

pub(crate) const SPLASH_SCREEN_PNG: &[u8] =
    include_bytes!("../data/Assets/SplashScreen.scale-200.png");
pub(crate) const SPLASH_SCREEN_PNG_FILENAME: &str = "SplashScreen.scale-200.png";

pub(crate) const SQUARE_44_LOGO_PNG: &[u8] =
    include_bytes!("../data/Assets/Square44x44Logo.scale-200.png");
pub(crate) const SQUARE_44_LOGO_PNG_FILENAME: &str = "Square44x44Logo.scale-200.png";

pub(crate) const SQUARE_150_LOGO_PNG: &[u8] =
    include_bytes!("../data/Assets/Square150x150Logo.scale-200.png");
pub(crate) const SQUARE_150_LOGO_PNG_FILENAME: &str = "Square150x150Logo.scale-200.png";

/// Templates used to feed the build system. These templates are picked up by the root
/// *build.rs* build script, adjusted as needed, and then copied to `OUT_DIR`.
///
/// Clients can choose to make modifications to these templates. This is somewhat of an
/// advanced topic, and not generally required (with the exception of managing assets
/// for later packaging though the *FileMapping.ini* file).
///
pub(crate) const TEMPLATES_DIR: &str = "templates";

pub(crate) const APPX_MANIFEST_TEMPLATE: &[u8] =
    include_bytes!("../data/templates/AppxManifest.xml");
pub(crate) const APPX_MANIFEST_TEMPLATE_FILENAME: &str = "AppxManifest.xml";

pub(crate) const FILE_MAPPINGS_TEMPLATE: &[u8] =
    include_bytes!("../data/templates/FileMapping.ini");
pub(crate) const FILE_MAPPINGS_TEMPLATE_FILENAME: &str = "FileMapping.ini";

/// Build system
///
pub(crate) const BUILD_RS: &[u8] = include_bytes!("../data/build.rs");
pub(crate) const BUILD_RS_FILENAME: &str = "build.rs";

/// Package metatdata table used by the build system and this tool
///
/// The string literal gets appended to the existing *Cargo.toml* file. Prior to doing
/// that, placeholders need to be replaced. The following constant uses placeholders
/// to allow the actual values to be shared between this tool and the *build.rs* script it
/// generates.
///
pub(crate) const PACKAGE_METADATA_INIT: &str = r#"
[build-dependencies]
toml = "0.5.8"
cargo-uwp = "0.2.0"


# Metadata driving the build system. Once created, you can freely fill in and change the
# required entries, and have the build system take it from there.

[package.metadata.appxmanifest]
# The following entries relate to the package identity. They are available through the
# [Microsoft Partner Center](https://partner.microsoft.com) dashboard, once you have
# registered a product name. Go to Products -> <product name> -> Product management ->
# Product Identity to find the respective information.

# (String, required) The package name: The value can be found under the key
# "Package/Identity/Name".
$package-identity-name$

# (String, required) The package publisher: The value can be found under the key
# "Package/Identity/Publisher".
$package-identity-publisher$

# (String, optional) The package version: A string of four period-delimited numeric
# values following the pattern "<Major>.<Minor>.<Build>.<Revision>". This is the public-
# facing version, that is displayed in the Microsoft Store. It is also used to name
# binary artifacts generated by the build system.
# When missing, this value is substituted with the "version" entry of the [package]
# table, with the <Revision> field defaulting to 0.
# package-identity-version = "0.0.0.0"


# The following entry controls whether to target Windows 10 Mobile.

# (String, optional) The phone product ID: A string representing the GUID of the Windows
# 10 Mobile product. It is required when targeting Windows 10 Mobile.
# When missing, no packages targeting Windows 10 Mobile are generated.
# package-phoneidentity-productid = "00000000-0000-0000-0000-000000000000"


# The next two entries designate public-facing properties of the application. Either one
# is displayed in the Microsoft Store.

# (String, optional) The package display name: This property must match the application
# name as registered in the Microsoft Partner Center. When missing, this value is
# substituted with the "name" entry of the [package] table.
# package-properties-displayname = "<app name>"

# (String, required) The package publisher display name: This property must match the
# publisher display name under which you plan to publish an application in the Microsoft
# Store. Depending on your Microsoft Partner Center registration this information can be
# found under Account settings -> Organization profile -> Legal -> Contact info.
$package-properties-publisherdisplayname$


# Public facing aspects of an application package.
# See [uap:VisualElements](https://docs.microsoft.com/en-us/uwp/schemas/appxpackage/uapmanifestschema/element-uap-visualelements)

# (String, required) A friendly name for the app that can be displayed to users.
$package-applications-visualelements-displayname$

# (String, required) The description of the app.
$package-applications-visualelements-description$
"#;

/// Placeholders and default values used by the string constant above:
///
/// $package-identity-name$ => PACKAGE_IDENTITY_NAME_KEY = "PACKAGE_IDENTITY_NAME_DEFAULT"
pub(crate) const PACKAGE_IDENTITY_NAME_PLACEHOLDER: &str = "$package-identity-name$";
/// $package-identity-publisher$ => PACKAGE_IDENTITY_PUBLISHER_KEY = "PACKAGE_IDENTITY_PUBLISHER_DEFAULT"
pub(crate) const PACKAGE_IDENTITY_PUBLISHER_PLACEHOLDER: &str = "$package-identity-publisher$";
/// $package-properties-publisherdisplayname$ => PACKAGE_PUBLISHER_DISPLAY_NAME_KEY = "PACKAGE_PUBLISHER_DISPLAY_NAME_DEFAULT"
pub(crate) const PACKAGE_PUBLISHER_DISPLAY_NAME_PLACEHOLDER: &str =
    "$package-properties-publisherdisplayname$";
/// $package-applications-visualelements-displayname$ => PACKAGE_VISUAL_DISPLAY_NAME_KEY = "PACKAGE_VISUAL_DISPLAY_NAME_DEFAULT"
pub(crate) const PACKAGE_VISUAL_DISPLAY_NAME_PLACEHOLDER: &str =
    "$package-applications-visualelements-displayname$";
/// $package-applications-visualelements-description$ => PACKAGE_VISUAL_DESCRIPTION_KEY = "PACKAGE_VISUAL_DESCRIPTION_DEFAULT"
pub(crate) const PACKAGE_VISUAL_DESCRIPTION_PLACEHOLDER: &str =
    "$package-applications-visualelements-description$";

/// Bindings crate
///
pub(crate) const BINDINGS_CRATE_PATH: &str = "bindings";
pub(crate) const BINDINGS_CARGO_TOML: &str = include_str!("../data/bindings/Cargo.toml_");
pub(crate) const BINDINGS_BUILD_RS: &[u8] = include_bytes!("../data/bindings/build.rs_");
pub(crate) const BINDINGS_SRC_LIB_RS: &[u8] = include_bytes!("../data/bindings/src/lib.rs_");

/// Generated src/main.rs
///
pub(crate) const SRC_MAIN_RS: &[u8] = include_bytes!("../data/src/main.rs");

/// Placeholders that get replaced during various operations, when copying from templates
/// to the final artifacts.
pub(crate) const WINDOWS_RS_VERSION_PLACEHOLDER: &str = "$windows-rs-version$";
