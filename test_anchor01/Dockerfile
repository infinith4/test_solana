FROM --platform=linux/amd64 mcr.microsoft.com/devcontainers/base:jammy

# [Optional] Uncomment this section to install additional OS packages.
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
     && apt-get -y install --no-install-recommends sudo curl

RUN apt-get install -y build-essential pkg-config libudev-dev llvm libclang-dev protobuf-compiler libssl-dev
RUN apt-get install -y libssl-dev libudev-dev pkg-config zlib1g-dev llvm clang cmake make libprotobuf-dev protobuf-compiler

# 環境変数の設定
ENV DOKCER_OPERATION_USER=solanauser
ENV DOKCER_OPERATION_USERPASSWORD=solanauser
ENV UID=1001
# ユーザーを作成して切り替え
## dockerグループを作成
RUN groupadd docker
## dokcerグループに所属するユーザーを作成
RUN useradd -g docker -u ${UID} ${DOKCER_OPERATION_USER} -m
## sudo権限付与
RUN usermod -aG sudo ${DOKCER_OPERATION_USER}
RUN echo ${DOKCER_OPERATION_USER}:${DOKCER_OPERATION_USERPASSWORD} | chpasswd && \
    echo "${DOKCER_OPERATION_USER}   ALL=(ALL) NOPASSWD:ALL" >> /etc/sudoers

## ユーザーの切り替え
USER ${DOKCER_OPERATION_USER}

# shellをbashに変更
SHELL [ "/bin/bash", "-c" ]

# nvmをインストール
RUN curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.37.2/install.sh | bash

# lts版のnodeをインストール
RUN . $HOME/.nvm/nvm.sh && \
    nvm install --lts && \
    nvm use --lts && \
    node -v && npm -v && \
    npm i -g yarn

SHELL [ "/bin/sh", "-c" ]

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
RUN . "$HOME/.cargo/env" && \ 
    cargo install --git https://github.com/coral-xyz/anchor avm --locked --force && avm install latest && avm use latest
