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

## The owned region inside it

A setup's `instruction` component does not replace `AGENTS.md`;
the consumer's text lives inside one marked region spliced between
`:::begin-ai-stp` and `:::end-ai-stp` -- visible markers, because at
least one product strips HTML comments. Every byte outside the pair
is preserved, and re-applying identical bytes is a no-op.

- **`patch_instruction_region`** is the operation that changes it,
  carrying the new section under `--instruction-section` -- exactly
  one ordered marker pair. Planning records the file's observed
  digest and presence; apply re-reads both and refuses a drifted
  file rather than splicing into text it did not measure.
- **`detach_instruction_region`** removes the marked section and
  nothing else. A file that held only the region is removed with
  it; a file that never had one detaches to itself, so a repeat
  is a no-op rather than an error.
- **Both refuse a `--target-scope`.** The region lives at the
  target root, a scoped request cannot address it, and `status`
  reports `instruction_region: null` under a scoped measure.
- **It is not whole-setup payload.** Install, replace, remove and
  reset preserve or detach the region through the kernel's two
  hooks instead of treating it as text they are free to empty.
- **Ambiguous markers refuse at plan time.** An unpaired or
  out-of-order marker fails before anything is written -- never
  as a mid-apply surprise.

