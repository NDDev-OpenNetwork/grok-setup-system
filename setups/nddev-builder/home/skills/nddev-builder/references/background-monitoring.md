# Background Work and Monitoring

Grok Build 1.0.18 carries these mechanisms in its own embedded user guide. They
are runtime tools, not new setup component kinds or paths.

## Choose the native mechanism

- Use a background terminal command for a finite process whose output is read
  later by task ID.
- Use a background subagent for bounded independent reasoning or implementation.
  `general-purpose` may edit and execute; `explore` and `plan` are intentionally
  narrower. Prefer worktree isolation for an editing subagent when shared files
  would collide.
- Use `send_subagent_message` to steer an active child rather than killing and
  replacing it. Use the queued form only when the message must become a later
  protected turn.
- Use `monitor` for a real-time line event stream from a long-lived command.
  Use `/loop` for periodic re-runs. They answer different timing questions.
- Use the bounded multi-task wait to join known task IDs. A wait returning while
  work is still active is not a failure and is not permission to kill it.
- Use the dashboard to inspect top-level sessions, subagents and background
  tasks. It is an observation surface, not a replacement for reading the final
  task result.

## Full-auto posture

The setup enables active-agent messages, subagent worktree snapshots, codebase
indexing, the dashboard, MCP liveness/status/config watchers and the goal tool.
The `monitor` and background-task tools are built in and require no invented
configuration key. Permission prompts stay disabled and the sandbox stays off;
monitoring work does not narrow the environment it observes.

## Evidence and limits

The source is the documentation embedded by the digest-verified 1.0.18 binary:
`16-subagents.md`, `20-background-tasks.md`, `23-dashboard.md`,
`24-monitoring-usage.md` and `26-config-reference.md`. The same binary was run
with the exact full-auto `config.toml`; `inspect --json` named it as the active
user layer and stderr was empty. This proves the file parses and the documented
keys are current. It does not claim a monitor ran against a live signed-in
session; that requires product credentials and is a separate runtime evidence
slice.
