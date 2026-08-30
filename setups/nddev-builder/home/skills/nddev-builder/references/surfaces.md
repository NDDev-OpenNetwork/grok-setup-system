# What This Harness Owns

Generated from `references/grok-baseline.json` by
`tools/build_nddev_builder.py`. Do not edit: the next render overwrites
it, and the baseline is where a correction belongs.

Every row below was decided by a source, and the source is named. Where
this file and the binary disagree, the binary is right -- ask it with
`grok-setup-system provider-info`.

**Configuration home**: `~/.grok`
**Environment override**: `GROK_HOME`

## The configuration file

`config.toml` is **toml**, and the parser accepts comments.

TOML, `#` comments in the grammar. No JSON schema.

**A value written here is not necessarily the effective one.** `requirements.toml` and the system pair at `/etc/grok` clamp every key below them -- the product's own documentation puts them at the top of the chain: *"requirements.toml / MDM (org-enforced; clamps every config layer below, including the overlay)"*. So a `full-auto` posture can install, verify and restore cleanly on a managed machine and change nothing.

## Owned surfaces

| path | kinds | shape | decided by | exercised by |
|---|---|---|---|---|
| `AGENTS.md` | instruction | file | <https://docs.x.ai/build/overview> | **ran it** |
| `config.toml` | setting | file | <https://docs.x.ai/build/settings> | **ran it** |
| `sandbox.toml` | *(routes no kind)* | file | <https://docs.x.ai/build/settings/reference> | read its bytes |
| `skills` | skill | directory | <https://docs.x.ai/build/features/skills-plugins-marketplaces> | **ran it** |
| `agents` | agent | directory | <https://docs.x.ai/build/features/skills-plugins-marketplaces> | **ran it** |
| `plugins` | plugin | directory | <https://docs.x.ai/build/features/skills-plugins-marketplaces> | **ran it** |
| `hooks` | hook | directory | <https://docs.x.ai/build/features/skills-plugins-marketplaces> | **ran it** |
| `workflows` | *(routes no kind)* | directory | <https://docs.x.ai/build/modes-and-commands> | read its bytes |
| `rules` | *(routes no kind)* | directory | <https://docs.x.ai/build/settings/reference> | **ran it** |
| `commands` | *(routes no kind)* | directory | <https://docs.x.ai/build/settings/reference> | **ran it** |
| `personas` | *(routes no kind)* | directory | <https://docs.x.ai/build/features/subagents> | read its bytes |
| `roles` | *(routes no kind)* | directory | <https://docs.x.ai/build/features/subagents> | read its bytes |

**A citation is not a measurement.** `decided by` says where a row came from; `exercised by` says whether anybody made the product demonstrate it. Where a row records no method the answer is a page and nothing else, because absence of a record of measurement is not evidence of measurement.

Here that is **8 run**, **4 read from the product's own bytes**, and **0 resting on a page alone**. The last number is the one worth acting on: a row in it is not wrong, it is untested, and the two are indistinguishable from here.

A surface that routes no kind is owned deliberately: a backup captures
it and a restore returns it, and no component is routed there because
the kind it would carry already routes somewhere else. One kind on two
surfaces makes a consumer's route ambiguous, and the guard in
`harness_runtime::surfaces` refuses it by name.

## A second target: `target_scope: user_root`

Rooted at `~/.agents`, which is **not** this product's configuration
home. A consumer reaches it by naming the scope on the request, and
every path below is relative to that root rather than to the home
above -- writing the root into the path again would nest it twice.

| path | routes | shape | decided by | exercised by |
| --- | --- | --- | --- | --- |
| `skills` | skill | directory | measured by running the pinned 1.0.13 binary with `grok inspect` against a temporary HOME, 2026-08-29 | **ran it** |

**Under a scope the namespace is the permission and the recorded
files are the inventory.** A root like this one is read by several
products at once, so `remove`, the capture and a restore all act on
the files this provider recorded writing -- never on the namespace
whole, which would take or revert a neighbour's work.

## Considered and not owned

20 rows. Each records what was searched, so the next reader does not repeat the search:

- **`.mcp.json`** — A project-level compatibility file Grok merges below config.toml, beside ~/.claude.json and .cursor/mcp.json. Not a surface under the Grok home; the real one is config.toml [mcp_servers.<name>].
- **`Agents.md`** — Grok Build accepts three spellings of its instruction file -- AGENTS.md, Agents.md and AGENT.md. This provider writes the first and owns only that one: on a case-insensitive filesystem the second is the same file, and on a case-sensitive one owning both would let a target hold two instruction documents that disagree with the product reading one and this provider reporting the other. Which of the three wins where several exist is not documented, so a target holding another spelling is reported rather than resolved.
- **`AGENT.md`** — The third spelling Grok Build accepts. Not owned, for the reason above.
- **`active_sessions.json`** — Live session bookkeeping the product writes into its home, alongside a lock file it holds open. Measured 2026-08-28 by installing Grok through this provider's own software lifecycle and running `mcp add` through `launch`. Disclaimed: it churns while the product runs, and copying it into a slot would capture a lock's idea of a running process.
- **`active_sessions.lock`** — The lock beside active_sessions.json, held while the product runs. Measured 2026-08-28.
- **`logs`** — The product's own log directory, created on first run. Measured 2026-08-28. Never read and never copied.
- **`docs`** — Written by the product into its home on first run. Measured 2026-08-28. Not owned: nothing here projects documentation into a product's home, and a directory the product regenerates is not a surface a setup can promise.
- **`NDDEV-GROK-PROVIDER.json`** — This provider's own state file: which setup is applied, the identity it recorded, and which slot reverses the last operation. Written by every operation and excluded from target identity, because counting it would leave a target different from the identity the operation just wrote. Not a projection surface and never ownable as one.
- **`.grok-setup-system`** — This provider's own control directory: the target lock, the backup slots and their payloads. Kept out of the declaration for the same reason as the state file, and recorded here because the declined list is where a reader looks before opening a file to find out what it is.
- **`$HOME/.claude/skills`** — Grok scans Claude Code's own skills directory for compatibility. From the same embedded reference: `skills = true  # scan ~/.claude/skills/ and <cwd>/.claude/skills/`, and the tier table gives `~/.claude/skills/` as User tier, Lowest priority, configurable. It is another product's home and never this provider's to own -- but worth recording, because claude-setup-system owns `skills` there and a remove of the Claude setup changes what Grok sees.
- **`auth.json`** — Authentication credentials. Grok's own embedded reference, carried in the 1.0.5 binary, lists `~/.grok/auth.json` as *Authentication credentials (auto-managed)*, set by `grok login`. Never owned and never captured: a slot holding a product's credentials would put them on disk in a second place, which is a worse outcome than an incomplete restore of files this provider never wrote. Recorded here because the declaration's `never_touch` is checked against this block -- without a row, the check measures nothing.
- **`lsp.json`** — *LSP server configuration (user-scoped)*, in the product's own reference. Real configuration and not owned: nothing in the closed set of component kinds describes a language-server list, and this provider does not declare kinds it cannot name.
- **`pager.toml`** — *TUI appearance configuration*, in the product's own reference. Not owned for the same reason as `lsp.json` and claude's keymap: appearance is a real surface with no kind to route it.
- **`memory`** — *Cross-session memory files and index*, holding a global `MEMORY.md`. What the product has learned across sessions -- a person's accumulated context rather than configuration, and a backup of it would put private text somewhere with a retention policy nobody chose.
- **`managed_config.toml`** — An administrator's **signed** policy, and this build owned it until 2026-08-28. Owning it deleted it.
- **`requirements.toml`** — The org-enforced clamp, and it left the owned set with the signed policy above for the same measured reason. The product's own documentation places it at the top of the precedence chain: *"requirements.toml / MDM (org-enforced; clamps every config layer below, including the overlay)"*, and its failure mode is denial rather than degradation -- `requirements.toml unreadable; treating as fail_closed`.
- **`managed_config.sig.json`** — The signature over `managed_config.toml`, beside `managed_identity.sig.json` and `managed_config_cache.json`. Unrecorded until 2026-08-28, which is what made the defect above possible: the policy was owned and its proof was not, so an install took one and left the other. All three are in `never_touch` with the policy they belong to.
- **`leader.sock`** — A unix socket in the configuration home, named by the product's own help: `--leader-socket <PATH>  Use a custom leader socket path instead of the default ~/.grok/leader.sock`. Not a configuration surface and not capturable -- a socket is a special file, and this provider's `copy_tree` refuses those by kind. Recorded so the next reader of this home knows what it is rather than repeating the search.
- **`GROK_CONFIG_PATH`** — Not a path in the target -- an **environment overlay**, recorded here because it changes what the product reads and nothing else in this file would tell a reader so. The pinned binary carries `xai_grok_config::env_overlay` with its own refusals (*"GROK_CONFIG_PATH is unreadable; ignoring the overlay"*, *"...exceeds the max overlay size..."*), and the product's documentation places it: *"GROK_CONFIG / GROK_CONFIG_PATH (tier 4) are config overlays: a merged config layer, not direct-setting environment variables"*.
- **`bundled`** — The product's own content, shipped inside the install rather than written by a person: the 1.0.5 binary carries `.grok/bundled/agents` and `.grok/bundled/skills`, and refuses to let anyone remove what is in it -- "Cannot delete bundled personas". Not owned for the same reason as `docs`: a directory the product ships and regenerates is not a surface a setup can promise, and a backup of it would copy the install into the slot.
