# Thunder in Rust

This is a game developed by the Rust game engine Bevy. This game is a redevelopment of the classic game Thunder.

## Web version

Visit this website to play the game: [Thunder](https://bucket-xv.github.io/Thunder-in-Rust/).

Doing so will pull about 40MB of game data from the repository, so please be patient.

Note that `Quit` option is invalid in the web page version.

## Local version

Coming soon.

## Development

### Prerequisites

You should have Rust installed on your computer. If you don't have it, you can install it by following the instructions on the [Rust website](https://www.rust-lang.org/learn/get-started).

<!-- Then you can install the Bevy game engine by running the following command: -->

### Clone the repository

```bash
git clone https://github.com/bucket-xv/Thunder-in-Rust.git
```

### Run the game

```bash
cargo run --release
```

### Generate the web version

If you are using Windows, run the following command:

```bash
make win-web
```

If you are using Linux, run the following command:

```bash
make linux-web
```

## How to play

- Use the arrow keys or `w`,`a`,`s` and `d` to move the player.
- Avoid being shot by the enemy and get the highest score as possible.

## Report

The game is still in development, so there may be some bugs. If you find any bugs, please report them to us.
