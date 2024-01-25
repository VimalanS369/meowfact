
# Meow Fact

## Introduction
This Rust program fetches a random cat fact from the [Cat Fact Ninja API](https://catfact.ninja/fact) using the reqwest and serde_json crates. The retrieved JSON response is then parsed, and the cat fact is printed to the console.

## Prerequisites
- Rust programming language
- Cargo package manager

## Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/VimalanS369/meow-fact.git
   cd meow-fact
   ```

2. Build the project using Cargo:
   ```bash
   cargo build --release
   ```

3. Run the executable:
   ```bash
   ./target/release/meow-fact
   ```

   - Ensure an active internet connection to fetch cat facts from the Cat Fact Ninja API.

## Usage
- The program fetches a random cat fact from the Cat Fact Ninja API and prints it to the console.

## Example Output
```bash
"The first formal cat show was held in England in 1871; in America, in 1895."
```

## Dependencies
- [reqwest](https://crates.io/crates/reqwest): A simple HTTP client for Rust.
- [serde_json](https://crates.io/crates/serde_json): A JSON serialization/deserialization library for Rust.

## Error Handling
The program utilizes Rust's error handling mechanism and returns a `Result` with a `Box<dyn Error>` trait to handle potential errors during execution.

Feel free to modify and use the code as needed! If you encounter any issues or have suggestions, please open an issue on the GitHub repository. Contributions are welcome!
