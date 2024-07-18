
# install rust

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

source ~/.bashrc

https://solana.com/developers/guides/getstarted/setup-local-development

sudo apt-get install -y \
    build-essential \
    pkg-config \
    libudev-dev llvm libclang-dev \
    protobuf-compiler libssl-dev

## install solana

sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
-> failed
https://github.com/solana-labs/solana/issues/24415


sudo apt-get update
sudo apt-get install libssl-dev libudev-dev pkg-config zlib1g-dev llvm clang cmake make libprotobuf-dev protobuf-compiler

git checkout refs/tags/v1.18.18

./cargo build

solana --version


https://solanacookbook.com/ja/getting-started/installation.html#macos-linux


# Dockerfile で --platform=linux/amd64 を指定しないといけない。


tar jxf solana-release-x86_64-unknown-linux-gnu.tar.bz2
cd solana-release/
export PATH=$PWD/bin:$PATH



cargo install --git https://github.com/coral-xyz/anchor avm --locked --force

avm install latest

avm use latest


Extract the zip file in your project's directory and run:

```
yarn
```

Build

```
anchor build
```

Test

```
anchor test
```

Run client

```
anchor run client
```



cargo install --git https://github.com/project-serum/anchor --tag v0.29.0 anchor-cli --locked

https://solana.com/developers/guides/getstarted/setup-local-development

cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install latest
avm use latest





cargo install --git https://github.com/project-serum/anchor --tag v0.30.1 anchor-cli --locked



```
solanauser ➜ /src $ solana --version
solana-cli 1.18.18 (src:83047136; feat:4215500110, client:SolanaLabs)
solanauser ➜ /src $ yarn -v
1.22.22
solanauser ➜ /src $ node -v
v20.15.1
solanauser ➜ /src $ anchor --version
anchor-cli 0.29.0
```
