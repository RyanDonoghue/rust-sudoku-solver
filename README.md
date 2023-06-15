
# Rust Sudoku Solver

A slighly over engineered first exploration into the Rust language.

The solver uses backtracking to recursively solve any 9 x 9 sudoku board. A board is "solved" when the following criteria are met:

- All numbers on a vertical column are unique.
- All numbers on a horizontal column are unique.
- All numbers inside the 9 3x3 blocks of a sudoku grid are unique.

Using Google Protocol Buffers (pro)


## Versions

- [Rust (1.70)](https://www.rust-lang.org/tools/install)
- [Protobuf (3)](https://protobuf.dev/programming-guides/proto3/)
- [Apache Kafka (v2.13-3.5.9)](https://kafka.apache.org/downloads)
- Cargo (1.70)
- [pb-rs (0.10.0)](https://crates.io/crates/pb-rs)

## Dependencies

[quick-protobuf (0.8.0)](https://docs.rs/quick-protobuf/latest/quick_protobuf/)

[kafka (0.9.0)](https://docs.rs/kafka/0.9.0/kafka/)

[crossterm (0.26.1)](https://docs.rs/crossterm/0.26.1/crossterm/)
## Running locally

To run, there is an assumption you have Apache Zookeeper and Apache Kafka running on port 9092, and a new topic created called "sudoku-boards".

```
cargo run --manifest-path sudoku-kafka-producer/Cargo.toml 
cargo run --manifest-path sudoku-kafka-consumer/Cargo.toml 
```

The producer and consumer can be run in any order. The producer will create a hard coded sudoku puzzle, send this to Kafka and quit. The consumer will continuosly poll for new  messages (sudoku boards).

Live output of the backtracking solve can be enabled and timing of output adjusted (microseconds) by tweaking these values:

```rust
const PRINTING_ENABLED: bool = true ;
const PRINT_DELAY: u64 = 50;
```


## Considerations/workarounds

- Protobuf does not support n-dimensional structures (such as the 2D vector I used), so instead there must be a seperate SudokuRow (with repeated int32 as values) and SudokuBoard (repeated SudokuRow)
- Backtracking probably the fastest way of solving a sudoku puzzle, however it still has a large time complexity (O(K ^ N), and would usually have the potential for high space complexity. However as this solution is dealing with references passed recursively that should not be an issue.
- While initial implmentations of the solver used u8 values (unsigned 8-bit integers)
- I've added a duration output for only the solving algorithm, purely so I could compare to similar solutions in other languages
