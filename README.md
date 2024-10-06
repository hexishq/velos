![logo](https://github.com/user-attachments/assets/3114fb4d-db25-4f0c-9f1b-2fb2781ddd8a)

# Velos

Velos is a highly efficient and lightweight Solana data streaming client with a focused goal: **decoupling** the data stream. Sometimes, all you need is the raw data, and Velos delivers it faster and with less compute power than traditional methods.

The client is designed to minimize resource consumption while providing a direct stream of Solana shreds, transactions, and commitment levels. Whether you're streaming transactions through gRPC or embedding Velos as a crate in your Rust application, the goal remains the same—**getting the data you need, when you need it, with maximum efficiency.**

## Why Velos?

In many Solana use cases, full nodes are overkill. Velos simplifies things by decoupling the data stream from the broader Solana client. This allows developers and services to focus purely on receiving real-time transactions and commitment statuses without the overhead of a full node.

### Cost-Efficient Setup

Velos is designed to run on cost-effective VPS setups, making it accessible to a wide range of developers. You can expect to run Velos on a machine with specs around:

- **CPU:** 1-2 vCPUs
- **Memory:** 2-4 GB RAM
- **Storage:** 20 GB SSD
- **Network:** 1 Gbps connection recommended

This should be available from most VPS providers for **less than $20 USD per month**.

## Features

- **Decoupled Data Stream:** Focus purely on streaming Solana shreds and transactions without unnecessary compute overhead.
- **Minimal Compute Usage:** Velos is designed to consume the least possible processing power while delivering real-time data streams.
- **Modular:** Use Velos as either a gRPC-based transaction streamer or as a crate in your Rust applications.
- **Support for Commitment Levels:**
  - **Processed:** Stream transactions as soon as they are processed.
  - **Confirmed:** Stream transactions once 31 validators have voted to confirm them.
  - **Finalized:** Stream transactions after final confirmation.

## Installation (Coming Soon)

### As a Crate in Rust

Add Velos to your `Cargo.toml`:

```toml
[dependencies]
velos = "0.0.1"
```

### As a gRPC Service

Clone the repository:

```bash
git clone https://github.com/hexishq/velos.git
cd velos
```

Run the service (once available):

```bash
cargo run --release
```

## Roadmap

- [ ] Initial version of the data streaming client
- [ ] Fine-tune machine recommendations based on real-world usage
- [ ] Optimize gRPC streaming
- [ ] Expand configuration options for finer control over performance

## Contributing

Contributions are welcome! Please feel free to submit pull requests or open issues to suggest features or report bugs.

1. Fork the repository
2. Create a new feature branch (`git checkout -b feature-branch`)
3. Commit your changes (`git commit -am 'Add new feature'`)
4. Push the branch (`git push origin feature-branch`)
5. Create a Pull Request

## License

Velos is licensed under the **Apache 2.0 License**. See `LICENSE` for more information.

## Acknowledgments

Special thanks to the Solana developer community for providing the tools and inspiration to build Velos.
