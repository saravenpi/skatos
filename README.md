# Skatos 🛹

A fast, colorful Rust CLI tool for managing environment variables with YAML-based storage.

**Inspired by [Charm's skate](https://github.com/charmbracelet/skate)**, but reimplemented from scratch in pure Rust with enhanced features like color-coded output and shell autocompletion.

![Demo](demo.gif)

## Features

- Pure Rust implementation with YAML storage in `~/.skatos/`
- Beautiful color-coded output for better readability
- Shell autocompletion for bash, zsh, fish, elvish, and powershell
- Fast and lightweight
- Import capability from original skate

## Prerequisites

- Rust (for building from source)
- Optional: [Charm's skate](https://github.com/charmbracelet/skate) (only needed if you want to import existing data from skate)

## Installation

### One-line install (recommended)

```bash
curl -fsSL https://raw.githubusercontent.com/saravenpi/skatos/main/install.sh | bash
```

This will:
- Clone the repository
- Build the project with incremental compilation
- Install to `~/.local/bin/skatos`
- Show instructions to add `~/.local/bin` to your PATH

### Manual installation

```bash
git clone https://github.com/saravenpi/skatos.git
cd skatos
cargo build --release
cp target/release/skatos ~/.local/bin/
```

### From source (development)

```bash
cargo install --path .
```

### Add to PATH

If you used the one-line installer or manual installation, add `~/.local/bin` to your PATH:

```bash
# Add to your shell profile (~/.bashrc, ~/.zshrc, etc.)
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

## Usage

### Environment File Generation

Generate a `.env` file from all skate 🛹 variables:
```bash
skatos env
```

Generate to a specific file:
```bash
skatos env --output .env.production
```

Filter variables by prefix:
```bash
skatos env --filter "API_"
```

Generate from a specific database:
```bash
skatos env-from-db production --output .env.prod
```

### Preview

Preview environment variables without writing to file:
```bash
skatos preview
skatos preview --filter "DB_"
```

### Variable Operations

Set a variable:
```bash
skatos set API_KEY "your-api-key"
```

Get a variable:
```bash
skatos get API_KEY
```

List all variables:
```bash
skatos list
```

List only keys:
```bash
skatos keys
```

List databases:
```bash
skatos dbs
```

Delete a variable:
```bash
skatos delete API_KEY
```

### Backup & Restore

Backup all skatos data:
```bash
skatos backup --output backup.json
```

Restore from backup:
```bash
skatos restore backup.json
```

### Import from Charm's Skate

If you have existing data in Charm's skate, you can import it:
```bash
skatos import
```

Note: This requires the skate CLI to be installed.

### Shell Completions

Generate shell completions:
```bash
# For zsh
skatos completions zsh > ~/.zsh/completions/_skatos

# For bash
skatos completions bash > ~/.local/share/bash-completion/completions/skatos

# For fish
skatos completions fish > ~/.config/fish/completions/skatos.fish
```

## Examples

```bash
# Set some environment variables
skatos set DATABASE_URL "postgres://localhost/mydb"
skatos set API_KEY "sk-1234567890"
skatos set REDIS_URL "redis://localhost:6379"

# Generate .env file
skatos env

# Preview what would be generated
skatos preview

# Generate filtered env file
skatos env --filter "API" --output .env.api
```