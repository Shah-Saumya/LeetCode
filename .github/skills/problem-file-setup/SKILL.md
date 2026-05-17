---
name: problem-file-setup
description: 'Setup basic Cargo package structure for LeetCode problems. Use when initializing a new problem folder in 1-easy, 2-medium, or 3-hard directories.'
argument-hint: 'problem-id, problem-name, difficulty-level'
user-invocable: true
---

# Problem File Setup

## When to Use
- Creating a new LeetCode problem folder
- Initializing boilerplate for problems in `1-easy`, `2-medium`, or `3-hard` directories
- Standardizing project structure across the workspace

## Important Notes
⚠️ **This skill sets up the file structure ONLY** — it does NOT provide problem solutions. Each problem must be solved independently by implementing the `Solution` methods. The skill only provides:
- Folder and file organization
- Boilerplate structure (Solution struct, main function pattern)
- Example of how to call Solution methods in main()

✓ The `main()` function demonstrates how to invoke Solution methods but contains NO problem logic or solutions.

## Problem Naming Convention
- Folder name: `XXXX_problem_name` (e.g., `0001_two_sum`, `0004_median_of_sorted_array`)
- Package name: Derived from folder by removing ID prefix and using snake_case (e.g., `two_sum`, `median_of_sorted_array`)

## File Structure
```
<difficulty>/<XXXX_problem_name>/
├── Cargo.toml
└── src/
    └── main.rs
```

## Rust Edition & Version
- **Latest Edition**: Rust 2024 (stabilized in Rust 1.85.0, February 2025)
- **Recommended Edition**: 2024 for new problems
- **Fallback**: 2021 edition if using older Rust toolchain

## Procedure

### 1. Create Problem Folder
```bash
# Navigate to the appropriate difficulty level
cd 1-easy  # or 2-medium, 3-hard
mkdir XXXX_problem_name
```

### 2. Initialize Cargo Project
```bash
cd XXXX_problem_name
cargo init --name=package_name
```

Where `package_name` is the derived package name (snake_case, no ID prefix).

### 3. Setup Cargo.toml
The minimal `Cargo.toml` structure:

```toml
[package]
name = "package_name"
version = "0.1.0"
edition = "2024"

[dependencies]
```

Key points:
- **name**: Derived from folder (without ID prefix)
- **version**: Always start with "0.1.0"
- **edition**: Use "2024" (latest stable, Rust 1.85.0+). Use "2021" if on older toolchain
- **dependencies**: Keep empty unless external crates are needed

### Rust 2024 Edition Highlights
New features in Rust 2024:
- **Unsafe extern blocks**: `extern` blocks now require `unsafe` keyword for safety
- **Async closures**: Support for `async || {}` syntax
- **Better error handling**: Improved diagnostics with `#[diagnostic::do_not_recommend]`
- **Temporary scope changes**: Refined scoping for temporaries in `if let` and tail expressions
- **Enhanced prelude**: Adds `Future` and `IntoFuture` to standard prelude

### 4. Setup src/main.rs
Template structure (YOU must implement the Solution methods):

```rust
// # PROBLEM_ID PROBLEM_TITLE
// Problem description/requirements as comments

fn main() {
    // Example: Call Solution methods with test data
    // DO NOT solve the problem here - implement in Solution impl block
    let result = Solution::function_name(test_input);
    println!("{:?}", result);
}

struct Solution;

impl Solution {
    pub fn function_name(param: Type) -> ReturnType {
        // TODO: Implement your solution here
        // You must write the logic to solve the problem
        todo!()
    }
}
```

Components:
- **Problem comment**: `// # <ID> <Title>` at the top with problem description
- **main() function**: Calls Solution methods with sample test cases (setup only, NO solution logic)
- **Solution struct**: Holds all problem-solving implementations
- **Method signature**: Use standard LeetCode signature patterns
- **Implementation**: You write the actual algorithm/solution in the impl block

⚠️ **Critical**: The main() function just demonstrates HOW to call the methods, not the problem solution. All algorithm logic goes inside the `impl Solution` methods.

### 5. Verification
Check your Rust version (2024 edition requires Rust 1.85.0+):
```bash
rustup --version
rustup update  # Update to latest if needed
```

Build and run your project:
```bash
cargo build
cargo run
```

Both commands should execute without errors.

## Using External Dependencies
If your solution requires external crates:

1. Add to `Cargo.toml` under `[dependencies]`:
   ```toml
   [dependencies]
   regex = "1.5"
   ```
2. Import in `src/main.rs`:
   ```rust
   use regex::Regex;
   ```
3. Use within your `impl Solution` methods

## Quality Checklist
- [ ] Folder name follows `XXXX_problem_name` convention
- [ ] Package name is derived correctly (no ID prefix)
- [ ] `Cargo.toml` has all required sections with edition 2024 (or 2021 if on older toolchain)
- [ ] `src/main.rs` includes problem description as comment
- [ ] Solution struct is present and properly implemented
- [ ] main() function calls Solution methods with test cases
- [ ] Rust version is 1.85.0+ (for 2024 edition) or 1.56+ (for 2021 edition)
- [ ] `cargo build` completes without errors
- [ ] `cargo run` executes and produces expected output
