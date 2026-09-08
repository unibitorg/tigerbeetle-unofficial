//! Proves that `TIGERBEETLE_USE_PREBUILT=1` never invokes `bindgen`/libclang.
//!
//! Spawns a real, isolated `cargo build` of this crate with a fixture prebuilt
//! manifest, `LIBCLANG_PATH` pointed at a directory that does not exist (so any
//! attempt to load libclang fails hard) and `ZIG_PATH` pointed at a binary that
//! does not exist (so any attempt to run the Zig build fails hard too). If the
//! build still succeeds, the prebuilt code path touched neither bindgen nor Zig.

#![cfg(any(
    all(target_arch = "aarch64", target_os = "macos"),
    all(target_arch = "x86_64", target_os = "linux", target_env = "gnu"),
    all(target_arch = "aarch64", target_os = "linux", target_env = "gnu"),
))]

use std::{
    env, fs,
    path::{Path, PathBuf},
    process::{Command, Output},
};

use sha2::{Digest as _, Sha256};

#[cfg(all(target_arch = "aarch64", target_os = "macos"))]
const HOST_TARGET: &str = "aarch64-apple-darwin";
#[cfg(all(target_arch = "x86_64", target_os = "linux", target_env = "gnu"))]
const HOST_TARGET: &str = "x86_64-unknown-linux-gnu";
#[cfg(all(target_arch = "aarch64", target_os = "linux", target_env = "gnu"))]
const HOST_TARGET: &str = "aarch64-unknown-linux-gnu";

fn host_target_lib_subdir() -> &'static str {
    match HOST_TARGET {
        "aarch64-unknown-linux-gnu" => "aarch64-linux-gnu.2.27",
        "aarch64-apple-darwin" => "aarch64-macos",
        "x86_64-unknown-linux-gnu" => "x86_64-linux-gnu.2.27",
        other => panic!("unsupported host target for this test: {other}"),
    }
}

fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    let mut out = String::with_capacity(digest.len() * 2);
    for byte in digest {
        use std::fmt::Write as _;
        write!(&mut out, "{byte:02x}").unwrap();
    }
    out
}

fn write_hashed(fixture_dir: &Path, name: &str, content: &[u8]) -> (String, String) {
    fs::write(fixture_dir.join(name), content).unwrap();
    (name.to_owned(), sha256_hex(content))
}

/// Builds a fixture "library + headers" prebuilt manifest for the current host
/// target and returns its path. This is the manifest format consumed via
/// `TIGERBEETLE_PREBUILT_MANIFEST`, unrelated to the crate's own shipped
/// `pregenerated/` bindings, which are resolved internally.
fn write_fixture_manifest(fixture_dir: &Path) -> PathBuf {
    let sys_crate_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    let tb_client_h = fs::read(sys_crate_dir.join("src/tb_client.h")).unwrap();
    let wrapper_h = fs::read(sys_crate_dir.join("src/wrapper.h")).unwrap();
    let (tb_client_h_name, tb_client_h_sha256) =
        write_hashed(fixture_dir, "tb_client.h", &tb_client_h);
    let (wrapper_h_name, wrapper_h_sha256) = write_hashed(fixture_dir, "wrapper.h", &wrapper_h);

    // A valid empty Unix archive is enough here: this smoke builds only the sys
    // rlib, while downstream owner release tests exercise the real C library.
    let (lib_name, lib_sha256) = write_hashed(fixture_dir, "libtb_client.a", b"!<arch>\n");

    let manifest = format!(
        r#"{{
  "schema_version": 1,
  "tigerbeetle_release": "0.16.78",
  "tigerbeetle_commit": "c3d9b09dc88e94dde9ac915c6e94a4c650332080",
  "sys_crate_version": {sys_crate_version:?},
  "headers": {{
    "tb_client.h": {{"path": {tb_client_h_name:?}, "sha256": {tb_client_h_sha256:?}}},
    "wrapper.h": {{"path": {wrapper_h_name:?}, "sha256": {wrapper_h_sha256:?}}}
  }},
  "targets": {{
    {host_target:?}: {{
      "tigerbeetle_lib_subdir": {lib_subdir:?},
      "lib": {{"path": {lib_name:?}, "sha256": {lib_sha256:?}}}
    }}
  }}
}}
"#,
        sys_crate_version = env!("CARGO_PKG_VERSION"),
        host_target = HOST_TARGET,
        lib_subdir = host_target_lib_subdir(),
    );

    let manifest_path = fixture_dir.join("manifest.json");
    fs::write(&manifest_path, manifest).unwrap();
    manifest_path
}

fn run_nested_build(manifest_path: &Path, target_name: &str, allow_fallback: bool) -> Output {
    let target_tmp_dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR"));
    let nested_target_dir = target_tmp_dir.join(target_name);
    let _ = fs::remove_dir_all(&nested_target_dir);
    let workspace_manifest_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("Cargo.toml");

    let mut command = Command::new(env!("CARGO"));
    command
        .args([
            "build",
            "-p",
            "tigerbeetle-unofficial-sys",
            "--lib",
            "--manifest-path",
        ])
        .arg(&workspace_manifest_path)
        .arg("--target-dir")
        .arg(&nested_target_dir)
        .env("TIGERBEETLE_USE_PREBUILT", "1")
        .env("TIGERBEETLE_PREBUILT_MANIFEST", manifest_path)
        .env("LIBCLANG_PATH", target_tmp_dir.join("no-such-libclang-dir"))
        .env("ZIG_PATH", target_tmp_dir.join("no-such-zig-binary"));
    if allow_fallback {
        command.env("TIGERBEETLE_ALLOW_ZIG_FALLBACK", "1");
    } else {
        command.env_remove("TIGERBEETLE_ALLOW_ZIG_FALLBACK");
    }
    command.output().expect("spawning nested `cargo build`")
}

#[test]
fn use_prebuilt_never_invokes_bindgen_or_zig() {
    let target_tmp_dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR"));
    let fixture_dir = target_tmp_dir.join("prebuilt_skips_bindgen_fixture");
    let _ = fs::remove_dir_all(&fixture_dir);
    fs::create_dir_all(&fixture_dir).unwrap();
    let manifest_path = write_fixture_manifest(&fixture_dir);

    let output = run_nested_build(&manifest_path, "nested-success-target", false);

    assert!(
        output.status.success(),
        "`cargo build` with TIGERBEETLE_USE_PREBUILT=1 failed: {}; this means the \
         prebuilt code path touched bindgen/libclang or Zig despite a valid prebuilt manifest \
         being supplied (both were sabotaged for this test): {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
}

#[test]
fn explicit_prebuilt_is_fail_closed_even_when_legacy_fallback_flag_is_set() {
    let target_tmp_dir = PathBuf::from(env!("CARGO_TARGET_TMPDIR"));
    let fixture_dir = target_tmp_dir.join("prebuilt_fail_closed_fixture");
    let _ = fs::remove_dir_all(&fixture_dir);
    fs::create_dir_all(&fixture_dir).unwrap();
    let manifest_path = write_fixture_manifest(&fixture_dir);
    fs::write(fixture_dir.join("tb_client.h"), b"tampered").unwrap();

    let output = run_nested_build(&manifest_path, "nested-fail-closed-target", true);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("sha256 mismatch"),
        "unexpected failure: {stderr}"
    );
    assert!(!stderr.contains("running Zig build"), "{stderr}");
    assert!(!stderr.contains("ZIG_PATH` is set"), "{stderr}");
}
