//! Embed a Windows VERSIONINFO resource so Explorer shows file and product version.

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-env-changed=CARGO_PKG_VERSION");
    println!("cargo:rerun-if-env-changed=CARGO_PKG_DESCRIPTION");

    let target = env::var("TARGET").unwrap_or_default();
    if !target.contains("windows") {
        return;
    }

    let (major, minor, patch, build) = version_parts();
    let shown = display_version(major, minor, patch, build);
    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR"));
    let rc_path = out_dir.join("version.rc");
    let res_path = out_dir.join("version.res");

    fs::write(&rc_path, version_script(&shown, major, minor, patch, build))
        .expect("write version.rc");

    let rc = find_rc().unwrap_or_else(|| {
        panic!("rc.exe not found; install the Windows SDK to embed the file version")
    });
    let status = Command::new(&rc)
        .arg("/nologo")
        .arg("/fo")
        .arg(&res_path)
        .arg(&rc_path)
        .status()
        .unwrap_or_else(|err| panic!("failed to run {}: {err}", rc.display()));
    if !status.success() {
        panic!("rc.exe failed with {status}");
    }

    let link_path = res_path.to_string_lossy().replace('\\', "/");
    println!("cargo:rustc-link-arg-bin=guid={link_path}");
}

fn version_parts() -> (u16, u16, u16, u16) {
    let version = env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "0.0.0".to_string());
    let mut parts = [0u16; 4];
    for (index, part) in version.split('.').take(4).enumerate() {
        parts[index] = part.parse().unwrap_or(0);
    }
    (parts[0], parts[1], parts[2], parts[3])
}

/// `2.0.0` is shown as `2.0`. A non-zero patch or build stays visible.
fn display_version(major: u16, minor: u16, patch: u16, build: u16) -> String {
    if build != 0 {
        format!("{major}.{minor}.{patch}.{build}")
    } else if patch != 0 {
        format!("{major}.{minor}.{patch}")
    } else {
        format!("{major}.{minor}")
    }
}

fn version_script(shown: &str, major: u16, minor: u16, patch: u16, build: u16) -> String {
    let description = rc_escape(
        &env::var("CARGO_PKG_DESCRIPTION").unwrap_or_else(|_| "guid".to_string()),
    );
    let product = rc_escape(&env::var("CARGO_PKG_NAME").unwrap_or_else(|_| "guid".to_string()));
    let shown = rc_escape(shown);
    format!(
        r#"LANGUAGE 9, 1

1 VERSIONINFO
FILEVERSION {major},{minor},{patch},{build}
PRODUCTVERSION {major},{minor},{patch},{build}
FILEFLAGSMASK 0x3fL
FILEFLAGS 0x0L
FILEOS 0x40004L
FILETYPE 0x1L
FILESUBTYPE 0x0L
BEGIN
    BLOCK "StringFileInfo"
    BEGIN
        BLOCK "040904B0"
        BEGIN
            VALUE "FileDescription", "{description}"
            VALUE "FileVersion", "{shown}"
            VALUE "InternalName", "{product}.exe"
            VALUE "OriginalFilename", "{product}.exe"
            VALUE "ProductName", "{product}"
            VALUE "ProductVersion", "{shown}"
        END
    END
    BLOCK "VarFileInfo"
    BEGIN
        VALUE "Translation", 0x409, 1200
    END
END
"#
    )
}

fn rc_escape(text: &str) -> String {
    text.replace('\\', "\\\\").replace('"', "\\\"")
}

fn find_rc() -> Option<PathBuf> {
    if let Some(path) = env::var_os("PATH").and_then(|path| {
        env::split_paths(&path).find_map(|dir| {
            let candidate = dir.join("rc.exe");
            candidate.is_file().then_some(candidate)
        })
    }) {
        return Some(path);
    }

    let mut found = Vec::new();
    for root in [
        Path::new(r"C:\Program Files (x86)\Windows Kits\10\bin"),
        Path::new(r"C:\Program Files\Windows Kits\10\bin"),
    ] {
        let entries = match fs::read_dir(root) {
            Ok(entries) => entries,
            Err(_) => continue,
        };
        for entry in entries.flatten() {
            for host in ["x64", "arm64", "x86"] {
                let candidate = entry.path().join(host).join("rc.exe");
                if candidate.is_file() {
                    found.push(candidate);
                }
            }
        }
    }
    found.sort();
    found.pop()
}
