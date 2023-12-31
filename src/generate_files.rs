pub fn generate_godot_project_file(project_name: &str) -> String {
    format!(
        r#"; Engine configuration file.
; It's best edited using the editor UI and not directly,
; since the parameters that go here are not all obvious.
;
; Format:
;   [section] ; section goes between []
;   param=value ; assign values to parameters

config_version=5

[application]

config/name="{}"
config/features=PackedStringArray("4.2", "GL Compatibility")
config/icon="res://icon.svg"

[rendering]

renderer/rendering_method="gl_compatibility"
renderer/rendering_method.mobile="gl_compatibility""#,
        project_name
    )
}

pub fn generate_gdextention_file(project_name: &str, reloadable: bool) -> String {
    format!(
        r#"[configuration]
entry_symbol = "gdext_rust_init"
compatibility_minimum = 4.1
{reloadable}
[libraries]
linux.debug.x86_64 =     "res://../rust/target/debug/lib{YourCrate}.so"
linux.release.x86_64 =   "res://../rust/target/release/lib{YourCrate}.so"
windows.debug.x86_64 =   "res://../rust/target/debug/{YourCrate}.dll"
windows.release.x86_64 = "res://../rust/target/release/{YourCrate}.dll"
macos.debug =            "res://../rust/target/debug/lib{YourCrate}.dylib"
macos.release =          "res://../rust/target/release/lib{YourCrate}.dylib"
macos.debug.arm64 =      "res://../rust/target/debug/lib{YourCrate}.dylib"
macos.release.arm64 =    "res://../rust/target/release/lib{YourCrate}.dylib""#,
        reloadable = if reloadable {
            "reloadable = true\n"
        } else {
            ""
        },
        YourCrate = project_name
    )
}

pub fn generate_cargo_toml(project_name: &str) -> String {
    format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]  # Compile this crate to a dynamic C library.

[dependencies]
godot = {{ git = "https://github.com/godot-rust/gdext", branch = "master" }}
"#,
        project_name
    )
}

pub fn generate_launch_config(godot_dir: &str, godot_location: &str) -> String {
    format!(
        r#"{{
    "configurations": [
        {{
            "name": "Debug Project (Godot 4)",
            "type": "lldb",
            "request": "launch",
            "preLaunchTask": "rust: cargo build",
            "cwd": "${{workspaceFolder}}/../{}",
            "args": [
                "-e", // run editor (remove this to launch the scene directly)
                "-w", // windowed mode
            ],
            "program": "{}"
        }}
    ]
}}"#,
        godot_dir, godot_location
    )
}

pub fn generate_godot_gitignore() -> String {
    r#"# Godot 4+ specific ignores
.godot/
!.godot/extension_list.cfg

# Godot-specific ignores
.import/
export.cfg
export_presets.cfg

# Imported translations (automatically generated from CSV files)
*.translation

# Mono-specific ignores
.mono/
data_*/
mono_crash.*.json"#
        .to_string()
}

pub fn generate_rust_gitignore() -> String {
    r#"# Generated by Cargo
# will have compiled files and executables
debug/
target/

# Remove Cargo.lock from gitignore if creating an executable, leave it for libraries
# More information here https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html
# Cargo.lock

# These are backup files generated by rustfmt
**/*.rs.bk

# MSVC Windows builds of rustc generate these, which store debugging information
*.pdb"#
        .to_string()
}
