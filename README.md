# Rusty Genesis

`vecnod-genesis-generator` is a Rust implementation for mining and handling the genesis block of a blockchain. This tool demonstrates how to create, mine, and validate the first block in a blockchain system.

## Features

- **Genesis Block Mining**: Mines the initial block of a blockchain with customizable difficulty.
- **Merkle Root Calculation**: Computes Merkle root from transaction data.
- **Block Header Serialization**: Serializes block headers for hashing.
- **Difficulty Adjustment**: Calculates mining difficulty based on target bits.

## Installation

To get started with `vecnod-genesis-generator`, you'll need to have Rust installed. If you don't have Rust installed, you can get it from [rust-lang.org](https://www.rust-lang.org/tools/install).

1. Clone the repository:

   `git clone https://github.com/Vecno-Foundation/vecnod-genesis-generator.git`
   `cd vecnod-genesis-generator`

2. Install dependencies:

   `cargo build`

## Usage

Run the program with:

`cargo run`

This will attempt to mine the genesis block with the predefined parameters in the `main` function. Here's what it does:

- It sets up a timestamp, difficulty target, and constructs a coinbase transaction.
- Calculates the Merkle root from the transaction(s).
- Mines the block by finding a nonce that results in a hash below the difficulty target.
- Prints out the block hash and other configuration details if successful.

## Configuration

The main configurations can be found in the `main` function:

- **Timestamp**: The start time for the block, in milliseconds since UNIX EPOCH.
- **Difficulty Bits**: Defines the mining difficulty, affecting how hard it is to find a valid block hash.
- **Coinbase Payload**: Data included in the coinbase transaction, which here includes a custom message.

Modify these values if you need different parameters for your blockchain's genesis block.

## Structure

- `main.rs`: Contains the primary logic for block mining and configuration printing.
- **Dependencies**:
  - `hex`: For encoding/decoding hexadecimal strings.
  - `sha2`: For SHA-256 hashing operations.
  - `num-bigint`: For handling large integers in difficulty calculations.

## TODO

- Expand to handle multiple transactions in the block.
- Implement a more dynamic difficulty adjustment algorithm.
- Add support for different hashing algorithms.

## Contributing

Contributions are welcome! Please fork the repository and submit pull requests. Ensure your code follows Rust conventions and includes appropriate tests.

## License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.