cargo install --list

if command -v cargo &>/dev/null; then
   echo "cargo is already installed"
else
   curl -o- https://sh.rustup.rs -sSf | sh
   echo "cargo has been installed"
fi

if command -v geckodriver &>/dev/null; then
   echo "geckodriver is already installed"
else
   cargo install geckodriver
   echo "geckodriver has been installed"
fi

if cargo install --list | grep -c "cargo-make v" &>/dev/null; then
   echo "cargo-make is already installed"
else
   cargo install cargo-make
   echo "cargo-make has been installed"
fi

if command -v wasm-pack &>/dev/null; then
   echo "wasm-pack is already installed"
else
   cargo install wasm-pack
   echo "wasm-pack has been installed"
fi

if cargo install --list | grep -c "cargo-generate v" &>/dev/null; then
   echo "cargo-generate is already installed"
else
   cargo install cargo-generate
   echo "cargo-generate has been installed"
fi

if command -v diesel &>/dev/null; then
   echo "diesel is already installed"
else
   cargo install diesel_cli --no-default-features --features postgres
   echo "diesel has been installed"
fi

if command -v nvm &>/dev/null; then
   echo "nvm is already installed"
else
   curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.5/install.sh | bash
   export NVM_DIR="$HOME/.nvm"
   [ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"  # This loads nvm
   [ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"  # This loads nvm bash_completion
   echo "nvm has been installed"
fi

if command -v node &>/dev/null; then
   echo "node is already installed"
   node --version
   nvm install node
else
   nvm install node
   echo "node has been installed"
fi

if command -v npm &>/dev/null; then
   echo "npm is already installed"
   npm --version
else
   echo "npm has been installed"
   nvm install npm
fi

if npm list -g --depth=0 | grep -c "chromedriver"  &>/dev/null; then
   echo "chromedriver is already installed"
else
   npm install -g chromedriver
   echo "chromedriver has been installed"
fi

# cargo make ci-flow
# cargo make dockerci
