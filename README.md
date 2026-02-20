# Daily Kanji Practice (TUI)

A minimal terminal-based tool to practice Kanji daily and track your performance over time.

Designed for consistency. Runs from your terminal, integrates easily into your shell startup, and generates PDF reports with detailed results (correct and wrong answers).

## Features

- Daily Kanji practice in a clean TUI
- Lightweight and fast (written in Rust)
- Automatic scoring
- PDF report generation with:
  - Your answers
  - Correct answers
  - Mistakes and score summary
- Easy integration into `.bashrc`, `.zshrc`, or `.config/nushell/config.nu`

## Preview

![TUI Demo](https://vhs.charm.sh/vhs-3mqNiUlQyvxoO8DQgrAQhx.gif)

## Installation

### Option 1 — Using Cargo (Recommended)

```bash
cargo install --git https://github.com/Vaishnav-Sabari-Girish/Daily-Kanji
````

### Option 2 — Build From Source

```bash
git clone https://github.com/Vaishnav-Sabari-Girish/Daily-Kanji.git
cd Daily-Kanji
cargo run --release      # Run without installing
cargo install --path .   # Install to ~/.cargo/bin
```

## Usage

Run the application from your terminal:

```bash
daily_kanji
````

By default, the app runs only once per day (triggered on your first terminal session of the day).

To run it multiple times in the same day, use the `--test` flag:

```bash
daily_kanji --test
```

> [!IMPORTANT]
> Using the `--test` flag reduces the number of questions from 15 (default) to 5.

To practice automatically every day, add it to your shell config

> [!NOTE]
> Runs once per day, triggered on your first terminal session of the day.

### Bash / Zsh

```bash
# ~/.bashrc or ~/.zshrc
daily_kanji
```

### Nushell

```nu
# ~/.config/nushell/config.nu
daily_kanji
```

This will prompt you with a daily Kanji session whenever a new terminal session starts.

## Reports

After each session, a PDF report is generated containing:

* Your score
* Correct responses
* Incorrect responses for review

This helps track progress and identify weak Kanji over time.

## Requirements

* Rust (latest stable recommended)
* A terminal that supports TUI rendering


## Project Goals

This tool focuses on:

* Daily repetition instead of bulk memorization
* Minimal friction (terminal-first workflow)
* Long-term retention through consistent exposure

## Contributing

Pull requests, issues, and feature suggestions are welcome.
If you find bugs or want new practice modes (JLPT levels, custom decks, spaced repetition), open an issue.

## License

[MIT License](./LICENSE)

## Changelog

[CHANGELOG.md](./CHANGELOG.md)

