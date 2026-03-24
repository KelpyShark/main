# KelpyShark Compiler Architecture

## Overview

The KelpyShark compiler follows a classic multi-stage pipeline:

```
Source Code (.ks)
       │
       ▼
┌─────────────┐
│    Lexer    │  Tokenizes source into token stream
└──────┬──────┘
       │
       ▼
┌─────────────┐
│   Parser    │  Pratt parser → AST
└──────┬──────┘
       │
       ▼
┌─────────────┐
│  Semantic   │  Validates AST correctness
│  Analyzer   │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│    Code     │  AST → C / JS / Java / C#
│  Generator  │
└──────┬──────┘
       │
       ▼
  Target Code
```

## Lexer (`compiler/src/lexer.rs`)

The lexer (tokenizer) converts raw source text into a stream of tokens.

### Token Types
- **Identifiers**: variable/function names
- **Keywords**: `def`, `if`, `else`, `while`, `for`, `in`, `return`, `import`, `true`, `false`, `and`, `or`, `not`, `print`
- **Literals**: strings (`"hello"`), numbers (`42`, `3.14`), booleans
- **Operators**: `+`, `-`, `*`, `/`, `%`, `=`, `==`, `!=`, `<`, `<=`, `>`, `>=`
- **Punctuation**: `(`, `)`, `{`, `}`, `[`, `]`, `,`, `:`, `.`
- **Special**: `NEWLINE`, `EOF`

### Error Reporting
The lexer tracks line and column positions for every token, enabling precise error messages.

## Parser (`compiler/src/parser.rs`)

Uses a **Pratt parser** (Top-Down Operator Precedence) for expressions and recursive descent for statements.

### Precedence Levels (lowest to highest)
1. `or`
2. `and`
3. `==`, `!=`
4. `<`, `<=`, `>`, `>=`
5. `+`, `-`
6. `*`, `/`, `%`
7. Unary: `not`, `-`
8. Call: `()`, `[]`, `.`

### Why Pratt?
- Elegant handling of operator precedence
- Easy to extend with new operators
- Clean, maintainable code

## AST (`compiler/src/ast.rs`)

### Statement Nodes
- `Assignment` — `name = expr`
- `FunctionDef` — `def name(params) { body }`
- `If` — `if cond { then } else { else }`
- `While` — `while cond { body }`
- `For` — `for var in iterable { body }`
- `Return` — `return expr`
- `Import` — `import module`
- `Print` — `print expr`
- `ExprStatement` — bare expression

### Expression Nodes
- `NumberLiteral`, `StringLiteral`, `BooleanLiteral`
- `Identifier`
- `BinaryOp`, `UnaryOp`
- `FunctionCall`
- `Index`, `MemberAccess`
- `ListLiteral`, `DictLiteral`
- `StringInterpolation`

## Code Generators

### C Generator (`compiler/src/codegen/c.rs`)
- All values represented as tagged union (`KsValue`)
- Emits complete standalone C file with runtime
- Generated code compiles with `gcc -o output output.c -lm`

### JavaScript Generator (`compiler/src/codegen/javascript.rs`)
- Direct mapping to JS semantics
- `print` → `console.log()`
- String interpolation → JS template literals
- Lists/dicts map directly to JS arrays/objects

## Interpreter (`interpreter/src/interpreter.rs`)

Tree-walking interpreter that evaluates AST nodes directly.

### Environment
Lexical scoping with a scope stack. Inner scopes can read/update outer variables.

### Values
All runtime values are represented by the `Value` enum:
- `Number(f64)`
- `String(String)`
- `Boolean(bool)`
- `List(Vec<Value>)`
- `Dict(HashMap<String, Value>)`
- `Function { name, params, body }`
- `NativeFunction { name, arity, func }`
- `Null`
