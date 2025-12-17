# pkg-latest

A simple CLI tool to resolve the latest version of npm and jsr packages. Useful for Deno commands where you want to always use the latest version without manually checking.

## Installation

```bash
cargo install pkg-latest
```

Or build from source:

```bash
cargo build --release
```

## Usage

`pkg-latest` takes a package specifier (with optional `npm:` or `jsr:` prefix) and outputs the same specifier with the latest version appended. If no prefix is provided, it defaults to `npm:`.

```bash
latest npm:@google/gemini-cli
# Output: npm:@google/gemini-cli@1.2.3

latest @google/gemini-cli
# Output: npm:@google/gemini-cli@1.2.3

latest jsr:@sigma/bisect
# Output: jsr:@sigma/bisect@0.5.0
```

### Use with Deno

The primary use case is with Deno command substitution:

```bash
# Fish shell
deno run -A (latest jsr:@sigma/bisect)
deno run --no-config -A (latest npm:@google/gemini-cli) $argv

# Bash/Zsh
deno run -A $(latest jsr:@sigma/bisect)
deno run --no-config -A $(latest npm:@google/gemini-cli) "$@"
```

This ensures you always run the latest version without having to manually update version numbers.

## How it works

- For npm packages: queries the npm registry at `https://registry.npmjs.org/`
- For jsr packages: queries the JSR registry at `https://jsr.io/`
- Fetches the `latest` dist-tag/version and appends it to the package specifier

## Dependencies

Minimal dependencies for fast compilation and small binary size:
- `ureq` - lightweight HTTP client
- `serde` - JSON parsing

## License

MIT
