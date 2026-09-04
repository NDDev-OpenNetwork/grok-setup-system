# Writing an agent for this harness

Generated from the vendor's own reference and the pinned binary. Do not edit: the next render overwrites it, and a correction belongs in the source this file is derived from.

**Where it goes**: `~/.grok/agents/<name>.md`

**Decided by**: measured in the 1.0.5 binary's own embedded reference

**How it runs**: Grok resolves the agent by name; plugin agents are `plugin-name:agent-name`.

> **Measured, not specified.** The fields below were read out of the > product rather than off a page. They are what this build does, not > a promise about what the next one will do. Where this matters, ask > the binary.

## Frontmatter

| field | required | what it does |
|---|---|---|
| `mcpInheritance` | no | `all`, `none`, `named` or `except`. |
| `mcpServers` | no | Servers for this agent. |
| `permissionMode` | no | Includes `bypassPermissions`. |
| `hooks` | no | Hooks this agent registers. |

## What bites

- The vendor's subagents page names the directories and does not list the frontmatter, so the fields above are measured rather than specified. Ask the binary before relying on one that is not here.
- **A plugin's agent is not allowed the same frontmatter as yours.** The product's own reference: plugin agent frontmatter cannot declare `mcpServers` or hooks, and cannot set `permissionMode: bypassPermissions`. A component that works from `agents/` can be refused once it ships inside a plugin.
- Personas are a second, separate mechanism: `.toml` files under `personas/`, applied during subagent resolution, and behavioural only. They are owned by this provider and route no kind -- there is no word for a behavioural overlay in the kind set, and declaring one would promise a rollback for something no consumer can install. `roles/` sits beside it on the same footing.

## The same file on the other harnesses

Generated from the same rows as the section above, for every harness in this estate that routes this kind. `—` means the product's own reference does not name the field, and **dropped** means it names it as one it accepts and does not act on.

| field | `claude` | `grok` | `opencode` | `antigravity` |
|---|---|---|---|---|
| `name` | **required** | — | yes | **required** |
| `description` | **required** | — | **required** | **required** |
| `tools` | yes | — | yes | yes |
| `disallowedTools` | yes | — | — | — |
| `model` | yes | — | yes | yes |
| `permissionMode` | yes | yes | — | — |
| `maxTurns` | yes | — | — | — |
| `skills` | yes | — | — | yes |
| `mcpServers` | yes | yes | — | yes |
| `hooks` | yes | yes | — | — |
| `memory` | yes | — | — | — |
| `background` | yes | — | — | — |
| `effort` | yes | — | — | — |
| `isolation` | yes | — | — | — |
| `color` | yes | — | yes | — |
| `initialPrompt` | yes | — | — | — |
| `mcpInheritance` | — | yes | — | — |
| `mode` | — | — | yes | — |
| `temperature` | — | — | yes | — |
| `top_p` | — | — | yes | — |
| `permission` | — | — | yes | — |
| `disable` | — | — | yes | — |
| `mainAgent` | — | — | — | yes |
| `subagent` | — | — | — | yes |
| `commandExecutionPolicy` | — | — | — | yes |

**The part that does not, and says nothing when it does not**: a field absent from a column is not rejected there -- it is read past. Nothing warns, no run fails, and the component behaves differently with the same bytes. Where the field was carrying a restriction, the restriction is simply gone. Check the column before relying on one.

## Before you ship one

- **The surface is declared, so the component is a promise.** Every kind   this provider declares is a promise of a rollback. A component written   to a path the declaration does not carry is installed by nobody and   removed by nobody.
- **Name it once.** Where the product derives identity from the directory   or the filename, the frontmatter `name` is either redundant or a second   place to be wrong. Keep them equal.
- **Read it back.** After an install, look at the file where the product   reads it, not at the step that put it there.
