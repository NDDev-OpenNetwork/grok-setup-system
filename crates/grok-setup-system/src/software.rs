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
        url: "https://x.ai/cli/grok-1.0.21-linux-aarch64",
        bytes: 130_274_504,
        sha256: "sha256:8a629e703cb08856fe7d837446bc225bcbb51c85f86fff0b0e0628cd89138582",
        shape: Shape::Raw,
        member: "",
    },
    Artifact {
        platform: "linux/x86_64",
        url: "https://x.ai/cli/grok-1.0.21-linux-x86_64",
        bytes: 156_218_560,
        sha256: "sha256:3a0bd1111628768b91c4ac550034565448309fe9ec2cb13b71dea90910dc9d98",
        shape: Shape::Raw,
        member: "",
    },
    Artifact {
        platform: "macos/arm64",
        url: "https://x.ai/cli/grok-1.0.21-macos-aarch64",
        bytes: 136_467_248,
        sha256: "sha256:daac3e1cc56771b94605c39cc091b7d3c3d4ef0aea3a45658250200aa22e936b",
        shape: Shape::Raw,
        member: "",
    },
    Artifact {
        platform: "macos/x86_64",
        url: "https://x.ai/cli/grok-1.0.21-macos-x86_64",
        bytes: 153_044_752,
        sha256: "sha256:bc275df8242ad530f88c0f110f14ff70535e2ad0ed64610b72d4d60c18183f3d",
        shape: Shape::Raw,
        member: "",
    },
    Artifact {
        platform: "windows/arm64",
        url: "https://x.ai/cli/grok-1.0.21-windows-aarch64.exe",
        bytes: 124_267_336,
        sha256: "sha256:1063015fab6cc43a8a4f3a4264e8f04477211cc6c507d3272d837f03a0543f41",
        shape: Shape::Raw,
        member: "",
    },
    Artifact {
        platform: "windows/x86_64",
        url: "https://x.ai/cli/grok-1.0.21-windows-x86_64.exe",
        bytes: 143_703_880,
        sha256: "sha256:59488238420820f455c59de3dab038a6561bc34af7cb7d961ffcd269f4d7d4e4",
        shape: Shape::Raw,
        member: "",
    },
];

/// The artifacts 1.0.18 was published as, kept so
/// `software_update` has a version to move from and `rollback` a tree to
/// return to. Measured from bytes when it was the current pin.
pub(crate) const PREVIOUS_ARTIFACTS: &[Artifact] = &[
    Artifact {
        platform: "linux/arm64",
        url: "https://x.ai/cli/grok-1.0.18-linux-aarch64",
        bytes: 129_675_416,
        sha256: "sha256:e5c96058b6f88b2e9ae7d61401e80ac6bcbcaf1be5681eae19e36a2b8aff121d",
        shape: Shape::Raw,
        member: "",
    },
    Artifact {
        platform: "linux/x86_64",
        url: "https://x.ai/cli/grok-1.0.18-linux-x86_64",
        bytes: 155_510_304,
        sha256: "sha256:42f6efb496feafea3e8807b75852cc1f792cba0bd0be8ecbd1c9f7ed5816a7f0",
        shape: Shape::Raw,
        member: "",
    },
    Artifact {
        platform: "macos/arm64",
        url: "https://x.ai/cli/grok-1.0.18-macos-aarch64",
        bytes: 135_828_368,
        sha256: "sha256:99b77202286face91f27f97caca0590377b36300ccca99ff18e730228f7a5ca3",
        shape: Shape::Raw,
        member: "",
    },
    Artifact {
        platform: "macos/x86_64",
        url: "https://x.ai/cli/grok-1.0.18-macos-x86_64",
        bytes: 152_347_968,
        sha256: "sha256:b6b682eb551e503dfb80368e46c2129a74ba51af598692ba8f4c59202653fd2a",
        shape: Shape::Raw,
        member: "",
    },
    Artifact {
        platform: "windows/arm64",
        url: "https://x.ai/cli/grok-1.0.18-windows-aarch64.exe",
        bytes: 123_747_656,
        sha256: "sha256:d4d38c41e7a18979ca0c591244c03d4c0909001d4a288f67d6e9e92d5ac0eca9",
        shape: Shape::Raw,
        member: "",
    },
    Artifact {
        platform: "windows/x86_64",
        url: "https://x.ai/cli/grok-1.0.18-windows-x86_64.exe",
        bytes: 143_124_808,
        sha256: "sha256:c407a08a5b987c528c8c515394ce839319cbe69a7c0f73e322348038754387eb",
        shape: Shape::Raw,
        member: "",
    },
];

/// Grok Build's program, and where its bytes come from.
pub(crate) const SOFTWARE: Software = Software {
    version: "1.0.21",
    command: "grok",
    delivery: Delivery::Artifacts(ARTIFACTS),
    unsupported: &[],
    previous: Some(Previous {
        version: "1.0.18",
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
