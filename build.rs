use std::{
    path::{Path, PathBuf},
    process::Command,
};

const BOOTLOADER_X86_64_UEFI_VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    let uefi_path = build_uefi_bootloader(&out_dir);
    println!(
        "cargo:rustc-env=UEFI_BOOTLOADER_PATH={}",
        uefi_path.display()
    );
}

#[cfg(not(docsrs_dummy_build))]
fn build_uefi_bootloader(out_dir: &Path) -> PathBuf {
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".into());
    let mut cmd = Command::new(cargo);
    cmd.arg("install").arg("bootloader-x86_64-uefi");
    if Path::new("uefi").exists() {
        // local build
        cmd.arg("--path").arg("uefi");
        println!("cargo:rerun-if-changed=uefi");
    } else {
        cmd.arg("--version").arg(BOOTLOADER_X86_64_UEFI_VERSION);
    }
    cmd.arg("--locked");
    cmd.arg("--target").arg("x86_64-unknown-uefi");
    cmd.arg("-Zbuild-std=core")
        .arg("-Zbuild-std-features=compiler-builtins-mem");
    cmd.arg("--root").arg(out_dir);
    cmd.env_remove("RUSTFLAGS");
    cmd.env_remove("CARGO_ENCODED_RUSTFLAGS");
    let status = cmd
        .status()
        .expect("failed to run cargo install for uefi bootloader");
    if status.success() {
        let path = out_dir.join("bin").join("bootloader-x86_64-uefi.efi");
        assert!(
            path.exists(),
            "uefi bootloader executable does not exist after building"
        );
        path
    } else {
        panic!("failed to build uefi bootloader");
    }
}

// dummy implementations because docsrs builds have no network access

#[cfg(docsrs_dummy_build)]
fn build_uefi_bootloader(_out_dir: &Path) -> PathBuf {
    PathBuf::new()
}
