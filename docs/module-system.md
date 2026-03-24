# KelpyShark Module System

KelpyShark supports modular code organization through an import system that provides
access to standard library modules and user-defined libraries.

---

## Import Syntax

```
import math
import http.server
```

The `import` keyword loads a module and makes its exported functions available in the
current scope.

---

## Module Resolution Order

When KelpyShark encounters an `import` statement, it searches in this order:

```
import math
   │
   ├─ 1. Standard Library    (built-in modules: math, strings, io, json, sys)
   │
   ├─ 2. Project libs/       (installed packages in libs/ directory)
   │
   └─ 3. Local registry      (~/.kelpyshark/registry/)
```

The first match wins. Standard library modules always take priority.

---

## Standard Library Modules

These modules are always available without installation:

### `math`

Mathematical functions and constants.

| Function | Description | Example |
|---|---|---|
| `abs(x)` | Absolute value | `abs(-5)` → `5` |
| `floor(x)` | Round down | `floor(3.7)` → `3` |
| `ceil(x)` | Round up | `ceil(3.2)` → `4` |
| `round(x)` | Round to nearest | `round(3.5)` → `4` |
| `sqrt(x)` | Square root | `sqrt(16)` → `4` |
| `pow(x, y)` | Exponentiation | `pow(2, 10)` → `1024` |
| `min(a, b)` | Minimum | `min(3, 7)` → `3` |
| `max(a, b)` | Maximum | `max(3, 7)` → `7` |
| `PI` | π constant | `3.14159...` |
| `E` | Euler's number | `2.71828...` |

### `strings`

String manipulation functions.

| Function | Description | Example |
|---|---|---|
| `upper(s)` | Uppercase | `upper("hi")` → `"HI"` |
| `lower(s)` | Lowercase | `lower("HI")` → `"hi"` |
| `trim(s)` | Remove whitespace | `trim("  hi  ")` → `"hi"` |
| `split(s, delim)` | Split into list | `split("a,b", ",")` → `["a","b"]` |
| `join(list, delim)` | Join list | `join(["a","b"], ",")` → `"a,b"` |
| `contains(s, sub)` | Check substring | `contains("hello", "ell")` → `true` |
| `replace(s, old, new)` | Replace text | `replace("hi", "i", "ey")` → `"hey"` |
| `starts_with(s, pre)` | Prefix check | `starts_with("hello", "he")` → `true` |
| `ends_with(s, suf)` | Suffix check | `ends_with("hello", "lo")` → `true` |
| `char_at(s, i)` | Character at index | `char_at("hello", 0)` → `"h"` |
| `substring(s, start, len)` | Extract slice | `substring("hello", 1, 3)` → `"ell"` |
| `reverse(s)` | Reverse string | `reverse("abc")` → `"cba"` |
| `length(s)` | String length | `length("hello")` → `5` |

### `io`

File and console I/O.

| Function | Description |
|---|---|
| `read_file(path)` | Read entire file as string |
| `write_file(path, content)` | Write string to file (overwrites) |
| `append_file(path, content)` | Append string to file |
| `file_exists(path)` | Check if file exists (boolean) |
| `input(prompt)` | Read a line from the console |

### `json`

JSON serialization and deserialization.

| Function | Description |
|---|---|
| `json_encode(value)` | Convert a KelpyShark value to a JSON string |
| `json_decode(string)` | Parse a JSON string into a KelpyShark value |

Supported value conversions:

| KelpyShark | JSON |
|---|---|
| Number | `number` |
| String | `"string"` |
| Boolean | `true` / `false` |
| Null | `null` |
| List | `[array]` |
| Dict | `{object}` |

### `sys`

System-level utilities.

| Function | Description |
|---|---|
| `env(name)` | Get environment variable (or Null) |
| `exit(code)` | Exit with status code |
| `clock()` | Current Unix timestamp (seconds) |
| `args()` | Command-line arguments as a list |
| `cwd()` | Current working directory |
| `platform()` | OS name (`"linux"`, `"macos"`, `"windows"`) |

---

## User-Defined Modules

### Creating a Library

A library is a directory with `kelpy.toml` and source files:

```
my_library/
├── kelpy.toml
└── src/
    ├── lib.ks          # Entry point (loaded on import)
    └── helpers.ks      # Additional module
```

### Exporting Functions

Any function defined at the top level of `lib.ks` is automatically exported:

```
# my_library/src/lib.ks

def greet(name) {
    print "Hello, {$name}!"
}

def farewell(name) {
    print "Goodbye, {$name}!"
}
```

### Using the Library

After installing (or placing in `libs/`):

```
import my_library

my_library.greet("world")
```

---

## Dot-Separated Imports

Sub-modules use dot notation:

```
import http.server
```

This resolves to `libs/http/src/server.ks` (or the equivalent in the standard library).

---

## Project Layout

```
my_project/
├── kelpy.toml          # Manifest with [dependencies]
├── src/
│   └── main.ks         # Your code with import statements
└── libs/               # Installed packages (auto-managed)
    ├── http/
    │   ├── kelpy.toml
    │   └── src/
    │       └── lib.ks
    └── json/
        ├── kelpy.toml
        └── src/
            └── lib.ks
```

---

## Built-in Functions

These are always available without any import:

| Function | Arity | Description |
|---|---|---|
| `len(x)` | 1 | Length of string, list, or dict |
| `type(x)` | 1 | Type name as a string |
| `str(x)` | 1 | Convert to string |
| `num(x)` | 1 | Convert to number |
| `push(list, val)` | 2 | Return new list with value appended |
| `print` | — | Print statement (keyword, not a function) |

---

## Example

```
import math
import strings

# Use standard library functions
radius = 5
area = math.PI * math.pow(radius, 2)
print "Area: {$area}"

name = "  KelpyShark  "
clean = strings.trim(name)
upper = strings.upper(clean)
print upper   # "KELPYSHARK"
```
