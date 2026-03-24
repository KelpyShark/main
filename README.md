# KelpyShark Programming Language

**A readable, versatile, beginner-friendly programming language capable of compiling to multiple targets.**

## Quick Start

```bash
# Build the project
cargo build --release

# Run a KelpyShark script
cargo run --bin kelpyshark-cli -- run examples/hello.ks

# Start the REPL
cargo run --bin kelpyshark-cli -- repl

# Create a new project
cargo run --bin kelpyshark-cli -- new myproject

# Compile to C
cargo run --bin kelpyshark-cli -- build examples/hello.ks --target c

# Compile to JavaScript
cargo run --bin kelpyshark-cli -- build examples/hello.ks --target js
```

## Example

```
# KelpyShark program
bob = {"age": 27, "name": "Bob Smith"}

example_list = ["apple", "banana", "orange"]

def greet(name) {
    print "Hello, {$name}! 🦈"
}

greet(bob["name"])

for fruit in example_list {
    print "I like " + fruit
}

def factorial(n) {
    if n <= 1 {
        return 1
    }
    return n * factorial(n - 1)
}

print "10! = " + factorial(10)
```

## Language Features

| Feature | Syntax |
|---|---|
| Variables | `x = 42` |
| Strings | `name = "KelpyShark"` |
| Booleans | `true`, `false` |
| Lists | `items = [1, 2, 3]` |
| Dicts | `data = {"key": "value"}` |
| Functions | `def add(a, b) { return a + b }` |
| If/Else | `if x > 5 { ... } else { ... }` |
| While | `while x < 10 { ... }` |
| For | `for item in list { ... }` |
| Print | `print "Hello!"` |
| Comments | `# single line`, `### multi line ###` |
| String Interpolation | `"Hello {$name}!"` |
| Imports | `import math` |

## Project Structure

```
kelpyshark/
  compiler/          # Lexer, Parser, AST, Semantic Analysis, Code Generators
    src/
      lexer.rs       # Tokenizer
      ast.rs         # AST node definitions
      parser.rs      # Pratt parser
      semantic.rs    # Semantic analyzer
      codegen/
        c.rs         # C transpiler
        javascript.rs # JavaScript transpiler
      error.rs       # Error types
  interpreter/       # Tree-walking interpreter
    src/
      interpreter.rs # AST evaluator + REPL
      environment.rs # Variable scoping
      value.rs       # Runtime value types
    tests/
      integration_tests.rs  # 35 end-to-end tests
  cli/               # The `kelpy` command
  stdlib/            # Standard library (math, strings, io, json, sys)
  package_manager/   # Package manager (manifest, registry, resolver, installer)
  installer/         # Cross-platform installer (bash + PowerShell)
  docs/              # Documentation
  examples/          # Example programs
```

## Compilation Pipeline

```
Source Code (.ks)
       ↓
    Lexer          → Token stream
       ↓
    Parser         → AST (Abstract Syntax Tree)
       ↓
  Semantic         → Validated AST
  Analyzer
       ↓
  Code Generator   → C / JavaScript / Java / C#
       ↓
  Native Compiler  → Binary (via GCC/Clang)
```

## Compilation Targets

| Target | Status | Output |
|---|---|---|
| **Interpreter** | ✅ Working | Direct execution |
| **C** | ✅ Working | `.c` file → GCC/Clang → binary |
| **JavaScript** | ✅ Working | `.js` file → Node.js / browser |
| **Java** | 🔜 Planned | `.java` file |
| **C#** | 🔜 Planned | `.cs` file |
| **WebAssembly** | 🔜 Optional | `.wasm` file |

## Running Tests

```bash
cargo test --workspace
```

## Built-in Functions

| Function | Description |
|---|---|
| `len(x)` | Length of string, list, or dict |
| `type(x)` | Type name as string |
| `str(x)` | Convert to string |
| `num(x)` | Convert to number |
| `push(list, value)` | Add item to list |

## File Extension

KelpyShark source files use the `.ks` extension.

## Documentation

| Document | Description |
|---|---|
| [Language Syntax](docs/language-syntax.md) | Complete syntax reference |
| [Compiler Architecture](docs/compiler-architecture.md) | Pipeline design and internals |
| [AST Specification](docs/ast-specification.md) | All AST node types, fields, and diagrams |
| [Package Manager](docs/package-manager.md) | kelpy.toml, install, publish, dependency resolution |
| [Module System](docs/module-system.md) | Imports, standard library, user-defined modules |

## Test Coverage

195 automated tests across the workspace:

| Crate | Tests | Coverage |
|---|---|---|
| Compiler | 75 | Lexer, parser, semantic, C codegen, JS codegen |
| Interpreter | 31 | Tree-walking evaluation, REPL, builtins |
| Integration | 35 | End-to-end pipeline (source → output) |
| Stdlib | 38 | math, strings, io, json, sys |
| Package Manager | 16 | manifest, registry, resolver, installer |

## License

MIT
