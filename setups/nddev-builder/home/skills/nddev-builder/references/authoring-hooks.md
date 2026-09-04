# Writing a hook for this harness

Generated from the vendor's own reference and the pinned binary. Do not edit: the next render overwrites it, and a correction belongs in the source this file is derived from.

**Where it goes**: `~/.grok/hooks/<name>.json`

**Decided by**: https://docs.x.ai/build/features/hooks

**How it runs**: Grok fires each `*.json` file's hooks on their events.

## Frontmatter

| field | required | what it does |
|---|---|---|
| `matcher` | no | Regex against tool names. Optional. |
| `type` | no | `command` or `http`. |
| `command` | no | Shell command, for `type: command`. |
| `url` | no | Endpoint, for `type: http`. |
| `timeout` | no | Seconds. Default 5. |

## Events

`SessionStart`  `SessionEnd`  `UserPromptSubmit`  `PreToolUse`  `PostToolUse`  `PostToolUseFailure`  `PermissionDenied`  `Stop`  `StopFailure`  `Notification`  `SubagentStart`  `SubagentStop`  `PreCompact`  `PostCompact`

## What bites

- This surface is a **directory of `*.json` files**, not one file. The other hook-routing harness in this estate is the opposite.
- The script contract: stdin carries `hookEventName`, `sessionId`, `cwd`, `workspaceRoot`, and for tool events `toolName` and `toolInput`. The environment carries `GROK_HOOK_EVENT`, `GROK_HOOK_NAME`, `GROK_SESSION_ID` and `GROK_WORKSPACE_ROOT`.
- **`PreToolUse` fails open.** Exit 0 allows and exit 2 denies, with `{"decision": "deny", "reason": "..."}` on stdout -- but a timeout, a crash or malformed output lets the tool proceed. A hook written as a control is not one unless it is also correct when it breaks: a guard that fails open denies nothing on the day it matters. The default timeout here is 5 seconds, which is the most likely way to reach that path by accident.

## The same file on the other harnesses

Generated from the same rows as the section above, for every harness in this estate that routes this kind. `—` means the product's own reference does not name the field, and **dropped** means it names it as one it accepts and does not act on.

| field | `codex` | `grok` | `antigravity` |
|---|---|---|---|
| `matcher` | yes | yes | — |
| `type` | yes | yes | yes |
| `command` | yes | yes | **required** |
| `commandWindows` | yes | — | — |
| `timeout` | yes | yes | yes |
| `statusMessage` | yes | — | — |
| `additionalContextLimit` | yes | — | — |
| `async` | yes | — | — |
| `server` | yes | — | — |
| `tool` | yes | — | — |
| `input` | yes | — | — |
| `url` | — | yes | — |

**The part that travels**: `type`, `command`, `timeout`. Everything else is a bet on one product.

**The part that does not, and says nothing when it does not**: a field absent from a column is not rejected there -- it is read past. Nothing warns, no run fails, and the component behaves differently with the same bytes. Where the field was carrying a restriction, the restriction is simply gone. Check the column before relying on one.

## Before you ship one

- **The surface is declared, so the component is a promise.** Every kind   this provider declares is a promise of a rollback. A component written   to a path the declaration does not carry is installed by nobody and   removed by nobody.
- **Name it once.** Where the product derives identity from the directory   or the filename, the frontmatter `name` is either redundant or a second   place to be wrong. Keep them equal.
- **Read it back.** After an install, look at the file where the product   reads it, not at the step that put it there.
