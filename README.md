# Skatos

A simple Rust CLI tool to generate environment files from [skate](https://github.com/charmbracelet/skate) variables.

## Prerequisites

- [skate](https://github.com/charmbracelet/skate) must be installed on your system
- Rust (for building from source)

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

Generate a `.env` file from all skate variables:
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

### Skate Operations

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

Backup all skate data:
```bash
skatos backup --output backup.json
```

Restore from backup:
```bash
skatos restore backup.json
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