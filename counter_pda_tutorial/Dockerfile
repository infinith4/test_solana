# FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye
FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye
# FROM mcr.microsoft.com/devcontainers/typescript-node:1-22-bookworm
# [Optional] Uncomment this section to install additional OS packages.
# RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
#     && apt-get -y install --no-install-recommends <your-package-list-here>

# [Optional] Uncomment if you want to install an additional version of node using nvm
# ARG EXTRA_NODE_VERSION=10
# RUN su node -c "source /usr/local/share/nvm/nvm.sh && nvm install ${EXTRA_NODE_VERSION}"

# [Optional] Uncomment if you want to install more global node modules
# RUN su node -c "npm install -g npm@10.8.0"

RUN apt-get update && apt-get upgrade
RUN apt-get install -y openssh-server sudo curl nano git
RUN apt-get install -y pkg-config build-essential libudev-dev libssl-dev libssl-dev

RUN su node -c "curl https://sh.rustup.rs -sSf | sh"
RUN source /home/.cargo/env
RUN rustup component add rustfmt
RUN rustup update

# RUN useradd -rm -d /home/ubuntu -s /bin/bash -g root -G sudo -u 1010 sshuser 

# RUN usermod -aG sudo sshuser

# RUN service ssh start

# RUN echo 'sshuser:password' | chpasswd

# COPY solana-install.sh /home/ubuntu
# # 実行権限を付与
# RUN chmod +x /home/ubuntu/solana-install.sh
# # シェルスクリプト実行
# RUN /home/ubuntu/solana-install.sh

# EXPOSE 22

# CMD ["/usr/sbin/sshd","-D"]

# RUN su node -c "$(curl -sSfL https://release.solana.com/v1.18.18/install)"
# RUN RUST_BACKTRACE=full cargo install --git https://github.com/project-serum/anchor anchor-cli --locked
# RUN cargo install --git https://github.com/coral-xyz/anchor --tag v0.29.0 anchor-cli --locked
# RUN ["/bin/bash", "-c", "$(curl -sSfL https://release.solana.com/v1.18.18/install)"]
# CMD ["avm", "install", "latest"]
# CMD ["avm", "use", "latest"]