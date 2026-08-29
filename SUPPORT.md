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

## Using this against a home you already have

**An owned namespace is removed whole.** The table below says what this build
owns; `remove` deletes each of those paths entirely, and a backup slot holds
what was there first. That includes content this build never wrote -- if the
product itself put a key in a configuration file this provider owns, `remove`
takes the file, not the keys this provider added to it.

Measured, with the real product: launching Codex through `launch` and running
`mcp add` writes `~/.codex/config.toml` with an `[mcp_servers.*]` entry; a
later `install` captures that file into a slot and replaces it; a later
`remove` deletes it. The entry is not lost -- `backups` lists the slot as
*before install, setup none*, and restoring it returns the file byte for byte
-- but it is not in the target either.

So: point `--target` at a home you are willing to have managed. `backups
--target <dir>` names every earlier state and which setup each preceded, and
`restore --backup <ref>` returns any of them exactly.

## When conformance says this provider is malformed

`ai-stp provider conformance --protocol-version 3` reports each case by name.
If the one that fails is `provider_info_v3_closed`, with a detail about fields
differing from the closed schema, **check the version of the checker before
suspecting this build**.

The v3 capability schema is compared as an exact field set, so a provider that
declares a field the checker predates is reported as malformed rather than as
newer. `scoped_projection_profiles` (`ADR-0125`) is the field this applies to,
and it is omitted entirely when empty -- so a build that declares no scope
satisfies an older checker by accident, and a build that declares one does not.

Two versions, two different answers, both measured:

| checker | result |
| --- | --- |
| `ai-stp-cli` 0.0.3 | five pass; Codex and Antigravity report `conforms=false`, detail *fields differ from the closed v3 schema* |
| `ai-stp-cli` 0.0.7 | six pass 23 of 23; Codex reports `conforms=false`, detail *a scoped projection profile names an unknown target scope* |

The remaining one is not a defect in this build. `0.0.7` carries the field but
its scope enum is `["project"]` alone, while the provider kit this program
vendors and verifies byte-for-byte -- kit `0.2.4`,
`provider-info.schema.json` -- gives `["project", "user_root"]`. The kit is the
artifact a provider is told to build against, so a build that declares
`user_root` is right by the document it was handed and wrong by the checker
shipped beside it. Raised with the consumer, who owns both.

Which is the general rule this section exists for: **check the version of the
checker before suspecting this build**, and prefer the newest, because an older
one reports a wider failure than the one it found.

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
| `AGENTS.md` | `instruction` | [source](https://docs.x.ai/build/overview; exercised against the pinned 1.0.5 binary) |
| `config.toml` | `setting` | [source](https://docs.x.ai/build/settings; exercised against the pinned 1.0.5 binary) |
| `sandbox.toml` | -- | [source](https://docs.x.ai/build/settings/reference; anchored literal measured in the pinned artifact by scripts/evidence.py) |
| `skills` | `skill` | [source](https://docs.x.ai/build/features/skills-plugins-marketplaces; exercised against the pinned 1.0.5 binary) |
| `agents` | `agent` | [source](https://docs.x.ai/build/features/skills-plugins-marketplaces; exercised against the pinned 1.0.5 binary) |
| `plugins` | `plugin` | [source](https://docs.x.ai/build/features/skills-plugins-marketplaces; exercised against the pinned 1.0.5 binary) |
| `hooks` | `hook` | [source](https://docs.x.ai/build/features/skills-plugins-marketplaces; exercised against the pinned 1.0.5 binary) |
| `workflows` | -- | [source](https://docs.x.ai/build/modes-and-commands; anchored literal measured in the pinned artifact by scripts/evidence.py) |
| `rules` | -- | [source](https://docs.x.ai/build/settings/reference; measured from the pinned 1.0.5 binary's own embedded reference text; exercised against the pinned 1.0.5 binary) |
| `commands` | -- | [source](https://docs.x.ai/build/settings/reference; measured by running the pinned 1.0.5 binary in a contained HOME, 2026-08-28) |
| `personas` | -- | [source](https://docs.x.ai/build/features/subagents; measured in the pinned 1.0.5 binary) |
| `roles` | -- | [source](https://docs.x.ai/build/features/subagents; measured in the pinned 1.0.5 binary) |

A path routing no component kind is owned so a setup can carry it;
nothing compiles a component to it.

### Considered and not owned

Everything named here is left exactly as it was found, like any
other file beside a target.

**`.mcp.json`** -- A project-level compatibility file Grok merges below config.toml, beside ~/.claude.json and .cursor/mcp.json. Not a surface under the Grok home; the real one is config.toml [mcp_servers.<name>]. ([source](https://docs.x.ai/build/features/mcp-servers))

**`Agents.md`** -- Grok Build accepts three spellings of its instruction file -- AGENTS.md, Agents.md and AGENT.md. This provider writes the first and owns only that one: on a case-insensitive filesystem the second is the same file, and on a case-sensitive one owning both would let a target hold two instruction documents that disagree with the product reading one and this provider reporting the other. Which of the three wins where several exist is not documented, so a target holding another spelling is reported rather than resolved. ([source](https://docs.x.ai/build/overview))

**`AGENT.md`** -- The third spelling Grok Build accepts. Not owned, for the reason above. ([source](https://docs.x.ai/build/overview))

**`active_sessions.json`** -- Live session bookkeeping the product writes into its home, alongside a lock file it holds open. Measured 2026-08-28 by installing Grok through this provider's own software lifecycle and running `mcp add` through `launch`. Disclaimed: it churns while the product runs, and copying it into a slot would capture a lock's idea of a running process. ([source](measured through launch; no vendor page names it))

**`active_sessions.lock`** -- The lock beside active_sessions.json, held while the product runs. Measured 2026-08-28. ([source](measured through launch; no vendor page names it))

**`logs`** -- The product's own log directory, created on first run. Measured 2026-08-28. Never read and never copied. ([source](measured through launch; no vendor page names it))

**`docs`** -- Written by the product into its home on first run. Measured 2026-08-28. Not owned: nothing here projects documentation into a product's home, and a directory the product regenerates is not a surface a setup can promise. ([source](measured through launch; no vendor page names it))

**`NDDEV-GROK-PROVIDER.json`** -- This provider's own state file: which setup is applied, the identity it recorded, and which slot reverses the last operation. Written by every operation and excluded from target identity, because counting it would leave a target different from the identity the operation just wrote. Not a projection surface and never ownable as one. ([source](this provider's own contract; no vendor page is involved))

**`.grok-setup-system`** -- This provider's own control directory: the target lock, the backup slots and their payloads. Kept out of the declaration for the same reason as the state file, and recorded here because the declined list is where a reader looks before opening a file to find out what it is. ([source](this provider's own contract; no vendor page is involved))

**`$HOME/.agents/skills`** -- Grok reads the user-level convention root. Its own embedded reference, carried in the pinned 1.0.5 binary, says it scans `.agents/skills/` (and `commands/`) *at each tier* -- and the tier table names User as one of them. Not owned: the root belongs to the convention rather than to this product, and Codex already declares it under `user_root`. A namespace is removed whole, so a second declaration would make either provider's remove take the other's skills. ([source](measured from the pinned artifact, digest verified before reading (grok 1.0.5, the binary's own reference text)))

**`$HOME/.claude/skills`** -- Grok scans Claude Code's own skills directory for compatibility. From the same embedded reference: `skills = true  # scan ~/.claude/skills/ and <cwd>/.claude/skills/`, and the tier table gives `~/.claude/skills/` as User tier, Lowest priority, configurable. It is another product's home and never this provider's to own -- but worth recording, because claude-setup-system owns `skills` there and a remove of the Claude setup changes what Grok sees. ([source](measured from the pinned artifact, digest verified before reading (grok 1.0.5, the binary's own reference text)))

**`auth.json`** -- Authentication credentials. Grok's own embedded reference, carried in the pinned 1.0.5 binary, lists `~/.grok/auth.json` as *Authentication credentials (auto-managed)*, set by `grok login`. Never owned and never captured: a slot holding a product's credentials would put them on disk in a second place, which is a worse outcome than an incomplete restore of files this provider never wrote. Recorded here because the declaration's `never_touch` is checked against this block -- without a row, the check measures nothing. ([source](measured from the pinned 1.0.5 binary's own reference text))

**`lsp.json`** -- *LSP server configuration (user-scoped)*, in the product's own reference. Real configuration and not owned: nothing in the closed set of component kinds describes a language-server list, and this provider does not declare kinds it cannot name. ([source](measured from the pinned 1.0.5 binary's own reference text))

**`pager.toml`** -- *TUI appearance configuration*, in the product's own reference. Not owned for the same reason as `lsp.json` and claude's keymap: appearance is a real surface with no kind to route it. ([source](measured from the pinned 1.0.5 binary's own reference text))

**`memory`** -- *Cross-session memory files and index*, holding a global `MEMORY.md`. What the product has learned across sessions -- a person's accumulated context rather than configuration, and a backup of it would put private text somewhere with a retention policy nobody chose. ([source](measured from the pinned 1.0.5 binary's own reference text))

**`managed_config.toml`** -- An administrator's **signed** policy, and this build owned it until 2026-08-28. Owning it deleted it.

**Measured on the shipped 0.0.11 binary**, against a target holding a managed grok home: `install baseline` removed `managed_config.toml` and `requirements.toml` and **kept** `managed_config.sig.json`, `managed_identity.sig.json` and `managed_config_cache.json`. That is exactly the state the product's own gate refuses -- `xai_grok_config::managed_cache` carries *"refusing session -- the signed is-managed claim requires an authentic policy sidecar and none is present"* and *"refusing session on tamper evidence"*.

The harm was the **split**: owning the policy and not its sidecars left a signature with nothing to verify. `grok setup` writes all of them, server-fetched and signature-checked -- its own `--json` help exists so the command "writes nothing to `~/.grok`".

**The previous reason for owning it was circular.** It read *"owned so a backup returns it byte-exact after an operation touches the home"* -- and the only reason an operation touched it was that it was owned. `restore` does bring it back, measured; the person's next `grok` run happens before that and is refused. Now in `never_touch`: not deleted, not captured into a slot, not hashed into an identity.

**And there is a system layer above this one, outside any target.** `/etc/grok/managed_config.toml` and `/etc/grok/requirements.toml` are path literals in the pinned binary. They get no row of their own because every recorded path here is relative to the target and those are relative to a root this provider never evaluates against -- the guard refused the row when it was tried, correctly. Recorded in this sentence instead, the same way antigravity's `~/.cache/ms-playwright-go` is.

**It bears on the `full-auto` posture.** That setup writes `[ui] permission_mode = always-approve`; the product's own documentation puts the system layer at the top of the chain -- *"requirements.toml / MDM (org-enforced; clamps every config layer below, including the overlay)"*. So on a managed machine the setup installs, verifies and restores cleanly and changes nothing, which is the same defect class as a correct key under a wrong name. ([source](https://docs.x.ai/build/settings/reference; measured by running grok-setup-system 0.0.11 against a managed home, 2026-08-28))

**`requirements.toml`** -- The org-enforced clamp, and it left the owned set with the signed policy above for the same measured reason. The product's own documentation places it at the top of the precedence chain: *"requirements.toml / MDM (org-enforced; clamps every config layer below, including the overlay)"*, and its failure mode is denial rather than degradation -- `requirements.toml unreadable; treating as fail_closed`.

A file whose malformed state locks a person out of their own tool is not one to delete on the way to installing a setup. In `never_touch` now. ([source](https://docs.x.ai/build/settings/reference; measured by running grok-setup-system 0.0.11 against a managed home, 2026-08-28))

**`managed_config.sig.json`** -- The signature over `managed_config.toml`, beside `managed_identity.sig.json` and `managed_config_cache.json`. Unrecorded until 2026-08-28, which is what made the defect above possible: the policy was owned and its proof was not, so an install took one and left the other. All three are in `never_touch` with the policy they belong to. ([source](measured from the pinned 1.0.5 binary: xai_grok_config::signed_policy and managed_cache, 2026-08-28))

**`leader.sock`** -- A unix socket in the configuration home, named by the product's own help: `--leader-socket <PATH>  Use a custom leader socket path instead of the default ~/.grok/leader.sock`. Not a configuration surface and not capturable -- a socket is a special file, and this provider's `copy_tree` refuses those by kind. Recorded so the next reader of this home knows what it is rather than repeating the search. ([source](grok 1.0.5 `setup --help`, measured 2026-08-28))

**`GROK_CONFIG_PATH`** -- Not a path in the target -- an **environment overlay**, recorded here because it changes what the product reads and nothing else in this file would tell a reader so. The pinned binary carries `xai_grok_config::env_overlay` with its own refusals (*"GROK_CONFIG_PATH is unreadable; ignoring the overlay"*, *"...exceeds the max overlay size..."*), and the product's documentation places it: *"GROK_CONFIG / GROK_CONFIG_PATH (tier 4) are config overlays: a merged config layer, not direct-setting environment variables"*.

So a person with it set has configuration this provider never sees, and `status` would report the target managed while the effective configuration differs. Nothing to own; worth knowing. ([source](measured from the pinned 1.0.5 binary, 2026-08-28))

**`bundled`** -- The product's own content, shipped inside the install rather than written by a person: the pinned 1.0.5 binary carries `.grok/bundled/agents` and `.grok/bundled/skills`, and refuses to let anyone remove what is in it -- "Cannot delete bundled personas". Not owned for the same reason as `docs`: a directory the product ships and regenerates is not a surface a setup can promise, and a backup of it would copy the install into the slot. ([source](measured in the pinned 1.0.5 binary; no vendor page names the directory))

## Response

One maintainer. Defects are triaged as time allows; security reports are
acknowledged first.
