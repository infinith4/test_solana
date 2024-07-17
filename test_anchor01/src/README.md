
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