use std::{io::Error, path::PathBuf};

use cliclack::{input, intro, log, multiselect, outro};
use colored::Colorize;

#[derive(Default, Clone, PartialEq, Eq, Debug)]
enum QolFeatures {
    #[default] // needed for multi-select
    Git,
    ReloadableExtension,
    VscodeLaunchConfig,
    VscodeExtensions,
}

fn main() -> Result<(), Error> {
    intro("Scaffold Godot-Rust Project".on_cyan().black())?;

    let dir: PathBuf = input("Project Directory (leave empty for current folder): ")
        .default_input(".")
        .interact()?;

    log::info("[Godot]".underline().bold())?;

    let godot_dir_name: String = input("Godot Directory Name: ")
        .default_input("godot")
        .interact()?;
    let godot_name: String = input("Project Name: ").interact()?;

    log::info("[Rust]".underline().bold())?;

    let rust_dir_name: String = input("Rust Directory Name: ")
        .default_input("rust")
        .interact()?;

    let rust_name: String = input("Rust Project Name: ")
        .default_input("rust")
        .interact()?;

    let qol_features: Vec<QolFeatures> = multiselect("QOL Features")
        .item(QolFeatures::Git, "Git", "")
        .item(
            QolFeatures::ReloadableExtension,
            "Reloadable Extension",
            "make the GDExtension reloadable",
        )
        .item(
            QolFeatures::VscodeLaunchConfig,
            "VSCode Launch Config",
            "create .vscode/launch.json",
        )
        .item(
            QolFeatures::VscodeExtensions,
            "VSCode Extensions",
            "create .vscode/extensions.json with recommended extensions",
        )
        .required(false)
        .interact()?;

    let godot_full_path = dir.join(&godot_dir_name);
    let rust_full_path = dir.join(&rust_dir_name);

    log::info("Creating Godot Project")?;
    std::fs::create_dir_all(&godot_full_path)?;

    std::fs::write(
        godot_full_path.join(".gitignore"),
        generate_godot_gitignore(),
    )?;

    std::fs::write(
        godot_full_path.join("project.godot"),
        generate_godot_project_file(&godot_name),
    )?;

    std::fs::write(
        godot_full_path.join(format!("{}.gdextension", &rust_name)),
        generate_gdextention_file(
            &rust_name,
            qol_features.contains(&QolFeatures::ReloadableExtension),
        ),
    )?;

    std::fs::write(
        godot_full_path.join("icon.svg"),
        r##"<svg height="128" width="128" xmlns="http://www.w3.org/2000/svg"><rect x="2" y="2" width="124" height="124" rx="14" fill="#363d52" stroke="#212532" stroke-width="4"/><g transform="scale(.101) translate(122 122)"><g fill="#fff"><path d="M105 673v33q407 354 814 0v-33z"/><path fill="#478cbf" d="m105 673 152 14q12 1 15 14l4 67 132 10 8-61q2-11 15-15h162q13 4 15 15l8 61 132-10 4-67q3-13 15-14l152-14V427q30-39 56-81-35-59-83-108-43 20-82 47-40-37-88-64 7-51 8-102-59-28-123-42-26 43-46 89-49-7-98 0-20-46-46-89-64 14-123 42 1 51 8 102-48 27-88 64-39-27-82-47-48 49-83 108 26 42 56 81zm0 33v39c0 276 813 276 813 0v-39l-134 12-5 69q-2 10-14 13l-162 11q-12 0-16-11l-10-65H447l-10 65q-4 11-16 11l-162-11q-12-3-14-13l-5-69z"/><path d="M483 600c3 34 55 34 58 0v-86c-3-34-55-34-58 0z"/><circle cx="725" cy="526" r="90"/><circle cx="299" cy="526" r="90"/></g><g fill="#414042"><circle cx="307" cy="532" r="60"/><circle cx="717" cy="532" r="60"/></g></g></svg>"##,
    )?;

    log::info("Creating Rust Project")?;
    std::fs::create_dir_all(&rust_full_path)?;

    std::fs::write(
        rust_full_path.join(".gitignore"),
        generate_godot_gitignore(),
    )?;

    std::fs::write(
        rust_full_path.join("Cargo.toml"),
        generate_cargo_toml(&rust_name),
    )?;

    let rust_src_dir = rust_full_path.join("src");

    std::fs::create_dir(&rust_src_dir)?;

    std::fs::write(
        rust_src_dir.join("lib.rs"),
        r#"use godot::prelude::*;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}"#,
    )?;

    for feature in qol_features {
        match feature {
            QolFeatures::Git => {
                log::info("Initializing Git")?;
                std::process::Command::new("git")
                    .arg("init")
                    .current_dir(&dir)
                    .spawn()?
                    .wait()?;
            }
            QolFeatures::VscodeLaunchConfig => {
                log::info("Creating VSCode Launch Config")?;

                let default_godot_location = if cfg!(target_os = "windows") {
                    "C:\\Program Files\\Godot\\Godot_v4.2.1-stable_win64.exe"
                } else if cfg!(target_os = "macos") {
                    "/Applications/Godot.app/Contents/MacOS/Godot"
                } else {
                    "/usr/bin/godot"
                };

                let godot_location: String = input("Godot Executable Location: ")
                    .default_input(default_godot_location)
                    .interact()?;

                std::fs::create_dir_all(rust_full_path.join(".vscode"))?;
                std::fs::write(
                    rust_full_path.join(".vscode/launch.json"),
                    generate_launch_config(
                        &godot_dir_name,
                        &godot_location,
                    ),
                )?;
            }
            QolFeatures::VscodeExtensions => {
                log::info("Creating VSCode Extensions Config")?;

                std::fs::create_dir_all(rust_full_path.join(".vscode"))?;
                std::fs::write(
                    rust_full_path.join(".vscode/extensions.json"),
                    r#"{
    "recommendations": [
        "rust-lang.rust",
        "vadimcn.vscode-lldb",
        "1YiB.rust-bundle",
        "tamasfe.even-better-toml"
    ]
}"#)?;
            }
            _ => {}
        };
    }


    outro("Done! Enjoy your new project!")
}

fn generate_godot_project_file(project_name: &str) -> String {
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

fn generate_gdextention_file(project_name: &str, reloadable: bool) -> String {
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

fn generate_cargo_toml(project_name: &str) -> String {
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

fn generate_launch_config(godot_dir: &str, godot_location: &str) -> String {
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

fn generate_godot_gitignore() -> String {
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
mono_crash.*.json"#.to_string()
}

fn generate_rust_gitignore() -> String {
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
*.pdb"#.to_string()
}
