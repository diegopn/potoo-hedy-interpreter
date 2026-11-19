# Hedy reference coverage

This crate targets the English profile in the current official Hedy teacher manual, levels 1–16, and the corresponding English level grammars. Hedy changes syntax gradually, so each level is treated as its own language profile. This table records implemented syntax and semantics, not parity with the website, lessons, editor, or classroom services.

| Level | Concepts and syntax in the selected reference | Crate coverage |
| --- | --- | --- |
| 1 | `print`, `ask` / `answer`, `echo`, `forward`, `turn`, and `play` | Implemented. The documented default turtle distances/turns and host callbacks are applied. |
| 2 | `is` assignments, stored answers, `sleep`, numeric `turn` | Implemented; level 1 `echo` is rejected. |
| 3 | Lists, `at random`, `add to`, `remove from` | Implemented with host-provided random selection. Whole-list printing is rejected. |
| 4 | Quoted text, turtle `clear`, and `color` | Implemented. |
| 5 | `if` / `else` and equality or membership conditions | Implemented using the level's Hedy syntax and indentation profile. |
| 6 | `elif` and negated list-membership conditions | Implemented. |
| 7 | Decimal values, arithmetic expressions, and `=` assignment | Implemented. |
| 8 | One-line `repeat ... times` | Implemented; the level-8 form is not accepted at later levels. |
| 9 | Indented multi-line blocks | Implemented; nested blocks remain unavailable until level 10. |
| 10 | Nested blocks and `and` / `or` conditions | Implemented with short-circuit evaluation. |
| 11 | `for` loops over lists | Implemented. |
| 12 | Hedy `define` / `call` functions | Implemented as parameterless functions; arguments and parameters are rejected until level 15. |
| 13 | Gradual Python-style syntax, `input()`, colon-terminated `if` / `elif` / `else`, and comparison operators | Implemented for the documented forms; Hedy `define`/`call`, list syntax, and `repeat` are not carried forward into this level. |
| 14 | Python-style list literals, indexing, list loops, and `range` loops | Implemented, including required colons on `for` headers. Range stop is exclusive; indexing uses the runtime's Python-like zero-based and negative-index behavior. |
| 15 | Python-style `def`, arguments, calls, and `return` | Implemented, including required colons on function definitions and recursion. |
| 16 | `while` loops | Implemented. |

## Boundaries

- Only the English vocabulary and the selected 1–16 progression are claimed. The `Vocabulary` API can substitute command words, but that alone does not establish compatibility with a translated Hedy profile.
- The official language-design wiki, last edited in 2024, describes an earlier 18-level progression where levels 17–18 add colons and parentheses. The current teacher manual calls level 16 the last level and uses a different progression; this crate follows that manual, so it does not combine the older levels 17–18 syntax with the current profile.
- Text input/output, random selection, sleep, turtle drawing, audio, and keyboard state are host capabilities. The CLI provides text I/O, random selection, and sleep; it has no turtle surface, audio device, or keyboard event source.
- The interpreter does not recreate the web editor, lessons, teacher/classroom features, visual error hints, or adventure catalog.
- A configurable execution-step limit is a runtime safety policy, not a Hedy language rule.
- The parser also accepts `if key is pressed` as an optional host-backed extension; it is not counted as part of the selected level-1–16 manual profile.

## Sources

- [Official Hedy teacher manual](https://hedy.org/for-teachers/manual), including the current level descriptions and transitions.
- [Official English level grammars](https://github.com/hedyorg/hedy/tree/main/grammars-Total).
