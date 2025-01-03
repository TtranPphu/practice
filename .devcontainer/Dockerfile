FROM ubuntu:22.04

RUN apt update && apt upgrade -y && apt install sudo zsh git vim zip build-essential gdb cmake python3-pip wget curl software-properties-common bpytop -y
RUN add-apt-repository ppa:deadsnakes/ppa -y
RUN apt update && apt upgrade -y && DEBIAN_FRONTEND=noninteractive apt install python3.11 python3.12 python3.13 -y

# powerlevel10k
RUN mkdir /usr/share/fonts/truetype/meslo
RUN wget https://github.com/romkatv/powerlevel10k-media/raw/master/MesloLGS%20NF%20Regular.ttf \
    -O /usr/share/fonts/truetype/meslo/MesloLGSNFRegular.ttf
RUN wget https://github.com/romkatv/powerlevel10k-media/raw/master/MesloLGS%20NF%20Bold.ttf \
    -O /usr/share/fonts/truetype/meslo/MesloLGSNF20Bold.ttf
RUN wget https://github.com/romkatv/powerlevel10k-media/raw/master/MesloLGS%20NF%20Italic.ttf \
    -O /usr/share/fonts/truetype/meslo/MesloLGSNFItalic.ttf
RUN wget https://github.com/romkatv/powerlevel10k-media/raw/master/MesloLGS%20NF%20Bold%20Italic.ttf \
    -O /usr/share/fonts/truetype/meslo/MesloLGSNFBoldItalic.ttf

# user
RUN adduser --shell /usr/bin/zsh --gecos '' --ingroup sudo --disabled-password practice
RUN echo '%sudo ALL=(ALL) NOPASSWD:ALL' >> /etc/sudoers
USER practice

# rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# uv
RUN curl --proto '=https' --tlsv1.2 -LsSf https://astral.sh/uv/install.sh | sh -s --

# zsh
RUN sh -c "$(wget https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh -O -)"
RUN git clone --depth=1 https://github.com/romkatv/powerlevel10k.git ${ZSH_CUSTOM:-$HOME/.oh-my-zsh/custom}/themes/powerlevel10k

COPY .zshrc /home/practice
COPY .p10k.zsh /home/practice