# KelpyShark Language Syntax Reference

## Comments

```
# This is a single-line comment

### This is a
multi-line comment ###
```

## Variables

KelpyShark is dynamically typed. Variables are created on assignment.

```
x = 42
name = "KelpyShark"
active = true
items = [1, 2, 3]
data = {"key": "value"}
```

## Data Types

| Type | Example |
|---|---|
| Number | `42`, `3.14` |
| String | `"hello"` |
| Boolean | `true`, `false` |
| List | `[1, "two", true]` |
| Dict | `{"name": "Bob", "age": 27}` |
| Null | (returned from functions with no return value) |

## Operators

### Arithmetic
| Op | Meaning |
|---|---|
| `+` | Add (or concatenate strings) |
| `-` | Subtract |
| `*` | Multiply |
| `/` | Divide |
| `%` | Modulo |

### Comparison
| Op | Meaning |
|---|---|
| `==` | Equal |
| `!=` | Not equal |
| `<` | Less than |
| `<=` | Less or equal |
| `>` | Greater than |
| `>=` | Greater or equal |

### Logical
| Op | Meaning |
|---|---|
| `and` | Logical AND |
| `or` | Logical OR |
| `not` | Logical NOT |

## Strings

```
greeting = "Hello, world!"
escaped = "line1\nline2\ttab"
interpolated = "Hello, {$name}!"
concatenated = "Hello" + " " + "world"
```

## Functions

```
def add(a, b) {
    return a + b
}

result = add(3, 7)
print result   # 10
```

Functions are first-class values and support recursion.

## Control Flow

### If / Else

```
if x >= 10 {
    print "big"
} else {
    print "small"
}
```

### While Loop

```
x = 0
while x < 5 {
    print x
    x = x + 1
}
```

### For Loop

```
fruits = ["apple", "banana", "cherry"]
for fruit in fruits {
    print fruit
}
```

## Collections

### Lists

```
items = [1, 2, 3]
print items[0]          # 1
print len(items)        # 3
```

### Dictionaries

```
person = {"name": "Alice", "age": 30}
print person["name"]    # Alice
print len(person)       # 2
```

## Imports

```
import math
import http.server
```

## Print

`print` is a statement keyword (not a function).

```
print "Hello!"
print 42
print "Value: {$x}"
```

## Built-in Functions

- `len(x)` — Returns length of a string, list, or dict
- `type(x)` — Returns the type name as a string
- `str(x)` — Converts any value to a string
- `num(x)` — Converts a value to a number
- `push(list, value)` — Returns a new list with the value appended
