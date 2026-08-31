# Papayoo

## Game rules

Papayoo is a card game. There're the 4 classic suits, Spades, Hearts, Diamonds, Clubs, and a special one named Joker.
There're several cards which worth a certain points amount :
- Jokers -> from 1 to 20 points
- Payoo -> a 7 of any of the 4 suits, random chosen at the round start, which worth 40 points

The game consists of 3 rounds. The player with the smaller score at the end win the game.

## Project structure

```bash
.
├── src
│   ├── card.rs
│   ├── deck.rs
│   ├── game.rs
│   ├── main.rs
│   ├── player.rs
│   └── render.rs
├── .gitignore
├── Cargo.toml
└── README.md
```

## UI

Papayoo's UI is written in French.

The game contains 1 human player and 3 AI players.

© Picot Solal
