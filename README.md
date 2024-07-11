# Brainfuck Interpreter in Rust
This project is a Brainfuck interpreter written in Rust. It reads Brainfuck source code from a file, parses it into instructions, and executes it. This interpreter supports all standard Brainfuck operations and includes a lexer, parser, and interpreter.

### Features
- Lexer: Reads Brainfuck source code and tokenizes it.
- Parser: Parses tokens into executable instructions.
- Interpreter: Executes the parsed Brainfuck instructions.
- Error Handling: Utilizes `anyhow` for robust error handling.

## Installation 

**Cloning the Repository**
```console
// bash 
git clone https://github.com/mrinalxdev/brainfuck-interpreter-rust.git
cd brainfuck-interpreter-rust
```

**Build the Project**

```bash 
cargo build --release
```

### Usage

To run the Brainfuck interpreter, you need to provide a Brainfuck source file as input.

```bash 
cargo run --release <brainfuck_file>
```


### Contribution 

Contributions are welcome! Please open an issue or submit a pull request on GitHub.