# advanced-random-string

`advanced-random-string` is a Rust library for generating random strings with customizable character sets. It supports both basic and cryptographically secure random number generators (RNGs).

## Features

- Generate random strings with customizable character sets.
- Use cryptographically secure RNGs for secure string generation.
- Allow users to specify their own RNGs for advanced use cases.
- Predefined character sets for convenience.

## Installation

Add `advanced-random-string` to your `Cargo.toml`:

```toml
[dependencies]
advanced-random-string = "0.1.3"
```

## Usage

### Generate a Random String with a Customizable Character Set

```rust
use advanced_random_string::{charset, random_string};

let random_string = random_string::generate(10, charset::BASE62);
println!("Generated string: {}", random_string);

// Specify a custom charset
let charset = b"MY_CHARSET";
let random_string_with_custom_charset = random_string::generate(10, charset);
println!("Generated string: {}", random_string_with_custom_charset);
```

### Generate an Unsecure Random String

```rust
use advanced_random_string::{charset, random_string};

let random_string = random_string::generate_unsecure(10, charset::BASE62);
println!("Generated string: {}", random_string);
```

### Generate a Cryptographically Secure Random String

```rust
use advanced_random_string::{charset, random_string};

let random_string = random_string::generate_os_secure(10, charset::BASE62);
println!("Generated string: {}", random_string);
```

### Generate a Random String with a User-Specified RNG

```rust
use rand::SeedableRng;
use rand::rngs::SmallRng;
use advanced_random_string::{charset, random_string};

let mut rng = SmallRng::from_entropy();
let random_string = random_string::generate_with_rng(10, charset::BASE62, &mut rng);
println!("Generated string: {}", random_string);
```

## Predefined Character Sets

The library provides some predefined character sets for convenience:

- ALPHANUMERIC
- UPPERCASE
- LOWERCASE
- DIGITS
- HEXADECIMAL
- ALPHABETIC
- BASE62
- BASE64
- ASCII_PRINTABLE
- URLSAFE_BASE64
- BINARY
- OCTAL
- SYMBOLS
- WHITESPACE
- EXTENDED_ASCII

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.