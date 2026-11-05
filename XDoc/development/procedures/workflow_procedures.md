Workflow procedures: repeatable operational workflows, such as testing, debugging, releases, and Git checkpoints.

OUR SACRED WORK FLOW

Edit
  ↓
Inspect
  ↓
cargo fmt -- --check

cargo fmt
  ↓
cargo check/test/clippy
  ↓
git diff
  ↓
git status
  ↓
YOU decide what gets staged
  ↓
YOU approve the commit
  ↓
push

///

git status
cargo test
git diff
git add .
git commit -m "Describe the change"
git push
git status

//

KNOWN GOOD BASELINE
        ↓
remove duplicate modules
        ↓
cargo test 🟢
        ↓
cargo fmt 🟢
        ↓
commit
        ↓
push
        ↓
KNOWN GOOD REMOTE BASELINE

//

                 YOUR MACHINE
                     │
              ┌──────▼──────┐
              │ git commit  │
              │  checkpoint │
              └──────┬──────┘
                     │
                     ▼
                  GITHUB
              ┌─────────────┐
              │ git push    │
              │ remote copy │
              └─────────────┘

And our sacred production pipeline:

RESEARCH
   ↓
RAW MATERIAL
   ↓
MATHEMATICAL DEFINITION
   ↓
RUST IMPLEMENTATION
   ↓
CLI EXPERIMENT
   ↓
TESTING
   ↓
XDOC
   ↓
COMMIT
   ↓
INGOT ACQUIRED

cargo test //

// section

1. Put a gate on the repo first

Before fixing anything, make “green” mean something:

cargo fmt --check, cargo clippy -- -D warnings, cargo test
A GitHub Actions workflow running those on every push. It’s about 15 lines of YAML, and then the “known good baseline” in your workflow doc is enforced by a machine, not by memory.