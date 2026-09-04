# Writing this harness's instruction file

Generated from `references/grok-baseline.json`. Do not edit:
the next render overwrites it, and the baseline is where a correction
belongs.

## Where it goes

`~/.grok/AGENTS.md`

Decided by: https://docs.x.ai/build/overview

## What the record says about it

Exercised 2026-08-28 by running `grok inspect` against a temporary GROK_HOME holding a marker component here; the product reports it back by name.

**Re-asked at 1.0.13 on 2026-08-31**, because the record rested on 1.0.5 and this product moved eight releases in a few hours. The pinned `grok-1.0.13-linux-x86_64` was fetched, its digest checked against the artifact table, and `grok inspect` run against a temporary home holding one marker here -- with a control root, `$HOME/.grok-not-a-root/skills/`, which the same run does not list. The product still reports this surface by name.

## Where the other harnesses keep theirs

| harness | path | shape |
|---|---|---|
| `antigravity` | `config/rules` | directory |
| `claude` | `CLAUDE.md` | file |
| `codex` | `AGENTS.md` | file |
| `cursor` | `rules` | directory |
| **this one** | `AGENTS.md` | file |
| `opencode` | `AGENTS.md` | file |
| `pi` | `AGENTS.md` | file |

**They are not interchangeable, and the difference is not only the
name.** One of the seven takes a *directory* of rules rather than a
single document, so a file moved between the two is not a rename.

**Some products read a neighbour's.** `references/surfaces.md` records
every such cross-read this estate has measured, on the declined rows:
a file written for one product can change what a second one sees, and
removing a setup can change what a third one sees. That is a property
of the products, not of this program, and it is the reason the declined
list is worth reading before writing here.

## Before you write one

- **This file is the floor, not the ceiling.** A repository's own
  instructions sit above it; write what is true everywhere and leave
  the rest to the project.
- **Read it back where the product reads it**, not where the install
  put it. Several of these products resolve a home through an override
  chain, and the two are not always the same directory.

