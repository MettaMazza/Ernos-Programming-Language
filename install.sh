#!/bin/bash

# ErnosPlain Installer Script
# Installs the ErnosPlain compiler globally on macOS and Linux systems.

set -e

# Terminal formatting colors
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
BOLD='\033[1m'
NC='\033[0m' # No Color

echo -e "${BOLD}${BLUE}==============================================${NC}"
echo -e "${BOLD}${BLUE}          ErnosPlain Installer Tool           ${NC}"
echo -e "${BOLD}${BLUE}==============================================${NC}"
echo ""

# 1. System Compatibility Check
echo -e "${BOLD}1. Checking system compatibility...${NC}"
OS=$(uname -s)
ARCH=$(uname -m)

if [ "$OS" != "Darwin" ] && [ "$OS" != "Linux" ]; then
    echo -e "${RED}Error: ErnosPlain supports macOS and Linux. Detected OS: $OS${NC}"
    exit 1
fi

if [ "$ARCH" != "arm64" ] && [ "$ARCH" != "aarch64" ] && [ "$ARCH" != "x86_64" ]; then
    echo -e "${YELLOW}Warning: Unsupported architecture: $ARCH${NC}"
    echo -e "${YELLOW}Supported: arm64, aarch64, x86_64${NC}"
    exit 1
fi
echo -e "${GREEN}✓ Compatible $OS $ARCH system detected.${NC}"
echo ""

# 2. Dependency Check (C compiler and Cargo)
echo -e "${BOLD}2. Verifying build dependencies...${NC}"

if [ "$OS" = "Linux" ]; then
    if command -v clang &> /dev/null; then
        echo -e "${GREEN}✓ Clang C compiler is installed.${NC}"
    elif command -v gcc &> /dev/null; then
        echo -e "${GREEN}✓ GCC C compiler is installed.${NC}"
    else
        echo -e "${RED}Error: A C compiler (gcc or clang) is required.${NC}"
        echo -e "${RED}Install with: sudo apt install build-essential  (Debian/Ubuntu)${NC}"
        echo -e "${RED}         or: sudo dnf install gcc               (Fedora)${NC}"
        exit 1
    fi
elif [ "$OS" = "Darwin" ]; then
    if ! command -v clang &> /dev/null; then
        echo -e "${RED}Error: 'clang' was not found on your system.${NC}"
        echo -e "${RED}Please install Xcode Command Line Tools by running: xcode-select --install${NC}"
        exit 1
    fi
    echo -e "${GREEN}✓ Clang C compiler is installed.${NC}"
fi

if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: Rust/Cargo is required to build the reference compiler.${NC}"
    echo -e "${RED}Please install Rust from https://rustup.rs/ before running this installer.${NC}"
    exit 1
fi
echo -e "${GREEN}✓ Rust/Cargo build tools are installed.${NC}"
echo ""

# 3. Compilation Phase
echo -e "${BOLD}3. Compiling ErnosPlain from source...${NC}"

# A. Build the Rust reference compiler
echo "Building the Rust bootstrap compiler driver..."
cargo build --release --quiet

# B. Build the modular self-hosted compiler. The reference driver resolves all
# imports in epc.ep, including checker, optimizer, and generated runtime modules.
echo "Compiling the modular self-hosted compiler..."
./target/release/ernos epc.ep

# C. Verify a stable three-generation self-replication fixpoint.
echo "Replicating compiler to second generation..."
cp ./epc ./epc_gen1
./epc_gen1 epc.ep

echo "Replicating compiler to third generation..."
cp ./epc ./epc_gen2
./epc_gen2 epc.ep
if ! cmp -s ./epc_gen2 ./epc; then
    echo -e "${RED}Error: self-hosted compiler did not reach a gen2/gen3 fixpoint.${NC}"
    exit 1
fi

# D. Clean up intermediate build products
rm -f ./epc_gen1 ./epc_gen2

echo -e "${GREEN}✓ Compilation and self-replication successful!${NC}"
echo ""

# 4. Installation Phase
echo -e "${BOLD}4. Installing binaries...${NC}"
INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"

mv ./epc "$INSTALL_DIR/epc"
echo -e "${GREEN}✓ Installed 'epc' (self-hosted compiler) to $INSTALL_DIR/epc${NC}"

# Install the feature-complete driver too. `ernos` provides check/transpile/bind/
# repl/--release/--native and resolves stdlib imports relative to its own dir.
cp target/release/ernos "$INSTALL_DIR/ernos"
echo -e "${GREEN}✓ Installed 'ernos' (full CLI) to $INSTALL_DIR/ernos${NC}"

# Install the standard library next to the binaries so `import "string"` etc.
# resolve from any directory (the resolver checks <exe_dir>/stdlib).
rm -rf "$INSTALL_DIR/stdlib"
cp -R stdlib "$INSTALL_DIR/stdlib"
echo -e "${GREEN}✓ Installed standard library to $INSTALL_DIR/stdlib${NC}"

# Verify the installed binaries themselves, not only their build-tree copies.
"$INSTALL_DIR/epc" check tests/test_native_basic.ep >/dev/null
"$INSTALL_DIR/ernos" check tests/test_native_basic.ep >/dev/null
echo -e "${GREEN}✓ Installed compilers passed static validation smoke tests.${NC}"
echo ""

# 5. PATH Verification and Guide
echo -e "${BOLD}5. Verification & PATH setup...${NC}"

# Detect the appropriate shell config file
if [ "$OS" = "Linux" ]; then
    if [ -n "$BASH_VERSION" ] || [ "$(basename "$SHELL")" = "bash" ]; then
        SHELL_RC="$HOME/.bashrc"
    elif [ "$(basename "$SHELL")" = "zsh" ]; then
        SHELL_RC="$HOME/.zshrc"
    else
        SHELL_RC="$HOME/.profile"
    fi
else
    SHELL_RC="$HOME/.zshrc"
fi

if [[ ":$PATH:" == *":$INSTALL_DIR:"* ]]; then
    echo -e "${GREEN}✓ $INSTALL_DIR is already in your shell's PATH variable.${NC}"
    echo ""
    echo -e "${BOLD}${GREEN}Installation Complete! 🎉${NC}"
    echo -e "You can now compile ErnosPlain files globally by typing: ${BOLD}ernos <file.ep>${NC} (or ${BOLD}epc <file.ep>${NC})"
else
    echo -e "${YELLOW}Almost done! You need to add $INSTALL_DIR to your shell's PATH.${NC}"
    echo -e "Run the following command to add it to your shell configuration ($(basename "$SHELL_RC")):"
    echo ""
    echo -e "  ${BOLD}echo 'export PATH=\"\$HOME/.local/bin:\$PATH\"' >> $SHELL_RC${NC}"
    echo ""
    echo -e "Then reload your shell: ${BOLD}source $SHELL_RC${NC}"
    echo -e "After doing this, you can compile globally by typing: ${BOLD}ernos <file.ep>${NC} (or ${BOLD}epc <file.ep>${NC})"
fi

echo -e "${BOLD}${BLUE}==============================================${NC}"
