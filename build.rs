use std::{env, process::Command};

fn main() {
    // check if target is 32bit
    let target = std::env::var("TARGET").unwrap();
    let ptr_size = env::var("CARGO_CFG_TARGET_POINTER_WIDTH")
        .expect("CARGO_CFG_TARGET_POINTER_WIDTH is not set!");
    if ptr_size != "64" {
        panic!(
            "This crate is not supported on 32-bit targets, {} has {}-bit pointer width",
            target, ptr_size
        );
    }

    if let Ok(output) = Command::new("git").args(["rev-parse", "HEAD"]).output() {
        let is_dirty = Command::new("git")
            .args([
                "diff",
                "--ignore-matching-lines='^version = \".*\"'",
                "--quiet",
            ])
            .status()
            .unwrap()
            .code()
            .unwrap_or_default()
            != 0;

        let dirty = if is_dirty { "-dirty" } else { "" };
        let git_hash: String = String::from_utf8(output.stdout).unwrap();
        println!(
            "cargo:rustc-env=GIT_HASH={}{dirty}",
            git_hash.strip_suffix('\n').unwrap_or(&git_hash)
        );
    } else {
        println!("cargo:rustc-env=GIT_HASH=unknown-revision");
    }
}
