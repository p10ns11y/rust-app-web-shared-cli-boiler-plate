## Overview

Simple template template to create new projects with a workspace and various type of packages including application, shared lib, web and cli, one_off scripts, etc.

## Project Structure

```plaintext
.
├── Cargo.lock
├── Cargo.toml
├── README.md
├── cli
│   ├── Cargo.toml
│   └── src
│       └── main.rs
├── devtools
│   ├── Cargo.toml
│   └── src
│       └── main.rs
├── shared
│   ├── Cargo.toml
│   └── src
│       ├── lib.rs
│       └── network.rs
├── src
│   ├── config.rs
│   ├── helpers.rs
│   ├── main.rs
│   ├── mod.rs
│   └── one_off_off_by_one_bit_script.rs
└── web
    ├── Cargo.toml
    └── src
        └── main.rs
```

## Manual preparation

In case you are curious how the project was created, here are the steps:

#### Steps

1. Initialize workspace

```bash
cargo new <root_project_name>
cd <root_project_name>
```

2. Update Root Cargo.toml

```toml
[workspace]
members = [
    "."  # Include root project
]
```

3. Create Shared Library

`cargo new --lib shared`

it update Root Cargo.toml to Include Shared automatically

```toml
[workspace]
members = [
   ".",
   "shared"
]
```

4. Create other packages, apps, libraries, etc.

```bash
cargo new cli
cargo new web
...
```

5.Configure Binary Cargo.tomls Each binary's Cargo.toml needs:

```toml
[dependencies]
shared = { path = "../shared" } # in root toml it is "./shared"
```

6. Verify the workspace

```bash
cargo check
cargo verify-project
```

### Running the projects

```bash
# These share the same root level Cargo.toml and dependencies
cargo run         # Runs root project
cargo run --bin one_off  # Runs one_off standalone script


# These projects use their own dependencies and can re-specify root dependencies
cargo run --bin devtools  # Runs devtools project
cargo run -p cli  # Runs cli project
cargo run -p web  # Runs web project
```

#### Credits

- To [Claude](claude.ai)

#### References

- [Rust Workspace](https://doc.rust-lang.org/cargo/reference/workspaces.html)
