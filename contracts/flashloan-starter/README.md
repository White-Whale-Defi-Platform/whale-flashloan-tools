# CosmWasm Minimal Starter Pack


This template contains almost the absolute minimum you need to get going with a CosmWasm smart contract in Rust. There is a specific focus put on it to be very barebones and this template is based on the [cw-template](https://raw.githubusercontent.com/CosmWasm/cw-template) by CosmWasm.

This is an opinionated template to build smart contracts in Rust to run inside a
[Cosmos SDK](https://github.com/cosmos/cosmos-sdk) module on all chains that enable it.
To understand the framework better, please read the overview in the
[cosmwasm repo](https://github.com/CosmWasm/cosmwasm/blob/master/README.md),
and dig into the [cosmwasm docs](https://www.cosmwasm.com).
This assumes you understand the theory and just want to get coding.

## Creating a new repo from template

Assuming you have a recent version of rust and cargo (v1.51.0+) installed
(via [rustup](https://rustup.rs/)),
then the following should get you a new repo to start a contract:

First, install
[cargo-generate](https://github.com/ashleygwilliams/cargo-generate).
Unless you did that before, run this line now:

```sh
cargo install cargo-generate --features vendored-openssl
```

Now, use it to create your new contract.
Go to the folder in which you want to place it and run:


**Latest: 0.16**

```sh
cargo generate --git https://github.com/0xFable/cw-minimal-template.git --name PROJECT_NAME
````

You will now have a new folder called `PROJECT_NAME` (I hope you changed that to something else)
containing a simple working contract and build system that you can customize.