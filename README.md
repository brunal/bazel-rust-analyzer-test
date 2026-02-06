This repository showcases a minimal setup of rust with bazel, making
rust-analyzer work with vscode.

For rust-analyer setup, 2 files are important, located in .vscode:
* tasks.json will generate rust-project.json when opening the folder. One
  downside is that this only happens when opening up the project. It might be
  possible to trigger it on other events (e.g. BUILD.bazel modifications) using
  extensions.
* settings.json gives a replacement for "cargo check". This provides
  diagnostics like "unused symbol". It runs a proper build, which is more than
  what "cargo check" does. That's not too bad as artifacts are cached. One
  issue I have is that when there is a single diagnostic, it lingers in VSCode
  after fixing it. The one way to make it go away is to have another diagnostic
  pop up...

Here's a capture of the "lingering diagnostic" issue:

[[vscode-rust-analyzer-diagnostic-lingering.webm]]

