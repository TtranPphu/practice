FROM ubuntu:22.04

RUN apt update && apt upgrade -y && apt install \
    sudo zsh wget curl git vim zip \
    gdb nasm build-essential cmake clisp python3-pip software-properties-common bpytop cloc -y

# powerlevel10k
ARG MESLO_URL="https://github.com/romkatv/powerlevel10k-media/raw/master/MesloLGS%20NF%20"
ARG MESLO_DIR="/usr/share/fonts/truetype/meslo"
RUN mkdir ${MESLO_DIR}
RUN wget ${MESLO_URL}Regular.ttf -O ${MESLO_DIR}/MesloLGSNFRegular.ttf
RUN wget ${MESLO_URL}Bold.ttf -O ${MESLO_DIR}/MesloLGSNFBold.ttf
RUN wget ${MESLO_URL}Italic.ttf -O ${MESLO_DIR}/MesloLGSNFItalic.ttf
RUN wget ${MESLO_URL}Bold%20Italic.ttf -O ${MESLO_DIR}/MesloLGSNFBoldItalic.ttf

# user
RUN groupadd practice
RUN adduser --shell /usr/bin/zsh --gecos '' --ingroup practice --disabled-password practice
RUN usermod -aG sudo practice
RUN echo '%sudo ALL=(ALL) NOPASSWD:ALL' >> /etc/sudoers
USER practice

# rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    . $HOME/.cargo/env && \
    rustc --version

# uv
RUN curl --proto '=https' --tlsv1.2 -LsSf https://astral.sh/uv/install.sh | sh -s -- && \
    . $HOME/.local/bin/env && \
    uv --version

RUN . $HOME/.local/bin/env && uv python install 3.11 3.12 3.13

# python requirements
COPY ./tools /tmp/tools
RUN pip install --no-warn-script-location -r /tmp/tools

# zsh
RUN sh -c "$(wget https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh -O -)"
RUN git clone --depth=1 \
    https://github.com/romkatv/powerlevel10k.git \
    ${ZSH_CUSTOM:-$HOME/.oh-my-zsh/custom}/themes/powerlevel10k
