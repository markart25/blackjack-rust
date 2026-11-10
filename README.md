
# rust blackjack

simple terminal blackjack game written in rust
very minimal amounts of AI was used in this. (no copy and pasting chunks of code)

features:
- fully working cli game
- all realistic logic like:
    - double down
    - natural blackjack vs with hit
    - 4 decks of cards that reshuffel at 14 cards or less
- chips

## how to Run

1. clone the repository:
   ```bash
   git clone https://github.com/markart25/blackjack-rust.git
   cd blackjack-rust
   ```

2. make sure you have Rust and Cargo installed. If not install via package maneger (pacman in the example), or see [rustup.rs](https://rustup.rs/) for macOS/Windows instructions.
   ```bash
    sudo pacman -S rustup
    rustup default stable
   ```

3. run the game:
   ```bash
   cargo run
   ```

Cargo will automatically download dependencies (`rand`) and compile the project the first time you run it.

[![MIT License](https://img.shields.io/badge/License-MIT-green.svg)](https://choosealicense.com/licenses/mit/)


