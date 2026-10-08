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
