# goap-rs

`goap-rs` is a Rust library that implements a goal-oriented action planning system. This library is useful for developing AI systems where actions need to be planned and executed based on various conditions. Mainly designed to be used in games.

## GOAP

Goal-Oriented Action Planning (GOAP) is a sophisticated AI planning method that was likely introduced by Dr. Alexander Nareyek around 2003 [^1] and later popularized by Dr. Jeff Orkin [^2].

## Features

* Property and Condition Management: Create and manage properties and conditions using unique identifiers.
* State Management: Easily manage states using a set of conditions.
* Action Execution: Define and execute actions with configurable preconditions and effects.

## Usage

To use `goap-rs` in your project, add it as a dependency in your Cargo.toml:

```toml
[dependencies]
goap-rs = "0.0.1"
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

[^1]: ["The 2004 Report of the IGDA's Artificial Intelligence Interface Standards Committee"](https://www.researchgate.net/publication/274541361_The_2004_Report_of_the_IGDA%27s_Artificial_Intelligence_Interface_Standards_Committee)
[^2]: ["Applying Goal-Oriented Action Planning to Games"](https://web.archive.org/web/20230912173044/https://alumni.media.mit.edu/~jorkin/GOAP_draft_AIWisdom2_2003.pdf)
