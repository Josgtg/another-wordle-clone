# another-wordle-clone

A terminal Wordle clone: guess a hidden five-letter word in English or Spanish, and get
colored feedback for every letter of each guess.

## Use the application

Releases are published on the
[releases page](https://github.com/Josgtg/another-wordle-clone/releases); the current
one is [v0.1.0](https://github.com/Josgtg/another-wordle-clone/releases/tag/v0.1.0).

Download
[`another-wordle-clone-0.1.0-x86_64-unknown-linux-gnu.zip`](https://github.com/Josgtg/another-wordle-clone/releases/download/v0.1.0/another-wordle-clone-0.1.0-x86_64-unknown-linux-gnu.zip)
for Linux x86_64. The archive unpacks to a directory holding the executable
`another_wordle_clone`, this README, the license and a `SHA256SUMS` file:

```sh
unzip another-wordle-clone-0.1.0-x86_64-unknown-linux-gnu.zip
cd another-wordle-clone-0.1.0
sha256sum -c SHA256SUMS
./another_wordle_clone
```

The game is fully interactive and needs a terminal: it reads every guess through a prompt
that queries the terminal (including its cursor position), so it cannot be driven by
piping input into it.

### Build from source

You can also build from the source. Requires a Rust toolchain with edition 2021 support.

Build:

```sh
cargo build --release
```

Run:

```sh
./target/release/another_wordle_clone
```

To put the binary on your `PATH`:

```sh
cargo install --path .
```

## Usage

```text
Usage: another_wordle_clone [OPTIONS]
```

| Flag | Meaning |
| --- | --- |
| `-l`, `--language <LANGUAGE>` | Language of the game and its word list: `en` / `english` or `es` / `español`. Defaults to Spanish. An unrecognized value falls back to Spanish. |
| `-s`, `--secret <SECRET>` | Use `SECRET` as the hidden word instead of a random one. It must be exactly five characters; otherwise a message is printed and a random word is used instead. |
| `-t`, `--tries <TRIES>` | Maximum number of guesses. Defaults to 5. |
| `-h`, `--help` | Print the help screen and exit. |

```sh
another_wordle_clone
another_wordle_clone -l en
another_wordle_clone -l en -t 7
another_wordle_clone -s about -t 3
```

There is no version flag.

## Gameplay

The game picks a five-letter secret word and lets you guess it. Press Enter to get past
the welcome screen, then type a guess and press Enter. The letters of the guess are shown
in color:

- **Green**: the letter is in the secret word at that position.
- **Yellow**: the letter is in the secret word, but at another position.
- **Red**: the letter is not in the secret word.

Repeated letters are counted, not just matched: a letter is marked yellow only as many
times as it still appears in the secret word, so a guess with an extra copy of a letter
shows the surplus copy as red. The remaining tries are printed after every guess that
does not end the round.

A guess is rejected — and does not use up an attempt — when it is not exactly five
characters long, when it contains a character that is not part of the language's
alphabet, or when it is not in the bundled dictionary for that language. A secret word
given with `--secret` is added to that dictionary, so it can itself be guessed.

You win when the guess matches the secret word; the game then congratulates you and shows
the word. If you use up every try, the game reveals the secret word.

### In-game commands

While the game asks for a guess, one of the single letters below is treated as a command
instead of a guess. Commands are case-insensitive.

| Command | Action |
| --- | --- |
| `a` | Show the alphabet with the hints revealed so far |
| `w` | Show the feedback for the previous guess |
| `l` | Show every word guessed so far |
| `t` | Show how many tries are left |
| `h` | List the available commands |
| `q` | End the current round (you are then asked whether to play again) |

### Languages

English and Spanish are supported, each with its own word list, dictionary and messages.
The default is Spanish; choose English at launch with `-l en`, or Spanish with `-l es`.

You can also switch after a finished round: answer `y` to "Play again?", then `y` to
"Change language?" and pick the language from the list. The new language starts with a
fresh round, including its welcome screen.

Spanish secret words may contain accents, and they can be typed without them: a guess of
`arbol` matches the secret word `árbol`. The secret word is displayed with its accents
when revealed.

### Quitting

At the guess prompt, `q` ends the current round and the game asks "Play again?". Answer
`n` to leave. Answering `n` to "Play again?" after any round also exits the game.

## Tech stack

- Rust 2021.
- [`clap`](https://docs.rs/clap) parses the command line and renders the help screen.
- [`promkit`](https://docs.rs/promkit) provides the interactive prompts for guessing,
  confirming and choosing a language.
- [`colored`](https://docs.rs/colored) colors the letter feedback.
- [`rand`](https://docs.rs/rand) picks the secret word.
- [`strum`](https://docs.rs/strum) iterates over the supported languages.
- The word lists under [`media/`](media/) are embedded in the binary at compile time.

## License

MIT License, Copyright (c) 2025 Josué. See [LICENSE.txt](LICENSE.txt).
