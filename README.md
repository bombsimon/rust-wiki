# rust-wiki

Wikipedia with Rust!

A command-line interface (CLI) tool to extract the excerpt from [Wikipedia](https://en.wikipedia.org) articles.

This code does:
* Simple searches with the Wikipedia API
* Blindly try to print title and extract of article

This code does **not**:
* Handle ambigous words
* Notify about redirets
* Handle missing articles
* Handle errors in the HTTPS request
* Most of the errors than can occur

All searches will be made towards the English version of Wikipedia. The user has to handle multi word searches by passing them quoted to the executable.

## Installation

Install Rust and Cargo:

```
$ curl https://sh.rustup.rs -sSf | sh
```

Install rust-wiki:
```
$ cargo install
```

## Usage

```
$ rust-wiki
Usage: rust-wiki <topic>

$ rust-wiki Shoes
Shoe
A shoe is an item of footwear intended to protect and comfort the human foot while the wearer is doing various activities.

$ rust-wiki "End of the universe"
Ultimate fate of the universe
The ultimate fate of the universe is a topic in physical cosmology, whose theoretical restrictions allow possible scenarios for the evolution and ultimate fate of the universe to be described and evaluated.
```
