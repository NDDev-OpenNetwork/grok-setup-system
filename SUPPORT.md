# Support

## Before opening anything

`--help` states what this build does and does not do. `status --target <dir>
--json` reports what it found in a target without changing it, and its output is
safe to share: it carries identities and digests, never secret values.

## Where to go

| You have | Go to |
| --- | --- |
| A defect | [Issues](../../issues) — use the defect template |
| A question about behaviour | [Issues](../../issues) — a blank issue is fine |
| A vulnerability | [Security advisories](../../security/advisories/new), privately |

Never open a public issue for a vulnerability, and never paste credentials,
tokens, or the contents of a backup slot anywhere in this repository. A backup
slot holds whatever the target held when it was captured.

## What this build does, and what it does not

The software lifecycle — installing, updating and removing the product
itself — is declared and does work. `plan` names the exact bytes offline,
whoever holds the network fetches them, and `apply` verifies and installs
with the network gone.

`launch` is declared. It starts the exact executable a software install
placed under `--prefix`, never a name found on `PATH`, and points the
product at `--target` through the environment variable its own
documentation names.

A provider that advertised an operation it cannot perform would let a caller ask
for something that cannot be honoured, which is worse than not offering it.

All five core operations do work: `backup`, `restore`, `remove`, `install` and
`replace`, both from the local setup catalog and from an `ai-stp-bundle/1`
arriving over the wire.

## What this build owns inside a target

Everything else in the target is a sibling overlay and is preserved
verbatim. Each row cites the vendor page it was read from, and the same
table is bound to the declaration by a test, so this cannot drift from
what `provider-info` publishes.

Configuration home as the product documents it: `~/.grok`.

| Path | Component kinds routed here | Decided by |
| --- | --- | --- |
| `AGENTS.md` | `instruction` | [source](https://docs.x.ai/build/overview) |
| `config.toml` | `setting` | [source](https://docs.x.ai/build/settings) |
| `managed_config.toml` | -- | [source](https://docs.x.ai/build/settings/reference) |
| `requirements.toml` | -- | [source](https://docs.x.ai/build/settings/reference) |
| `sandbox.toml` | -- | [source](https://docs.x.ai/build/settings/reference) |
| `skills` | `skill` | [source](https://docs.x.ai/build/features/skills-plugins-marketplaces) |
| `agents` | `agent` | [source](https://docs.x.ai/build/features/skills-plugins-marketplaces) |
| `plugins` | `plugin` | [source](https://docs.x.ai/build/features/skills-plugins-marketplaces) |
| `hooks` | `hook` | [source](https://docs.x.ai/build/features/skills-plugins-marketplaces) |
| `workflows` | -- | [source](https://docs.x.ai/build/modes-and-commands) |

A path routing no component kind is owned so a setup can carry it;
nothing compiles a component to it.

### Considered and not owned

Everything named here is left exactly as it was found, like any
other file beside a target.

**`commands`** -- Grok Build has no commands directory. Its slash commands are skills: a user-invocable skill surfaces as /<skill-name>, qualified /local:<name> on collision. The consumer's rule command -> commands was claude-code's row copied one product down and was removed from ai-stp on 2026-08-27. ([source](https://docs.x.ai/build/modes-and-commands))

**`.mcp.json`** -- A project-level compatibility file Grok merges below config.toml, beside ~/.claude.json and .cursor/mcp.json. Not a surface under the Grok home; the real one is config.toml [mcp_servers.<name>]. ([source](https://docs.x.ai/build/features/mcp-servers))

## Response

One maintainer. Defects are triaged as time allows; security reports are
acknowledged first.
