# rust-craft

rust-craft is a collection of crates to make minecraft development (client, server) with rust possible.

### Motivation

There's no better way of learning proc-macros and feature-flags by implementing them for a game you like (and also learning more about its packets on the way).

### Crates

| Name  | Description |
| - | - |
| protocol-core  | Packet related abstraction traits and implementations for common types |
| protocol-derive  | Proc-macros responsible for implementing Readable, Writeable and Packet traits for structs automatically via `#[derive(Packet)]` |
| protocol-packets | Packet structs and documentation |

### Examples

Examples can be found on the folder `/examples`, more examples will come when possible.

### Desired Features
Not in order.

- [x] Handshake packet definitions. (protocol-packets)
- [x] Status packet definitions. (protocol-packets)
- [ ] Login packet definitions. (protocol-packets)
- [ ] Play packet definitions. (protocol-packets)
- [ ] Connection wrapper that stores data (state, compression) and send/receive packets. (protocol-packets)
- [ ] Nbt type (like Json). (protocol-core)
- [ ] ChatComponent type. (protocol-packets)

### Inspiration

- [feather-rs (inactive)](https://github.com/feather-rs/feather)
- [valence](https://github.com/valence-rs/valence)

### License

This project is licensed under the [MIT license].

[MIT license]: https://github.com/devwckd/rust-craft/blob/main/LICENSE

### Contributions

PRs are welcome :D