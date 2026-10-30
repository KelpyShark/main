# KelpyShark examples

Each file is a small program you can read, run, and change. From the repository
root, run an example with:

```console
kelpyshark run examples/hello.ks
```

| File | What it demonstrates |
|---|---|
| [`hello.ks`](hello.ks) | Variables, functions, lists, dictionaries, loops, and interpolation |
| [`data_structures.ks`](data_structures.ks) | Lists, dictionaries, iteration, and a `while` loop |
| [`fizzbuzz.ks`](fizzbuzz.ks) | Functions, loops, comparisons, and conditional branches |
| [`functions.ks`](functions.ks) | Recursion and reusable functions, including factorial and Fibonacci examples |
| [`strings.ks`](strings.ks) | Interpolation, concatenation, string length, and conversions |
| [`c_hello.ks`](c_hello.ks) | A small program for generating C source with `kelpyshark build` |

To try another program, replace `hello.ks` in the command with its filename.
For example:

```console
kelpyshark run examples/data_structures.ks
```

`kelpyshark build examples/c_hello.ks --target c` writes a `.c` source file.
Use a C compiler separately if you want to create a native executable.
