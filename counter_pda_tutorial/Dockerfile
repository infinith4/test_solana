# FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye
FROM mcr.microsoft.com/devcontainers/rust:1-1-bullseye
# [Optional] Uncomment this section to install additional OS packages.
# RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
#     && apt-get -y install --no-install-recommends <your-package-list-here>

# [Optional] Uncomment if you want to install an additional version of node using nvm
# ARG EXTRA_NODE_VERSION=10
# RUN su node -c "source /usr/local/share/nvm/nvm.sh && nvm install ${EXTRA_NODE_VERSION}"

# [Optional] Uncomment if you want to install more global node modules
# RUN su node -c "npm install -g npm@10.8.0"

RUN apt-get update && apt-get upgrade
RUN apt-get install -y sudo pkg-config build-essential libudev-dev libssl-dev
# RUN cargo install --git https://github.com/coral-xyz/anchor --tag v0.29.0 anchor-cli --locked
# RUN ["/bin/bash", "-c", "$(curl -sSfL https://release.solana.com/v1.18.18/install)"]
# CMD ["avm", "install", "latest"]
# CMD ["avm", "use", "latest"]