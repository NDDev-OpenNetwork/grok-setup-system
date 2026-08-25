//! The Grok Build setup system.
//!
//! This file is the harness's *facts*. Every command over them lives in
//! [`harness_runtime`], shared with every other setup system, so a change to
//! behaviour lands once and a change to Grok Build's surface lands here.
//!
//! The owner assigned this harness the program lifecycle as well, and it is
//! declared: `src/software.rs` carries the six artifacts xAI publishes, and
//! grok is the one product whose bytes *are* the program -- its direct
//! distribution needs no unpacking at all.

use std::process::ExitCode;

mod software;

use harness_runtime::Harness;
use provider_v3::{ComponentKind, ProjectionKind};

/// Everything specific to Grok Build, verified against `grok-baseline.json`.
pub const GROK: Harness = Harness {
    // The consumer's closed harness enum spells this `grok-build`.
    harness_id: "grok-build",
    provider_id: "grok-setup-system",
    version: env!("CARGO_PKG_VERSION"),
    product: "Grok Build",
    vendor: "xAI",
    documented_config_home: "~/.grok",
    config_home_env: "GROK_HOME",
    control_directory: ".grok-setup-system",
    state_file: "NDDEV-GROK-PROVIDER.json",
    predecessor_state_file: "NDDEV-GROK-BUILD-SETUP.json",
    profile_id: "grok/native-and-plugins/1",
    // Everything outside this list is a sibling overlay preserved verbatim.
    native_namespaces: &[
        "AGENTS.md",
        "config.toml",
        "managed_config.toml",
        "requirements.toml",
        "sandbox.toml",
        "skills",
        "agents",
        "commands",
        "plugins",
        "hooks",
        ".mcp.json",
    ],
    // The product's own: credentials, session history and runtime caches. Never
    // read, never written, and never copied into a backup slot.
    never_touch: &["sessions"],
    permission_profiles: &["default"],
    component_kinds: &[
        ComponentKind::Instruction,
        ComponentKind::Skill,
        ComponentKind::Agent,
        ComponentKind::Hook,
        ComponentKind::Plugin,
        ComponentKind::Setting,
    ],
    projection_kinds: &[
        ProjectionKind::NativeFiles,
        ProjectionKind::Marketplace,
        ProjectionKind::Plugin,
    ],
    max_files: 8192,
    max_bytes: 64 * 1024 * 1024,
    kit_identity: include_str!("../../../provider-kit/v3/KIT-IDENTITY.json"),
    software: Some(software::SOFTWARE),
};

fn main() -> ExitCode {
    harness_runtime::run(&GROK, std::env::args().skip(1).collect())
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::panic)]

    use super::*;

    #[test]
    fn the_declaration_is_valid_and_names_this_host() {
        let info = GROK.provider_info().unwrap();
        assert_eq!(info.provider_id, env!("CARGO_PKG_NAME"));
        assert_eq!(info.harness_id, "grok-build");
        assert_eq!(info.protocol_version, 3);
        assert!(info.supports_this_host());
    }

    #[test]
    fn no_namespace_is_both_owned_and_disclaimed() {
        for name in GROK.never_touch {
            assert!(
                !GROK.native_namespaces.contains(name),
                "{name} is claimed and disclaimed"
            );
        }
    }

    #[test]
    fn the_baseline_this_harness_cites_is_present_and_readable() {
        // The facts above are transcribed from it; a build whose baseline is
        // missing has no evidence for what it claims to own.
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../references/grok-baseline.json");
        let value: serde_json::Value =
            serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
        assert!(value.is_object());
    }

    #[test]
    fn the_control_directory_and_state_file_are_provider_owned_not_product_owned() {
        assert!(GROK.control_directory.contains("setup-system"));
        assert!(GROK.state_file.starts_with("NDDEV-"));
        assert!(!GROK.native_namespaces.contains(&GROK.state_file));
    }
}
