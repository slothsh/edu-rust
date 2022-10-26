# First Hello World! in Rust
**Source:** https://doc.rust-lang.org/book/ch01-02-hello-world.html

### Basic Code

The message is printed to the terminal using a macro, hence the `cmd!` syntax.
Functions are declared using the `fn` keyword.
Just like C/C++, the entry point for your application must be called `main()`:

```rust
fn main() {
    printlin!("Hello, World!");
}
```

### Compilation Using rustc

The Rust compiler can be invoked directly to compile source files:

```bash
rustc src.rs
```

### Observations

Rust syntax is very similar to C-like languages:

- Scopes are enclosed in curly-braces '{}'
- Spaces are ignored
- Stack based allocation
- functions, locals & macros can be declared and used
- Semi-colons are used to punctuate ends of expressions
- Brackets used for function call parameters
- Strongly-typed

Rust building procedures is similar to C:

- Source code is compiled __ahead-of-time__
- Executable is compiled for platform target

___
