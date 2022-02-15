//! Based on https://github.com/rustyhorde/vergen/
//! and on https://github.com/fusion-engineering/rust-git-version.

pub trait IVersion {
    fn version(&self) -> Version;
}

/// Information gathered right before compilation.
#[derive(near_sdk::serde::Serialize, near_sdk::serde::Deserialize, Debug, PartialEq)]
#[serde(crate = "near_sdk::serde")]
pub struct Version {
    /// The project's name.  
    /// eg. `my-project`.
    pub name: String,
    /// The project's semver.  
    /// eg. `0.1.0`.
    pub semver: String,
    /// The git 40-byte commit sha.  
    /// eg. `2beb5ec70ee2c0490cd0f4964544c998e6badbcc`.
    pub git_sha: String,
    /// The commit datetime.  
    /// eg. `2022-02-11 14:26:08 -0300`.
    pub git_datetime: String,
    /// Whether there are modified or untracked files.  
    ///
    /// ie. A contract that was trying to be reproducible
    /// should show `false` in `git_dirty`;
    /// otherwise other un-commited or un-added files could
    /// change the build contents.
    pub git_dirty: bool,
    /// Active cargo features, comma-separated.  
    /// If none were active, `default` is shown.  
    /// eg. `default`.
    pub cargo_features: String,
    /// The build profile.  
    /// eg. `release`.
    pub cargo_profile: String,
    /// eg. `1.56.1`.
    pub rustc_semver: String,
    /// eg. `13.0`.
    pub rustc_llvm: String,
    /// eg. `59eed8a2aac0230a8b53e89d4e99d55912ba6b35`.
    pub rustc_sha: String,
}

impl Version {
    pub fn set_env(&self) {
        // the self.name and self.semver values are set into the
        // env by cargo itself
        //
        set_env("NEARAPPS_GIT_SHA", &self.git_sha);
        set_env("NEARAPPS_GIT_DATETIME", &self.git_datetime);
        set_env("NEARAPPS_GIT_DIRTY", &self.git_dirty.to_string());
        set_env("NEARAPPS_CARGO_FEATURES", &self.cargo_features);
        set_env("NEARAPPS_CARGO_PROFILE", &self.cargo_profile);
        set_env("NEARAPPS_RUSTC_SEMVER", &self.rustc_semver);
        set_env("NEARAPPS_RUSTC_LLVM", &self.rustc_llvm);
        set_env("NEARAPPS_RUSTC_SHA", &self.rustc_sha);
    }
}

#[macro_export]
macro_rules! version_from_env {
    () => {{
        const NAME: &str = std::env!("CARGO_PKG_NAME");
        const SEMVER: &str = std::env!("CARGO_PKG_VERSION");
        const GIT_SHA: &str = std::env!("NEARAPPS_GIT_SHA");
        const GIT_DATETIME: &str = std::env!("NEARAPPS_GIT_DATETIME");
        const GIT_DIRTY: &str = std::env!("NEARAPPS_GIT_DIRTY");
        const CARGO_FEATURES: &str = std::env!("NEARAPPS_CARGO_FEATURES");
        const CARGO_PROFILE: &str = std::env!("NEARAPPS_CARGO_PROFILE");
        const RUSTC_SEMVER: &str = std::env!("NEARAPPS_RUSTC_SEMVER");
        const RUSTC_LLVM: &str = std::env!("NEARAPPS_RUSTC_LLVM");
        const RUSTC_SHA: &str = std::env!("NEARAPPS_RUSTC_SHA");

        Version {
            name: NAME.to_string(),
            semver: SEMVER.to_string(),
            git_sha: GIT_SHA.to_string(),
            git_datetime: GIT_DATETIME.to_string(),
            git_dirty: GIT_DIRTY.parse().unwrap(),
            cargo_features: CARGO_FEATURES.to_string(),
            cargo_profile: CARGO_PROFILE.to_string(),
            rustc_semver: RUSTC_SEMVER.to_string(),
            rustc_llvm: RUSTC_LLVM.to_string(),
            rustc_sha: RUSTC_SHA.to_string(),
        }
    }};
}

fn set_env(key: &str, value: &str) {
    assert!(!key.contains('='));
    println!("cargo:rustc-env={}={}", key, value);
}

pub mod build {
    use super::Version;
    use std::env;
    use std::path::{Path, PathBuf};
    use std::process::Command;

    #[allow(clippy::print_literal)]
    pub fn setup_rerun() {
        println!("cargo:rerun-if-changed={}", "build.rs");

        let current_dir = canonicalize_path(&env::current_dir().unwrap());
        let build_file = PathBuf::from(current_dir).join("build.rs");

        // println!("cargo:rerun-if-changed={}", current_dir.display());

        // touch build.rs to trigger re-build
        {
            let cmd = Command::new("touch").arg(build_file).output().unwrap();
            assert!(cmd.status.success());
            let output = String::from_utf8_lossy(&cmd.stdout);
            assert!(output.is_empty());
        };

        //re-run on git HEAD change
        let repo_toplevel = {
            let cmd = Command::new("git")
                .arg("rev-parse")
                .args(["--show-toplevel"])
                .output()
                .unwrap();
            assert!(cmd.status.success());
            let output = String::from_utf8_lossy(&cmd.stdout);
            // strips newline suffix
            let mut lines = output.lines();
            let git_toplevel = lines.next().unwrap();
            assert_eq!(lines.count(), 0);
            git_toplevel.to_string()
        };
        let git_head = {
            let git_dirname = {
                let cmd = Command::new("git")
                    .arg("rev-parse")
                    .args(["--git-dir"])
                    .output()
                    .unwrap();
                assert!(cmd.status.success());
                let output = String::from_utf8_lossy(&cmd.stdout);
                // strips newline suffix
                let mut lines = output.lines();
                let git_dirname = lines.next().unwrap();
                assert_eq!(lines.count(), 0);
                git_dirname.to_string()
            };

            PathBuf::from(repo_toplevel).join(git_dirname).join("HEAD")
        };

        println!("cargo:rerun-if-changed={}", git_head.display());
    }

    /// Gets information
    pub fn create_version() -> Version {
        let git_sha = {
            let cmd = Command::new("git")
                .arg("describe")
                .args(["--abbrev=40", "--always", "--exclude='*'"])
                .output()
                .unwrap();
            assert!(cmd.status.success());
            let output = String::from_utf8_lossy(&cmd.stdout);
            // strips newline suffix
            let mut lines = output.lines();
            let git_sha = lines.next().unwrap();
            assert_eq!(lines.count(), 0);
            git_sha.to_string()
        };

        let git_datetime = {
            let cmd = Command::new("git")
                .args(["--no-pager", "show"])
                .args(["-s", "--format=%ci"])
                .output()
                .unwrap();
            assert!(cmd.status.success());
            let output = String::from_utf8_lossy(&cmd.stdout);
            // strips newline suffix
            let mut lines = output.lines();
            let git_datetime = lines.next().unwrap();
            assert_eq!(lines.count(), 0);
            git_datetime.to_string()
        };

        let git_dirty = {
            let cmd = Command::new("git")
                .arg("status")
                .args(["--porcelain", "-u", "--no-column"])
                .output()
                .unwrap();
            assert!(cmd.status.success());
            let output = String::from_utf8_lossy(&cmd.stdout);
            !output.is_empty()
        };

        let cargo_features = {
            let features: Vec<String> = env::vars().filter_map(is_cargo_feature).collect();
            let feature_str = features.as_slice().join(",");
            if feature_str.is_empty() {
                "default".to_string()
            } else {
                feature_str
            }
        };

        let cargo_profile = env::var("PROFILE").unwrap();

        let rustc = rustc_version::version_meta().unwrap();
        let rustc_semver = rustc.semver.to_string();
        let rustc_llvm = rustc.llvm_version.unwrap().to_string();
        let rustc_sha = rustc.commit_hash.unwrap();

        const NAME: &str = std::env!("CARGO_PKG_NAME");
        const SEMVER: &str = std::env!("CARGO_PKG_VERSION");

        Version {
            name: NAME.to_string(),
            semver: SEMVER.to_string(),
            git_sha,
            git_datetime,
            git_dirty,
            cargo_features,
            cargo_profile,
            rustc_semver,
            rustc_llvm,
            rustc_sha,
        }

        // set_env("NEARAPPS_GIT_SHA", &git_sha);
        // set_env("NEARAPPS_GIT_DATETIME", &git_datetime);
        // set_env("NEARAPPS_GIT_DIRTY", &git_dirty.to_string());
        // set_env("NEARAPPS_CARGO_FEATURES", &cargo_features);
        // set_env("NEARAPPS_CARGO_PROFILE", &cargo_profile);
        // set_env("NEARAPPS_RUSTC_SEMVER", &rustc_semver);
        // set_env("NEARAPPS_RUSTC_LLVM", &rustc_llvm);
        // set_env("NEARAPPS_RUSTC_SHA", &rustc_sha);
    }

    fn is_cargo_feature(var: (String, String)) -> Option<String> {
        let (k, _) = var;
        if k.starts_with("CARGO_FEATURE_") {
            Some(k.replace("CARGO_FEATURE_", "").to_lowercase())
        } else {
            None
        }
    }

    fn canonicalize_path(path: &Path) -> String {
        path.canonicalize()
            .unwrap()
            .into_os_string()
            .into_string()
            .unwrap()
    }
}
