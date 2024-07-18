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
ENV DOKCER_OPERATION_GROUP=solanauser
# ユーザーを作成して切り替え
## dockerグループを作成
RUN groupadd ${DOKCER_OPERATION_GROUP}
## dockerグループに所属するユーザーを作成
RUN useradd -g ${DOKCER_OPERATION_GROUP} -u ${UID} ${DOKCER_OPERATION_USER} -m
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

# USER root
# RUN apt-get update && apt-get install -y \
#     git \
#     zsh \
#     sed \
#     && apt-get clean \
#     && rm -rf /var/lib/apt/lists/*

# ENV SHELL /usr/bin/zsh
# RUN zsh
# #set default shell to zsh
# RUN chsh -s /bin/zsh

# USER ${DOKCER_OPERATION_USER}

# #zprezto install
# RUN git clone --recursive \
#     https://github.com/sorin-ionescu/prezto.git \
#     $HOME/.zprezto
# SHELL ["/bin/zsh", "-c"]
# RUN setopt EXTENDED_GLOB; \
#     for rcfile in "${ZDOTDIR:-$HOME}"/.zprezto/runcoms/^README.md(.N); do \
#       ln -s "$rcfile" "${ZDOTDIR:-$HOME}/.${rcfile:t}"; \
#     done
# RUN git clone --recurse https://github.com/belak/prezto-contrib $HOME/.zprezto/contrib
# RUN sed -i "/'prompt'/c \\\ 'contrib-prompt' \\\\\n  'prompt'" /root/.zpreztorc
# RUN sed -i "s/theme 'sorin'/theme 'spaceship'/g" /root/.zpreztorc

# CMD ["/bin/zsh"]


USER root

RUN echo "if [ -t 1 ]; then" >> /root/.bashrc
RUN echo "exec zsh" >> /root/.bashrc
RUN echo "fi" >> /root/.bashrc

USER ${DOKCER_OPERATION_USER}

RUN echo "if [ -t 1 ]; then" >> $HOME/.bashrc
RUN echo "exec zsh" >> $HOME/.bashrc
RUN echo "fi" >> $HOME/.bashrc

ADD .zshrc /home/solanauser/.zshrc