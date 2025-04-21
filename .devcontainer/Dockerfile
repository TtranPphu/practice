FROM ubuntu:22.04

RUN apt update && apt upgrade -y && apt install \
    sudo zsh wget curl git vim zip \
    python3-pip software-properties-common \
    bpytop tree ncdu cloc neofetch \
    build-essential cmake gdb \
    nasm clisp \
    -y

# user
ARG USER_NAME=practice
RUN groupadd ${USER_NAME}
RUN adduser --shell /usr/bin/zsh --gecos '' --ingroup ${USER_NAME} --disabled-password ${USER_NAME}
RUN usermod -aG sudo ${USER_NAME}
RUN echo '%sudo ALL=(ALL) NOPASSWD:ALL' >> /etc/sudoers
USER ${USER_NAME}

# set zsh as default shell
ENV SHELL=/usr/bin/zsh

# oh-my-zsh & powerlevel10k
RUN sh -c "$(wget https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh -O -)"
RUN git clone --depth=1 \
    https://github.com/romkatv/powerlevel10k.git \
    ${ZSH_CUSTOM:-$HOME/.oh-my-zsh/custom}/themes/powerlevel10k
COPY .zshrc /home/${USER_NAME}/.zshrc
COPY .p10k.zsh /home/${USER_NAME}/.p10k.zsh

# neo-vim and dependencies
ARG GITHUB_USER=TtranPphu
ARG GIT_BRANCH=TtranPphu-patch-1
RUN sudo add-apt-repository ppa:neovim-ppa/unstable -y
RUN sudo apt-get update && sudo apt-get install make gcc ripgrep unzip git xclip neovim fonts-noto-color-emoji -y
RUN git clone https://github.com/${GITHUB_USER}/kickstart.nvim \
    --branch ${GIT_BRANCH} "${XDG_CONFIG_HOME:-$HOME/.config}"/nvim

# rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    . $HOME/.cargo/env && \
    rustc --version

# uv for python
ARG PYTHON_VERSION=3.12
RUN curl --proto '=https' --tlsv1.2 -LsSf https://astral.sh/uv/install.sh | sh -s --
RUN . /home/${USER_NAME}/.local/bin/env && uv python install ${PYTHON_VERSION}

# mongodbcli
RUN sudo apt-get install -y gnupg
RUN wget -qO - https://www.mongodb.org/static/pgp/server-6.0.asc | sudo apt-key add -
RUN echo "deb [ arch=amd64,arm64 ] https://repo.mongodb.org/apt/ubuntu jammy/mongodb-org/6.0 multiverse" | sudo tee /etc/apt/sources.list.d/mongodb-org-6.0.list
RUN sudo apt-get update && sudo apt-get install -y mongodb-mongosh

