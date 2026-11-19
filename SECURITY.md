# Security Policy

## Reporting a vulnerability

Do not publish vulnerability details in issues, pull requests, discussions, or public comments. If the private vulnerability reporting form is available on this repository's Security page, use it. Otherwise, use the [Potoo contact page](https://diegopn.github.io/#contato) only to request a private reporting channel. Do not include technical details or a proof of concept in that first message.

After arranging a private channel, include the affected commit, Rust version, operating system, reproduction steps, and impact. Remove credentials, personal information, and unrelated data.

For other project concerns, see [CONTRIBUTING.md](CONTRIBUTING.md) and [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md).

## Current scope

This repository parses and interprets Hedy source in a Rust runtime. The interpreter does not expose operating-system commands, native code loading, or direct filesystem access. Programs can still consume host resources through loops, recursion, input/output, sleep, graphics, audio, and keyboard services.

Execution stops after a configurable statement limit (one million by default), and function calls have a separate depth limit. Hosts that enable graphics, audio, keyboard, or other external services should apply their own resource and access limits. Do not assume the language statement limit constrains time spent inside a host callback.
