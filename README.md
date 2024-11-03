![thumb](https://github.com/user-attachments/assets/900efffb-c453-44c1-bda8-a22fb5ce0dfa)
![velos](https://github.com/user-attachments/assets/bdc33f74-3873-4a35-8362-3855fd4729ff)

# Velos

Velos is a specialized data streaming client for Solana that dramatically reduces infrastructure costs through efficient decoupling of the data reception layer. 

By focusing solely on processing shreds, verifying them, constructing entries, and receiving gossip votes for commitment tracking, Velos provides a lightweight solution that can run on minimal hardware while maintaining full data parity.

## Why Velos?

Traditional access to real-time Solana data requires running full nodes with complete runtime and related services - excessive overhead when your goal is data streaming. Velos solves this by:

- **Focused Architecture:** Processes only essential data components
- **Minimal Resources:** Runs on lightweight infrastructure
- **Cost Efficiency:** Reduces infrastructure costs by 50x
- **Global Scalability:** Deploy multiple instances easily
- **Zero DevOps:** Simple setup and maintenance

### For Institutions
- Deploy globally with minimal costs
- Superior scalability with lightweight instances
- Simple redundancy across regions
- Full data parity without infrastructure complexity

### For Developers
- Efficient data streaming without full node overhead
- Minimalist approach focused on essential data flow
- Zero infrastructure knowledge needed
- Focus on building, not maintenance

## Features

- **Optimized Data Reception:**
  - Direct shred processing
  - Entry construction
  - Transaction streaming
  - Commitment tracking via gossip
  
- **Efficient Architecture:**
  - Minimal resource consumption
  - Streamlined data flow
  - High-performance processing

- **Developer Tools:**
  - gRPC API
  - Rust crate integration
  - Simple configuration
  - Plugin system (coming soon)

## Installation (Coming Soon)

### As a Crate
```toml
[dependencies]
velos = "0.0.1"
```

### As a Service
```bash
git clone https://github.com/hexishq/velos.git
cd velos
cargo run --release
```

## Roadmap

Phase 1: v0 - Core Data Streaming (Q4 2024)
- [ ] Gossip Protocol Connection
- [ ] Turbine Integration
  - [ ] Shred reception and verification
  - [ ] Entry processing
  - [ ] Transaction streaming
- [ ] Jito Integration
- [ ] gRPC Implementation

Phase 2: v1 - Plugin System
- [ ] Geyser Interface Layer
- [ ] Adaptable Plugin Architecture
- [ ] Extended API Support

## Contributing

We welcome contributions! Feel free to:
- Open issues for bugs or feature requests
- Submit pull requests
- Join discussions
- Share feedback

## License

Velos is licensed under the Apache 2.0 License. See `LICENSE` for details.

## Acknowledgments

Built with inspiration from the Solana community and a commitment to making blockchain infrastructure more accessible for everyone.
