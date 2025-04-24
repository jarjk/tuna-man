# 🏓 Tuna Man: tournament manager

> NOTE: WIP

**Tuna Man** is a Rust-powered CLI/TUI application that creates and manages tournaments, initially built for table tennis.
Whether you're organizing a casual game night or a competitive event, it helps you manage tournaments with ease. 

## Features

- 💾 **CSV-Based Input**: Easily import players or teams from a `.csv` file.
- 🔓 **Flexibility**: It can be used for tournaments of any sport.
- 🏆 **Multiple Formats**: Can automatically create brackets for multiple tournament formats.

> **_TODO_**
> - [ ] 🖥️ **TUI interface**: [Ratatui](https://ratatui.rs) integration for a sleek terminal user interface is in the works.
> - 🔄 **More Tournament formats**: Upcoming support for multiple tournament formats eg.:
>   - [x] double-elimination
>   - [x] single-elimination
>   - [x] Round-robin
>   - [ ] Swiss-system
>   - [ ] more?
>   - [ ] seeding where applicable
> - [ ] library?

## Getting Started

### Prerequisites

- **Rust** installed (<https://www.rust-lang.org/tools/install>)
- A CSV file with participants in `<player/team>,<class>` format (where `<class>` is optional, consists of an integer \[0,255\] and a character, eg: '12Z')

### Installation

Clone the repository:

```bash
git clone https://codeberg.org/jark/tuna-man.git
cd tuna-man
```

Build the project:

```bash
cargo build --release
```

All-in-one easy mode:  
```bash
cargo install --locked --git "https://codeberg.org/jark/tuna-man"
```

### Usage

To create a tournament, simply run the following command, providing the path to your `.csv` file:

```bash
tuna-man <FILE>
```

- example input file without class
  ```csv
  name
  Alice
  Bob
  Jennice
  ...
  ```
- example input file with class
  ```csv
  name,class
  Alice,11A
  Bob,9B
  Jennice,0C
  ...
  ```

### Notable Options:

- `-h`, `--help`: Display help message with usage details.

## alternatives

*well*: these are proprietary, quite resource heavy, need a browser, internet connection, an account, not all features are free,  
**but**: they're far more feature-rich, easy-to-use, no need for installation, actually usable at this very moment

- [commoninja brackets](https://www.commoninja.com/brackets/editor/participants)
- [scoreholio](https://app.scoreholio.com)

