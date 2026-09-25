//! Grok Build's own program, as measured rather than as described.
//!
//! Generated from the `software_artifacts` block of
//! `references/grok-baseline.json`. Every member path below was read out
//! of the archive it names, not assumed: codex's carries the target triple and
//! so genuinely differs per platform.
//!
//! Where a `previous_software_artifacts` block is present, it is transcribed
//! too. It is not a second choice: the outgoing current pin is stored there on
//! a bump, so the pair is always two consecutive real releases and there is
//! still exactly one value to keep fresh.
//!
//! Do not edit. The test at the bottom re-reads that baseline and compares it
//! field by field, so an edit here fails rather than silently installing bytes
//! nobody measured.

use harness_runtime::{Artifact, Delivery, Previous, Shape, Software};

/// The artifacts grok is published as.
pub(crate) const ARTIFACTS: &[Artifact] = &[
    Artifact {
        platform: "linux/arm64",
        url: "https://x.ai/cli/grok-1.0.41-linux-aarch64",
        bytes: 138_577_144,
        sha256: "sha256:7c0b8c973af6a78e2037f19ed93033471b8c5e722f9ff04b86b092e066e60d74",
        shape: Shape::Raw,
        member: "",
    },
    Artifact {
        platform: "linux/x86_64",
        url: "https://x.ai/cli/grok-1.0.41-linux-x86_64",
        bytes: 165_967_424,
        sha256: "sha256:9ce03ed23e16ea01072b4496263d6213a27899e1e3e107f008d36edf82e70407",
        shape: Shape::Raw,
        member: "",
    },
    Artifact {
        platform: "macos/arm64",
        url: "https://x.ai/cli/grok-1.0.41-macos-aarch64",
        bytes: 145_657_952,
        sha256: "sha256:9c844eb13365180787d9ad22b2b3748a024be8e1ed845253cc114781b31c591d",
        shape: Shape::Raw,
        member: "",
    },
    Artifact {
        platform: "macos/x86_64",
        url: "https://x.ai/cli/grok-1.0.41-macos-x86_64",
        bytes: 163_046_304,
        sha256: "sha256:8fce04ab8f33a0f604e405a64b3ec28cf92aa600ca6b80106e4b14e8bf80d95b",
        shape: Shape::Raw,
        member: "",
    },
    Artifact {
        platform: "windows/arm64",
        url: "https://x.ai/cli/grok-1.0.41-windows-aarch64.exe",
        bytes: 133_252_424,
        sha256: "sha256:ea454a36db52b263a502b6628d55a7f1d24710d00682ac81e241f91dd160c540",
        shape: Shape::Raw,
        member: "",
    },
    Artifact {
        platform: "windows/x86_64",
        url: "https://x.ai/cli/grok-1.0.41-windows-x86_64.exe",
        bytes: 154_082_120,
        sha256: "sha256:ab5d2a424f08281798acbdbb06076166fe000d7995ede94a673417b805210a25",
        shape: Shape::Raw,
        member: "",
    },
];

/// The artifacts 1.0.38 was published as, kept so
/// `software_update` has a version to move from and `rollback` a tree to
/// return to. Measured from bytes when it was the current pin.
pub(crate) const PREVIOUS_ARTIFACTS: &[Artifact] = &[
    Artifact {
        platform: "linux/arm64",
        url: "https://x.ai/cli/grok-1.0.38-linux-aarch64",
        bytes: 137_404_712,
        sha256: "sha256:d19211b4d22421c622ceefce62a12e8954f3495f45378e0b315f4e80dd358f96",
        shape: Shape::Raw,
        member: "",
    },
    Artifact {
        platform: "linux/x86_64",
        url: "https://x.ai/cli/grok-1.0.38-linux-x86_64",
        bytes: 164_567_296,
        sha256: "sha256:d09092c50f1bf1cd686a0ecd3489aedd83ec2d16b27a821ca9f0994a4b605ef9",
        shape: Shape::Raw,
        member: "",
    },
    Artifact {
        platform: "macos/arm64",
        url: "https://x.ai/cli/grok-1.0.38-macos-aarch64",
        bytes: 144_397_280,
        sha256: "sha256:a3c5c279339a1294cc99b4d105fe7c67a9f64d20647f2edf131ee72d70f94ed1",
        shape: Shape::Raw,
        member: "",
    },
    Artifact {
        platform: "macos/x86_64",
        url: "https://x.ai/cli/grok-1.0.38-macos-x86_64",
        bytes: 161_685_856,
        sha256: "sha256:1aa859cf7fc6f407a3d3a11c4a1e56b0bed5bf9ca9a5605a8728a25afd938187",
        shape: Shape::Raw,
        member: "",
    },
    Artifact {
        platform: "windows/arm64",
        url: "https://x.ai/cli/grok-1.0.38-windows-aarch64.exe",
        bytes: 132_134_728,
        sha256: "sha256:c63d47ddc3cee388ee4c9069467adc5a521d3d14721534fec0b1a29a2c5f6ee1",
        shape: Shape::Raw,
        member: "",
    },
    Artifact {
        platform: "windows/x86_64",
        url: "https://x.ai/cli/grok-1.0.38-windows-x86_64.exe",
        bytes: 152_750_920,
        sha256: "sha256:a716c252c209629dc315a425c0fd8564c1bedabbdf42034e32e901f4a460c4cb",
        shape: Shape::Raw,
        member: "",
    },
];

/// Grok Build's program, and where its bytes come from.
pub(crate) const SOFTWARE: Software = Software {
    version: "1.0.41",
    command: "grok",
    delivery: Delivery::Artifacts(ARTIFACTS),
    unsupported: &[],
    previous: Some(Previous {
        version: "1.0.38",
        artifacts: PREVIOUS_ARTIFACTS,
    }),
};

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::panic)]

    // Named rather than glob-imported: a product delivered by a package manager
    // has no `Artifact` in scope, and the test is the same text for all seven.
    use harness_runtime::{Delivery, Shape};

    use super::SOFTWARE;

    fn measured() -> serde_json::Value {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../references/grok-baseline.json");
        serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
    }

    #[test]
    fn every_artifact_compiled_in_is_the_one_the_baseline_measured() {
        let block = &measured()["software_artifacts"];
        assert_eq!(block["version"], SOFTWARE.version);
        assert_eq!(block["command"], SOFTWARE.command);

        let Delivery::Artifacts(compiled) = SOFTWARE.delivery else {
            // A product delivered by a package manager has no artifacts, and
            // the baseline must agree that it has none.
            assert_eq!(block["shape"], "manager");
            assert!(block["platforms"].as_object().unwrap().is_empty());
            return;
        };
        let published = block["platforms"].as_object().unwrap();
        assert_eq!(
            compiled.len(),
            published.len(),
            "the table and the baseline disagree on how many platforms exist"
        );
        for artifact in compiled {
            let entry = &published[artifact.platform];
            assert_eq!(entry["url"], artifact.url, "{}", artifact.platform);
            assert_eq!(entry["bytes"], artifact.bytes, "{}", artifact.platform);
            assert_eq!(entry["sha256"], artifact.sha256, "{}", artifact.platform);
            let member = entry.get("member").and_then(serde_json::Value::as_str);
            assert_eq!(
                member.unwrap_or(""),
                artifact.member,
                "{} names a different member",
                artifact.platform
            );
            assert_eq!(
                artifact.shape == Shape::Raw,
                member.is_none(),
                "{} disagrees about whether the bytes are the program",
                artifact.platform
            );
        }
    }

    /// The second pin is the baseline's, or it is absent in both places.
    ///
    /// Asserted from either side rather than only where it exists: a harness
    /// that has never been bumped must compile in `None`, and a build that
    /// dropped the block while the baseline still carried it would otherwise
    /// pass by having nothing to compare.
    #[test]
    fn the_version_this_build_can_move_between_is_the_one_measured_before_it() {
        let baseline = measured();
        let recorded = baseline.get("previous_software_artifacts");
        let Some(earlier) = SOFTWARE.previous else {
            assert!(
                recorded.is_none(),
                "the baseline records a previous release and this build names none"
            );
            return;
        };
        let block = recorded.unwrap_or_else(|| {
            panic!("this build names a previous release the baseline does not record")
        });
        assert_eq!(block["version"], earlier.version);
        assert_ne!(
            earlier.version, SOFTWARE.version,
            "a second pin equal to the first is one version wearing two names"
        );
        let published = block["platforms"].as_object().unwrap();
        assert_eq!(
            earlier.artifacts.len(),
            published.len(),
            "the previous table and the baseline disagree on how many platforms exist"
        );
        for artifact in earlier.artifacts {
            let entry = &published[artifact.platform];
            assert_eq!(entry["url"], artifact.url, "{}", artifact.platform);
            assert_eq!(entry["bytes"], artifact.bytes, "{}", artifact.platform);
            assert_eq!(entry["sha256"], artifact.sha256, "{}", artifact.platform);
        }
    }

    #[test]
    fn a_platform_the_vendor_does_not_publish_is_listed_rather_than_missing() {
        let block = &measured()["software_artifacts"];
        let unpublished: Vec<&str> = block
            .get("unpublished")
            .and_then(serde_json::Value::as_array)
            .map(|entries| {
                entries
                    .iter()
                    .filter_map(serde_json::Value::as_str)
                    .collect()
            })
            .unwrap_or_default();
        assert_eq!(unpublished, SOFTWARE.unsupported);
    }

    #[test]
    fn no_release_calls_a_platform_both_published_and_unpublished() {
        let baseline = measured();
        for name in ["software_artifacts", "previous_software_artifacts"] {
            let Some(block) = baseline.get(name) else {
                continue;
            };
            let published = block["platforms"].as_object().unwrap();
            let unpublished = block
                .get("unpublished")
                .and_then(serde_json::Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(serde_json::Value::as_str);
            for platform in unpublished {
                assert!(
                    !published.contains_key(platform),
                    "{name}: {platform} is both published and unpublished"
                );
            }
        }
    }
}
