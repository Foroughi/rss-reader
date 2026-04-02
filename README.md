# rss-reader

A fast, terminal-based RSS/Atom feed reader built with Rust.

![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)
![TUI](https://img.shields.io/badge/TUI-ratatui-blue.svg)

## Features

- **RSS & Atom Support** - Reads both RSS 2.0 and Atom feed formats
- **Terminal UI** - Built with [ratatui](https://github.com/ratatui-org/ratatui) for a fast, responsive TUI experience
- **Vim-like Navigation** - `j`/`k` for navigation, `h`/`l` for pane switching
- **Feed Aggregation** - Combines multiple feeds with tagging and filtering
- **Article Scoring** - Items are ranked by score and date
- **Browser Integration** - Press `o` to open articles in your default browser
- **Nix Support** - Includes flake.nix for reproducible development environments

## Installation


### Build from Source

```bash
cargo build --release
./target/release/rss-reader
```

### Using Nix

```bash
nix profile add github:Foroughi/rss-reader
```

#### Prerequisites

Install Nix with flake support:

```bash
# On Linux/macOS with flakes enabled
nix --version  # Should be 2.4+

# If flakes are not enabled, add to ~/.config/nix/nix.conf:
# experimental-features = nix-command flakes
```

#### Development Shell

Enter a shell with Rust and all dependencies:

```bash
nix develop
cargo run
```

#### Build

Build the release binary:

```bash
nix build
./result/bin/rss-reader
```

#### Run Directly

Build and run without installing:

```bash
nix run
```

#### NixOS/Home Manager

Add to your NixOS configuration or home-manager:

```nix
# In your NixOS configuration.nix or home.nix
{ pkgs, ... }:
{
  environment.systemPackages = with pkgs; [
    (pkgs.rustPlatform.buildRustPackage {
      pname = "rss-reader";
      version = "0.1.0";
      src = /path/to/rss-reader;
      cargoLock.lockFile = /path/to/rss-reader/Cargo.lock;
    })
  ];
}
```

#### Updating Dependencies

```bash
nix flake update
```

## Configuration

On first run, the application creates a default config at:

- Linux/macOS: `~/.config/rss-reader/config.toml`

### Example Configuration

```toml
[[rss]]
url = "https://feeds.arstechnica.com/arstechnica/index"
tag = "Ars Technica"

[[rss]]
url = "http://rss.slashdot.org/Slashdot/slashdotMain"
tag = "Slashdot"

[[rss]]
url = "https://blog.rust-lang.org/feed.xml"
tag = "Rust Blog"

[[rss]]
url = "https://news.ycombinator.com/rss"
tag = "Hacker News"
```

## Usage

### Navigation

| Key | Action |
|-----|--------|
| `j` / `↓` | Move down |
| `k` / `↑` | Move up |
| `h` / `←` | Go to filters pane |
| `l` / `→` / `Enter` | Open article detail |
| `o` | Open article in browser |
| `q` | Quit |
| `Esc` | Back to list view |

### Filtering

Use the left sidebar to filter by feed source. Select "All" to see aggregated items from all feeds.

## Architecture

```
src/
├── main.rs           # Entry point, feed initialization
├── config.rs        # Configuration loading
├── app.rs           # Application state
├── domain/          # Domain models (Item, Feed)
├── sources/         # Feed source implementations (RSS, future: HN, Reddit, YouTube)
├── services/        # Business logic (aggregation, fetching, storage)
└── ui/              # TUI components
    ├── ui.rs        # Main UI loop and event handling
    ├── layout.rs   # Layout management
    └── components/ # UI widgets (sidebar, list, detail, header, statusbar)
```

## Dependencies

- **ratatui** - TUI framework
- **crossterm** - Terminal handling
- **tokio** - Async runtime
- **rss** - RSS parsing
- **atom_syndication** - Atom parsing
- **reqwest** - HTTP client
- **html2text** - HTML to plain text conversion

## License

MIT
