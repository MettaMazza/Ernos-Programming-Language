#!/bin/bash

# ErnosPlain Installer Script
# Installs the ErnosPlain compiler globally on macOS ARM64 systems.

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

if [ "$OS" != "Darwin" ]; then
    echo -e "${RED}Error: ErnosPlain currently only supports macOS.${NC}"
    exit 1
fi

if [ "$ARCH" != "arm64" ]; then
    echo -e "${YELLOW}Warning: ErnosPlain compiles natively to ARM64 assembly (Apple Silicon).${NC}"
    echo -e "${YELLOW}Non-ARM64 systems are not officially supported.${NC}"
    exit 1
fi
echo -e "${GREEN}✓ Compatible macOS ARM64 system detected.${NC}"
echo ""

# 2. Dependency Check (Clang and Cargo)
echo -e "${BOLD}2. Verifying build dependencies...${NC}"
if ! command -v clang &> /dev/null; then
    echo -e "${RED}Error: 'clang' was not found on your system.${NC}"
    echo -e "${RED}Please install Xcode Command Line Tools by running: xcode-select --install${NC}"
    exit 1
fi
echo -e "${GREEN}✓ Clang assembler is installed.${NC}"

if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: Rust/Cargo is required to build the bootstrap compiler driver.${NC}"
    echo -e "${RED}Please install Rust from https://rustup.rs/ before running this installer.${NC}"
    exit 1
fi
echo -e "${GREEN}✓ Rust/Cargo build tools are installed.${NC}"
echo ""

# 3. Compilation Phase
echo -e "${BOLD}3. Compiling ErnosPlain from source...${NC}"

# A. Build the Rust bootstrap compiler
echo "Building the Rust bootstrap compiler driver..."
cargo build --release --quiet
cp target/release/ernosplain ./epc_bootstrap

# B. Concatenate and build the self-hosted compiler
echo "Generating the self-hosted compiler unit..."
cat ep_lexer.ep ep_parser.ep ep_codegen.ep epc.ep > self_hosted_compiler.ep

echo "Compiling self-hosted compiler with the bootstrap compiler..."
./epc_bootstrap self_hosted_compiler.ep

# C. Verify self-replication
echo "Replicating compiler to second-generation binary..."
cp ./self_hosted_compiler ./self_hosted_compiler_gen1
./self_hosted_compiler_gen1 self_hosted_compiler.ep

# D. Clean up intermediate build products
rm -f ./epc_bootstrap ./self_hosted_compiler_gen1 ./self_hosted_compiler.ep

echo -e "${GREEN}✓ Compilation and self-replication successful!${NC}"
echo ""

# 4. Installation Phase
echo -e "${BOLD}4. Installing binary...${NC}"
INSTALL_DIR="$HOME/.local/bin"
mkdir -p "$INSTALL_DIR"

mv ./self_hosted_compiler "$INSTALL_DIR/epc"
echo -e "${GREEN}✓ Installed 'epc' binary to $INSTALL_DIR/epc${NC}"
echo ""

# 5. PATH Verification and Guide
echo -e "${BOLD}5. Verification & PATH setup...${NC}"
if [[ ":$PATH:" == *":$INSTALL_DIR:"* ]]; then
    echo -e "${GREEN}✓ $INSTALL_DIR is already in your shell's PATH variable.${NC}"
    echo ""
    echo -e "${BOLD}${GREEN}Installation Complete! 🎉${NC}"
    echo -e "You can now compile ErnosPlain files globally by typing: ${BOLD}epc <file.ep>${NC}"
else
    echo -e "${YELLOW}Almost done! You need to add $INSTALL_DIR to your shell's PATH.${NC}"
    echo -e "Run the following command to add it to your shell configuration (e.g., .zshrc):"
    echo ""
    echo -e "  ${BOLD}echo 'export PATH=\"\$HOME/.local/bin:\$PATH\"' >> ~/.zshrc${NC}"
    echo ""
    echo -e "Then reload your shell: ${BOLD}source ~/.zshrc${NC}"
    echo -e "After doing this, you can compile globally by typing: ${BOLD}epc <file.ep>${NC}"
fi

echo -e "${BOLD}${BLUE}==============================================${NC}"
