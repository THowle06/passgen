# Rust Password Generator

A simple and secure command-line password generator written in Rust.

## Features

- Cryptographically secure password generation
- Configurable length and count
- Optional exclusion of symbols
- Guaranteed inclusion of character classes
- Entopy calculation with strength classification
- Optional clipboard support (feature-gated)

## Installation

Clone the repository and build with Cargo:

```bash
git clone https://github.com/THowle06/passgen.git
cd passgen
cargo build --release
```

Run the binary:

```bash
./target/release/passgen
```

## Usage

### Generate a password

```bash
cargo run
```

### Generate multiple passwords

```bash
cargo run -- --count 5
```

## Custom length

```bash
cargo run -- --length 20
```

### Exclude symbols

```bash
cargo run -- --no-symbols
```

### Require all character classes

```bash
cargo run -- --require-each-class
```

## Show entropy

```bash
cargo run -- --show-entropy
```

## Copy to clipboard (Linux)

```bash
cargo run --features clipboard-support -- --clipboard
```

## Example Output

```text
!9XyTPac7$E5tWWF
f^j8BHsF0+Te_iJr

Entropy: 99.97 bits (Strong)
```

## Security Notes

- Passwords are generated using a cryptographically secure randon number generator.
- Entropy is calculated based on password length and character set size.
- Clipboard support depends on the system environment (X11/Wayland).

## Development

Run tests:

```bash
cargo test
```

```bash
cargo clippy
```

```bash
cargo fmt
```
