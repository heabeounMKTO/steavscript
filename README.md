# BetielScript (bts)

BetielScript is a small tree-walking interpreter written in Rust. It implements a
[Lox](https://craftinginterpreters.com/)-style scripting language, but with
Khmer-inspired keywords instead of the usual English ones — variables use `akthe`
instead of `var`, loops use `somhab` instead of `for`, and so on.

Every `.bts` script is required to start with a `bongSlanhOun "...";` statement
(literally "declare/announce" in the language's spirit) — the interpreter refuses
to run a file that doesn't start with it.

## Example

```bts
bongSlanhOun "jea reang rohot?";

somhab (akthe i = 0; i < 20; i = i + 1){
  jongyeytha "jongyeyjengha";
}
```

- `bongSlanhOun "...";` — required first line of every script.
- `somhab (init; condition; increment) { ... }` — a `for` loop.
- `akthe` — declare a variable.
- `jongyeytha` — print a value.

## Keyword Reference

| Keyword       | Meaning              |
|---------------|----------------------|
| `bongSlanhOun`| required file header |
| `ng`          | `and`                |
| `tnak`        | `class`              |
| `minjengte`   | `else`               |
| `ort`         | `false`              |
| `rupamun`     | `fun` (function)     |
| `somhab`      | `for`                |
| `ber`         | `if`                 |
| `sone`        | `nil`                |
| `reu`         | `or`                 |
| `jongyeytha`  | `print`              |
| `morvenh`     | `return`             |
| `super`       | `super`              |
| `nis`         | `this`               |
| `ok`          | `true`               |
| `akthe`       | `var`                |
| `nvpeldae`    | `while`              |

Standard operators (`+`, `-`, `*`, `/`, `=`, `==`, `!=`, `<`, `<=`, `>`, `>=`, `!`)
and literals (numbers, strings, `{ }` blocks, `( )` grouping) work as in Lox.

## Building

Requires a recent stable Rust toolchain (edition 2021).

```sh
cargo build --release
```

## Usage

Run a script file:

```sh
cargo run --release -- testbts/main.bts
```

Or launch an interactive REPL by running with no arguments:

```sh
cargo run --release
```

> Note: the `bongSlanhOun` header check is currently only enforced for file mode,
> not for REPL input.

## Project Structure

```
src/
├── main.rs         # entry point, keyword table, REPL/file runner
├── scanner.rs       # lexer: source text -> tokens
├── token.rs         # token & token type definitions
├── parser.rs         # recursive-descent parser: tokens -> AST
├── syntax.rs         # AST node definitions (Expr/Stmt) + visitor traits, AST printer
├── interpreter.rs    # tree-walking evaluator
├── env.rs             # lexical environments / variable scoping
├── function.rs        # user-defined and native callable functions
├── object.rs          # runtime value representation (Object)
└── error.rs           # error types (parse, runtime, return-as-control-flow, IO)
testbts/
└── main.bts           # sample script
```

## Language Features

- Variables, arithmetic, string/boolean literals, and control flow (`ber`/`minjengte`
  if-else, `nvpeldae` while, `somhab` for).
- First-class functions (`rupamun`) with `morvenh` (return) support.
- A built-in native function, `clock`, for reading the current time.
- Lexically scoped environments (blocks introduce new scopes).

## Status

This is a personal/learning project (following the structure of *Crafting
Interpreters*) and is a work in progress. Classes (`tnak`) are tokenized/parsed
for but not yet fully implemented in the interpreter.

## License

No license specified yet.
