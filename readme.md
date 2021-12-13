[![codecov](https://codecov.io/gh/JosiahBull/rust-p2p-chat/branch/main/graph/badge.svg?token=G8ijVDm4rj)](https://codecov.io/gh/JosiahBull/rust-p2p-chat)
![Build](https://github.com/JosiahBull/festival-api/actions/workflows/test.yml/badge.svg)
[![Docs](https://github.com/JosiahBull/festival-api/actions/workflows/docs.yml/badge.svg)](https://josiahbull.github.io/festival-api/festival_api/index.html)

# P2P Chat Client

This is a simple demonstration of a peer to peer chat client, written entirely in rust utilising the libp2p library.

## Demo

On two seperate computers connected to the same LAN, run:
```sh
git clone https://github.com/JosiahBull/rust-p2p-chat
cd rust-p2p-chat
cargo run
```

The peers should discover one another, and you can message between them. The last 32 messages of history are stored if one peer disconnects.

## Test
```sh
cargo test
```