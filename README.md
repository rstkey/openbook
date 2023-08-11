# OpenBook V2

⚠️ Disclaimer: The current code is being audited. Use it at your own risk!

A central-limit order-book program based on [Mango V4](https://github.com/blockworks-foundation/mango-v4) and the [previous OpenBook program](https://github.com/openbook-dex/program) (which was a fork of [Serum](https://github.com/project-serum/serum-dex)).

## License

See the [LICENSE file](LICENSE).

The majority of this repo is MIT-licensed, but some parts needed for compiling
the Solana program are under GPL.

All GPL code is gated behind the `enable-gpl` feature. If you use the `openbook-v2`
crate as a dependency with the `client` or `cpi` features, you use only MIT
parts of it.

The intention is for you to be able to depend on the `openbook-v2` crate for
building closed-source tools and integrations, including other Solana programs
that call into the Openbook program.

But deriving a Solana program with similar functionality to the Openbook program
from this codebase would require the changes and improvements to stay publicly
available under GPL.

## Deployed versions

| tag       | network | program ID                                   |
| --------- | ------- | -------------------------------------------- |
| v0.1      | devnet  | opnbkNkqux64GppQhwbyEVc3axhssFhVYuwar8rDHCuA |
| v0.1      | testnet | opnbkNkqux64GppQhwbyEVc3axhssFhVYuwar8rDHCuA |
| test-v0.6 | devnet  | 8qkavBpvoHVYkmPhu6QRpXRX39Kcop9uMXvZorBAz43o |
| test-v0.6 | testnet | 8qkavBpvoHVYkmPhu6QRpXRX39Kcop9uMXvZorBAz43o |
| test-v0.6 | mainnet | 8qkavBpvoHVYkmPhu6QRpXRX39Kcop9uMXvZorBAz43o |

## Building & testing

### Pre-requisites

Before you can build the program, you will first need to install the following:

- [Rust](https://www.rust-lang.org/tools/install)
- [Solana](https://docs.solana.com/cli/install-solana-cli-tools)
- [Anchor](https://www.anchor-lang.com/docs/installation) (v0.27.0)
- [Just](https://github.com/casey/just#installation)

### Installing

To install the repo, run:

```bash
git clone https://github.com/openbook-dex/openbook-v2.git --recursive
```

The recursive flag ensures that you receive all of the submodules. If you have already cloned without passing in this flag, you can run:

```bash
git submodule init
git submodule update
```

To ensure that you always have the latest submodules, you can configure your git like so:

```bash
git config --global submodule.recurse true
```

### Building

To build, run:

```bash
just build
```

### IDL

To generate the progam & typescript IDLs, run:

```bash
just idl
```

### Testing

To see whether all of the tests are passing, run:

```bash
just test-all
```

To drill down on a specific test (e.g., test_expired_order), run:

```bash
just test test_expired_order
```

If you want to have tests that automatically re-run when you edit a file, install
[entr](https://github.com/eradman/entr) and run:

```bash
just test-dev
```

### TS Client

```bash
yarn build
```
