<p align="center">
  <img src="https://diegopn.github.io/potoo-site/src/assets/images/language-interpreter-icon.svg?v=1c651ca7" alt="Potoo IDE language interpreter icon" width="120" />
</p>

<h1 align="center">Hedy Interpreter</h1>

<p align="center">A host-driven Rust interpreter for the English Hedy language profile, levels 1–16.</p>

<p align="center">
  <a href="https://github.com/diegopn/potoo-hedy-interpreter/releases">Download</a> ·
  <a href="https://diegopn.github.io/potoo-site/manual/hedy/">Website</a> ·
  <a href="https://diegopn.github.io/potoo-site/manual/hedy/">User manual</a>
</p>

<p align="center">
  <a href="https://www.rust-lang.org/"><img alt="Rust" src="https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white"></a>
  <a href="https://doc.rust-lang.org/cargo/"><img alt="Cargo" src="https://img.shields.io/badge/Cargo-DEA584?style=for-the-badge&logo=rust&logoColor=white"></a>
  <img alt="Rust library" src="https://img.shields.io/badge/Rust_library-4A86CF?style=for-the-badge">
  <img alt="CLI" src="https://img.shields.io/badge/CLI-6A4C93?style=for-the-badge">
</p>

This Rust package is part of the Potoo IDE ecosystem and is separate from the desktop interface. Compatibility with the IDE has not been implemented or verified.

Hedy is a textual, gradual, multilingual programming language. This crate implements the English syntax profile described by the current Hedy teacher manual and English level grammars. It does not include official translations of command words for other locales.

## Language coverage

- **Level 1:** `print`, `ask`, `answer`, `echo` with an optional prefix, turtle movement and turns, and `play`. Bare `forward` defaults to 50 steps; bare `turn` turns right.
- **Level 2:** variables with `is`, stored answers, and `sleep`.
- **Level 3:** lists, random selection, and adding or removing list items.
- **Level 4:** quoted text, turtle `clear`, and `color`.
- **Levels 5–6:** conditionals, `else`, `elif`, and list-membership conditions.
- **Level 7:** arithmetic expressions, decimal values, and `=` assignment.
- **Level 8:** one-line `repeat`; **levels 9–12** add indented blocks, nesting, boolean conditions, list iteration, and parameterless `define`/`call` functions.
- **Level 13:** Python-like expressions, `input()`, comparison operators, and colon-terminated conditionals. The earlier `ask` assignment form is rejected from this level onward.
- **Level 14:** bracketed list literals and indexing, Python-style list and `range` loops.
- **Level 15:** Python-style function definitions, calls, parameters, and return values.
- **Level 16:** `while` loops.
- Runtime values include text, numbers, booleans, and lists. Function locals, recursive calls, short-circuit boolean operators, random selection, and source-located diagnostics are supported.
- The host API supplies input/output, random selection, sleep, turtle drawing, audio, and keyboard state. A statement limit stops unbounded loops and recursion.

The library exposes an explicit level and a caller-supplied `Vocabulary`. Supplying translated command words does not provide a complete or verified localized Hedy profile.

## Host capabilities and limits

The command-line runner connects text input and output to standard input/output and supplies random selection and sleep. It has no graphical turtle, audio device, or keyboard event source; those commands return an explicit error unless an embedding host provides them. The crate interprets Hedy source and does not implement the upstream website, lessons, classroom features, or visual editor.

The parser also accepts `if key is pressed` as an optional host-backed extension. It is not counted as part of the documented level-1–16 language profile.

## Command line

Install Rust 1.93 or later, then run a source file with its Hedy level:

```sh
cargo run --locked -- --level 16 source.hedy
```

Use a level from 1 through 16. The CLI reports source diagnostics before execution and stops after the default statement limit.

## References

- [Official Hedy teacher manual](https://www.hedy.org/for-teachers/manual)
- [Official English level grammars](https://github.com/hedyorg/hedy/tree/main/grammars-Total)
- [Potoo Hedy manual](https://diegopn.github.io/potoo-site/manual/hedy/)

## Support

If this project is useful to you, you can support its continued development:

<p align="left">
  <a href="https://github.com/sponsors/diegopn"><img alt="Become a sponsor" src="https://img.shields.io/badge/Become_a_sponsor-EA4AAA?style=for-the-badge&logo=githubsponsors&logoColor=white"></a>
</p>

## Project policies

- [Code of conduct](CODE_OF_CONDUCT.md)
- [Contributing](CONTRIBUTING.md)
- [Security policy](SECURITY.md)
- [GPL-3.0-or-later license](LICENSE)
