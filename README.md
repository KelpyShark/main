# Learn KelpyShark

KelpyShark is a small, dynamically typed programming language. This guide gets you
from your first program to the language reference, with short examples you can
run as you learn.

## Start here

New to KelpyShark? Follow the [step-by-step tutorial](https://kelpyshark.kelpw.ing/tutorial/introduction).
It covers installation, variables, data types, conditions, loops, functions,
collections, classes, errors, and modules. Use the [language reference](https://kelpyshark.kelpw.ing/reference/keywords)
when you need to look up a specific feature.

## Your first program

Create a file named `hello.ks` and add this code:

```ks
name = "World"
print("Hello, {$name}!")
```

Run the file with the KelpyShark command-line tool:

```console
kelpyshark run hello.ks
```

The program prints:

```text
Hello, World!
```

KelpyShark runs statements from top to bottom. A `.ks` file does not need a
`main` function.

## Install KelpyShark

Use the installer for your operating system:

**Windows PowerShell**

```powershell
irm https://github.com/KelpyShark/main/raw/main/installer/install.ps1 | iex
```

**macOS or Linux**

```sh
curl -fsSL https://github.com/KelpyShark/main/raw/main/installer/install.sh | bash
```

Open a new terminal and check that the command is available:

```console
kelpyshark --version
```

See the [installation lesson](https://kelpyshark.kelpw.ing/tutorial/setup) for the full walkthrough.

## Learn by example

KelpyShark uses assignment to create or update variables. Use `def` to define
functions, braces to mark blocks, and `print` to show a value:

```ks
def greet(name) {
    print("Hello, {$name}!")
}

for name in ["Ada", "Lin"] {
    greet(name)
}
```

Output:

```text
Hello, Ada!
Hello, Lin!
```

Each tutorial lesson introduces one idea, shows a runnable example, and ends
with practice prompts. Save an example in a `.ks` file and run it with
`kelpyshark run filename.ks`.

## Language basics

| Topic | Example |
|---|---|
| Variables | `score = 42` |
| Values | `name = "Ada"`, `ready = true`, `missing = null` |
| Lists | `colors = ["blue", "green"]` |
| Dictionaries | `person = {"name": "Ada"}` |
| Functions | `def add(a, b) { return a + b }` |
| Conditions | `if score >= 10 { print("Pass") }` |
| Loops | `for color in colors { print(color) }` |
| String interpolation | `print("Hello, {$name}!")` |
| Imports | `import math` |

Blocks can use braces or a colon and `end`. Indentation helps people read the
code, but it does not define a block.

## Run or build a program

Use `run` to execute a file with the KelpyShark interpreter:

```console
kelpyshark run hello.ks
```

Use `build` to generate source code for another language:

```console
kelpyshark build hello.ks --target js
```

`build` writes generated source; it does not run a JavaScript, Java, C#, or C
compiler for you. Generated targets support different subsets of KelpyShark.
The interpreter is the most complete way to run a KelpyShark program.

| Target | Generated file | Notes |
|---|---|---|
| Interpreter | Runs the `.ks` file | Full language runtime and registered standard-library modules |
| C | `.c` | Limited support; use the generated source with a C toolchain |
| JavaScript | `.js` | Supported subset; use Node.js or a browser runtime |
| Java | `.java` | Experimental subset; requires a JDK |
| C# | `.cs` | Experimental subset; requires the .NET SDK |

Local source files and packages installed under `libs/` cannot yet be imported
from a KelpyShark program. Imports currently load registered standard-library
modules such as `math`, `strings`, `io`, `json`, `sys`, and `http`.

## Command-line shortcuts

| Command | What it does |
|---|---|
| `kelpyshark run file.ks` | Run a program |
| `kelpyshark repl` | Start the interactive interpreter |
| `kelpyshark build file.ks --target js` | Generate JavaScript source |
| `kelpyshark new my-project` | Create a project with `kelpy.toml`, `src/main.ks`, and `libs/` |
| `kelpyshark install` | Install dependencies listed in the current project's manifest |
| `kelpyshark --help` | Show available commands and options |

The CLI also accepts `ks` and `kshark` as executable names. Package publishing
uses a local registry on the current machine; it does not upload to a public
service.

## Documentation

- [Tutorial: start learning KelpyShark](https://kelpyshark.kelpw.ing/tutorial/introduction)
- [Language reference](https://kelpyshark.kelpw.ing/reference/keywords)
- [Syntax and grammar](https://kelpyshark.kelpw.ing/reference/grammar)
- [Built-in functions](https://kelpyshark.kelpw.ing/reference/builtin-functions)
- [Standard library](https://kelpyshark.kelpw.ing/tutorial/stdlib)
- [Command-line reference](https://kelpyshark.kelpw.ing/general/cli)
- [Compiler and generated targets](https://kelpyshark.kelpw.ing/general/codegen)
- [Package manager](https://kelpyshark.kelpw.ing/general/package-manager)

## License

MIT
