# Substrate Vanity Address Generator

This project is a Substrate vanity address generator written in Rust. It generates random Substrate addresses in parallel until it finds one that matches a provided regex pattern. The program prints the matching address, its corresponding seed, and the total number of addresses generated.

# Substrate addresses can be used with

- Polkadot
- Bittensor
- Kusama
- Edgeware
- Acala Network
- Moonbeam
- Phala Network
- Plasm Network
- Centrifuge
- ChainX
- Subsocial
- Darwinia Network
- Clover Finance
- Crust Network
- Robonomics
- HydraDX


## Features

- Multi-threaded address generation using Rust's threading capabilities.
- Generates addresses using a seed instead of a mnemonic to increase speed.
- Handles clean exit and prints the count of generated addresses on interruption (Ctrl-C).
- Allows specifying the regex pattern for matching addresses via command-line arguments.

## Prerequisites

- Rust and Cargo installed. Follow the instructions on [rust-lang.org](https://www.rust-lang.org/learn/get-started) to install Rust.

## Installation

Clone the repository:

```sh
git clone https://github.com/Quint-S/substrate-vanity-address.git
cd substrate-vanity-address
```

## Usage
Run the program with the desired regex pattern as a command-line argument. For example, to find addresses containing "pepe" or "frog":

```sh
cargo run -- "(?i)(pepe|frog)"
```
The (?i) at the beginning of the regex pattern makes it case-insensitive. 

## Example output
<pre>
Generated: 21411 addresses
Vanity Address Found: 5GVwzVG9QtodPvb5rA9<b>FroG</b>61DLfac7qxeDEvLGqpnF3kLU9
Secret Seed: 0x45ac89ffdcf78d2ef0f09186ba1193ed26071a0c7f428fc414757b7e39babe60
Time elapsed: 339.11ms
</pre>
