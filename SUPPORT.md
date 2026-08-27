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

## What `status` reports, and what it does not

`state` answers **who manages this target**, and never *whether a setup is
installed*. Three values, and the distinction matters most for the fourth
situation, which is not a fourth value:

| | |
| --- | --- |
| `missing` | the directory is empty |
| `unmanaged` | it holds content, none of it this provider's |
| `managed` | this provider's state file is present and current |

`missing` used to be looser -- it asked whether this provider owned anything,
so a directory full of another product's files reported `missing`. A consumer
reads this to decide what it is looking at, and being told a populated
directory is empty invites it to treat the place as free. Emptiness is about
the directory, not about us.

**After a `remove`, `state` stays `managed`, and that is the honest answer.**
The setup is gone -- no file a product reads survives it -- but the control
directory and a backup slot remain, and that slot is what makes the removal
reversible: `restore` brings the setup back. A target reported as `missing`
while a restore is pending would be a lie in the direction that costs someone
their data.

Whether a setup is installed is carried by `setup_stable_id`, which is `null`
exactly when none is. That is the field to test, not this word.
`target_identity_digest` corroborates it -- after a remove it is the digest of
an empty tree -- but the field is the direct answer and the digest is not.

## The network, stated exactly

**This artifact does not link the network, and no local phase can spawn
anything that could.** Two lints hold it rather than a promise: `std::net` is
refused outright, and `std::process::Command` is refused everywhere but two
named places -- the `launch` command, which is declared in `provider-info` and
absent from builds that do not declare it, and a lifecycle probe that drives
this binary's own executable. Adding a `tar` shell-out to ordinary code fails
the build with *only `launch` may spawn, and it is declared*. Every crate that
may be linked is named in `deny.toml`, so a transitive dependency cannot arrive
unread.

Those are claims about the source, and a lint can be wrong, bypassed, or simply
disbelieved. So `ci` reads the shipped binary too: a `boundary` job asks the
import table of the artifact this build produces whether any network symbol is
present, and whether a build declaring no `launch` imports anything that could
spawn. You can run it yourself against a downloaded release --
`nm -D --undefined-only <binary>` on Linux, `nm -u` on macOS -- and it needs no
part of this repository to be trusted.

**What that does not buy, said plainly because the stronger claim is the
tempting one.** This is a dynamically linked program: it imports `syscall` from
libc like any other, so no property of the binary can prove a socket is
unreachable to code that is determined to open one. What is proven is narrower
and still worth having: no code path here reaches for the network, none can be
added without the build refusing, and no local phase can hand the job to a
child process. If your threat model needs the guarantee rather than the
absence, run `plan` and `apply` under whatever sandbox you already trust; both
phases are offline by design, and `apply` verifies the digests it was given
with the network gone.

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

**`Agents.md`** -- Grok Build accepts three spellings of its instruction file -- AGENTS.md, Agents.md and AGENT.md. This provider writes the first and owns only that one: on a case-insensitive filesystem the second is the same file, and on a case-sensitive one owning both would let a target hold two instruction documents that disagree with the product reading one and this provider reporting the other. Which of the three wins where several exist is not documented, so a target holding another spelling is reported rather than resolved. ([source](https://docs.x.ai/build/overview))

**`AGENT.md`** -- The third spelling Grok Build accepts. Not owned, for the reason above. ([source](https://docs.x.ai/build/overview))

## Response

One maintainer. Defects are triaged as time allows; security reports are
acknowledged first.
