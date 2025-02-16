## update and install some things we should probably have
apt-get update --assume-yes
apt-get upgrade --assume-yes
apt-get install --assume-yes \
  curl \
  git \
  gnupg2 \
  jq \
  sudo \
  zsh \
  vim \
  build-essential \
  openssl \
  libssl-dev \
  pkg-config \
  libpq-dev \
  # Node.js \
  npm

## Install rustup and common components
curl https://sh.rustup.rs -sSf | sh -s -- -y 
rustup install stable
## rustup install nightly
rustup component add rustfmt
## rustup component add rustfmt --toolchain nightly
rustup component add rustfmt --toolchain stable
rustup component add clippy 
rustup component add clippy --toolchain stable
## rustup component add clippy --toolchain nightly

source "$HOME/.cargo/env"
cargo install cargo-expand
cargo install cargo-edit
cargo install cargo-make
cargo install wasm-pack
cargo install cargo-generate
cargo install geckodriver
cargo install --list

## setup and install oh-my-zsh
sh -c "$(curl -fsSL https://raw.githubusercontent.com/robbyrussell/oh-my-zsh/master/tools/install.sh)"
cp -R /root/.oh-my-zsh /home/$USERNAME
cp /root/.zshrc /home/$USERNAME
sed -i -e "s/\/root\/.oh-my-zsh/\/home\/$USERNAME\/.oh-my-zsh/g" /home/$USERNAME/.zshrc
chown -R $USER_UID:$USER_GID /home/$USERNAME/.oh-my-zsh /home/$USERNAME/.zshrc
