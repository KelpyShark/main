# KelpyShark AST Specification

The Abstract Syntax Tree (AST) is the intermediate representation produced by the parser
and consumed by the semantic analyzer, interpreter, and code generators.

Every node carries a `SourceLocation` (line, column) for error reporting.

---

## AST Structure Diagram

```
                          Program
                            │
                ┌───────────┼───────────┐
                ▼           ▼           ▼
           Statement    Statement    Statement ...
                │
   ┌────────────┼────────────────────────┐
   ▼            ▼            ▼           ▼
Assignment  FunctionDef   If/While/For  Print ...
   │            │            │
   ▼            ▼            ▼
  Expr      Vec<Stmt>   Expr + Vec<Stmt>
   │
   ├──► NumberLiteral    (leaf)
   ├──► StringLiteral    (leaf)
   ├──► BooleanLiteral   (leaf)
   ├──► Identifier       (leaf)
   ├──► BinaryOp         (left: Expr, right: Expr)
   ├──► UnaryOp          (operand: Expr)
   ├──► FunctionCall     (callee: Expr, args: Vec<Expr>)
   ├──► Index            (object: Expr, index: Expr)
   ├──► MemberAccess     (object: Expr, member: String)
   ├──► ListLiteral      (elements: Vec<Expr>)
   ├──► DictLiteral      (entries: Vec<(Expr, Expr)>)
   └──► StringInterpolation (parts: Vec<StringPart>)
```

---

## Root Node

### `Program`

The top-level node representing an entire `.ks` source file.

| Field | Type | Description |
|---|---|---|
| `statements` | `Vec<Statement>` | Ordered list of top-level statements |

---

## Statement Nodes

### `Assignment`

Variable assignment. Creates or updates a binding.

```
x = 42
name = "KelpyShark"
```

| Field | Type | Description |
|---|---|---|
| `name` | `String` | Variable name |
| `value` | `Expr` | Right-hand side expression |
| `location` | `SourceLocation` | Source position |

### `FunctionDef`

Function definition. Parameters are positional.

```
def add(a, b) {
    return a + b
}
```

| Field | Type | Description |
|---|---|---|
| `name` | `String` | Function name |
| `params` | `Vec<String>` | Parameter names |
| `body` | `Vec<Statement>` | Function body statements |
| `location` | `SourceLocation` | Source position |

### `If`

Conditional with optional else branch.

```
if x > 10 {
    print "big"
} else {
    print "small"
}
```

| Field | Type | Description |
|---|---|---|
| `condition` | `Expr` | Condition expression |
| `then_body` | `Vec<Statement>` | True branch |
| `else_body` | `Option<Vec<Statement>>` | Optional false branch |
| `location` | `SourceLocation` | Source position |

### `While`

Pre-condition loop.

```
while x < 10 {
    x = x + 1
}
```

| Field | Type | Description |
|---|---|---|
| `condition` | `Expr` | Loop condition |
| `body` | `Vec<Statement>` | Loop body |
| `location` | `SourceLocation` | Source position |

### `For`

Iteration over a collection.

```
for item in items {
    print item
}
```

| Field | Type | Description |
|---|---|---|
| `variable` | `String` | Loop variable name |
| `iterable` | `Expr` | Expression producing the iterable |
| `body` | `Vec<Statement>` | Loop body |
| `location` | `SourceLocation` | Source position |

### `Return`

Return from a function. Only valid inside `FunctionDef`.

```
return x + 1
```

| Field | Type | Description |
|---|---|---|
| `value` | `Option<Expr>` | Return value (None for bare `return`) |
| `location` | `SourceLocation` | Source position |

### `Import`

Module import.

```
import math
import http.server
```

| Field | Type | Description |
|---|---|---|
| `module` | `String` | Module path (dot-separated) |
| `location` | `SourceLocation` | Source position |

### `Print`

Print statement (keyword, not a function).

```
print "Hello!"
print x + 1
```

| Field | Type | Description |
|---|---|---|
| `value` | `Expr` | Expression to print |
| `location` | `SourceLocation` | Source position |

### `ExprStatement`

Bare expression used as a statement (e.g., a function call).

```
do_something()
```

| Field | Type | Description |
|---|---|---|
| `expr` | `Expr` | The expression |
| `location` | `SourceLocation` | Source position |

---

## Expression Nodes

### `NumberLiteral`

Integer or floating-point number.

| Field | Type | Description |
|---|---|---|
| `value` | `f64` | Numeric value |
| `location` | `SourceLocation` | Source position |

Examples: `42`, `3.14`, `0`, `-1`

### `StringLiteral`

Plain string (no interpolation).

| Field | Type | Description |
|---|---|---|
| `value` | `String` | String contents (escape sequences resolved) |
| `location` | `SourceLocation` | Source position |

Supported escapes: `\n`, `\t`, `\\`, `\"`

### `BooleanLiteral`

| Field | Type | Description |
|---|---|---|
| `value` | `bool` | `true` or `false` |
| `location` | `SourceLocation` | Source position |

### `Identifier`

Variable or function name reference.

| Field | Type | Description |
|---|---|---|
| `name` | `String` | The identifier |
| `location` | `SourceLocation` | Source position |

### `BinaryOp`

Binary operation between two expressions.

| Field | Type | Description |
|---|---|---|
| `left` | `Box<Expr>` | Left operand |
| `op` | `BinaryOperator` | Operator |
| `right` | `Box<Expr>` | Right operand |
| `location` | `SourceLocation` | Source position |

### `UnaryOp`

Unary prefix operation.

| Field | Type | Description |
|---|---|---|
| `op` | `UnaryOperator` | Operator |
| `operand` | `Box<Expr>` | Operand |
| `location` | `SourceLocation` | Source position |

### `FunctionCall`

Function invocation. The callee is an expression to support first-class functions.

| Field | Type | Description |
|---|---|---|
| `callee` | `Box<Expr>` | Expression that resolves to a function |
| `args` | `Vec<Expr>` | Positional arguments |
| `location` | `SourceLocation` | Source position |

### `Index`

Bracket access on lists and dictionaries.

```
list[0]
dict["key"]
```

| Field | Type | Description |
|---|---|---|
| `object` | `Box<Expr>` | The collection |
| `index` | `Box<Expr>` | The index/key expression |
| `location` | `SourceLocation` | Source position |

### `MemberAccess`

Dot access.

```
obj.field
```

| Field | Type | Description |
|---|---|---|
| `object` | `Box<Expr>` | The object |
| `member` | `String` | Field name |
| `location` | `SourceLocation` | Source position |

### `ListLiteral`

List constructor.

```
[1, 2, 3]
["a", "b"]
```

| Field | Type | Description |
|---|---|---|
| `elements` | `Vec<Expr>` | Element expressions |
| `location` | `SourceLocation` | Source position |

### `DictLiteral`

Dictionary constructor. Keys and values are both expressions.

```
{"name": "Alice", "age": 30}
```

| Field | Type | Description |
|---|---|---|
| `entries` | `Vec<(Expr, Expr)>` | Key-value pairs |
| `location` | `SourceLocation` | Source position |

### `StringInterpolation`

A string containing embedded `{$expr}` expressions.

```
"Hello, {$name}! You are {$age} years old."
```

| Field | Type | Description |
|---|---|---|
| `parts` | `Vec<StringPart>` | Sequence of literal and expression parts |
| `location` | `SourceLocation` | Source position |

---

## Auxiliary Types

### `StringPart`

Component of an interpolated string.

| Variant | Data | Description |
|---|---|---|
| `Literal` | `String` | Plain text segment |
| `Expression` | `Expr` | Evaluated expression segment |

### `BinaryOperator`

| Variant | Symbol | Precedence |
|---|---|---|
| `Add` | `+` | 5 |
| `Subtract` | `-` | 5 |
| `Multiply` | `*` | 6 |
| `Divide` | `/` | 6 |
| `Modulo` | `%` | 6 |
| `Equal` | `==` | 3 |
| `NotEqual` | `!=` | 3 |
| `LessThan` | `<` | 4 |
| `LessEqual` | `<=` | 4 |
| `GreaterThan` | `>` | 4 |
| `GreaterEqual` | `>=` | 4 |
| `And` | `and` | 2 |
| `Or` | `or` | 1 |

### `UnaryOperator`

| Variant | Symbol | Precedence |
|---|---|---|
| `Negate` | `-` | 7 |
| `Not` | `not` | 7 |

### `SourceLocation`

Attached to every node for error reporting.

| Field | Type | Description |
|---|---|---|
| `line` | `usize` | 1-based line number |
| `column` | `usize` | 1-based column number |

---

## Semantic Analysis Passes

After the AST is built, the semantic analyzer validates it. The following checks are performed:

```
          AST
           │
           ▼
┌──────────────────┐
│ Scope Resolution │  Track variable definitions and usages
└────────┬─────────┘
         │
         ▼
┌──────────────────┐
│ Function Checks  │  Arity, duplicates, return placement
└────────┬─────────┘
         │
         ▼
┌──────────────────┐
│  Warning Pass    │  Unused variables
└────────┬─────────┘
         │
         ▼
  Diagnostics (errors / warnings)
```

### Checks

| Check | Severity | Description |
|---|---|---|
| Undefined variable | Error | Variable used before assignment |
| Undefined function | Error | Call to unknown function |
| Arity mismatch | Error | Wrong number of arguments |
| Duplicate function | Error | Two `def` with same name |
| Return outside function | Error | `return` at top level |
| Unused variable | Warning | Variable assigned but never read |

### Built-in Awareness

The analyzer knows about built-in functions (`len`, `type`, `str`, `num`, `push`, `print`)
and will not flag them as undefined.

---

## Example AST

Given this source:

```
def greet(name) {
    print "Hello, {$name}!"
}
greet("world")
```

The AST looks like:

```
Program
├── FunctionDef
│   ├── name: "greet"
│   ├── params: ["name"]
│   └── body:
│       └── Print
│           └── StringInterpolation
│               ├── Literal("Hello, ")
│               ├── Expression(Identifier("name"))
│               └── Literal("!")
└── ExprStatement
    └── FunctionCall
        ├── callee: Identifier("greet")
        └── args:
            └── StringLiteral("world")
```
