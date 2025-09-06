#!/bin/bash

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
REPO_URL="https://github.com/saravenpi/skatos.git"
INSTALL_DIR="$HOME/.local/bin"
PROJECT_NAME="skatos"
BUILD_DIR="$HOME/.cache/skatos-build"

print_colored() {
    local color=$1
    local message=$2
    echo -e "${color}${message}${NC}"
}

print_success() {
    print_colored "$GREEN" "✓ $1"
}

print_error() {
    print_colored "$RED" "✗ $1"
}

print_warning() {
    print_colored "$YELLOW" "⚠ $1"
}

print_info() {
    print_colored "$BLUE" "ℹ $1"
}

check_dependencies() {
    print_info "Checking dependencies..."
    
    if ! command -v git &> /dev/null; then
        print_error "git is required but not installed. Please install git first."
        exit 1
    fi
    
    if ! command -v cargo &> /dev/null; then
        print_error "Rust/Cargo is required but not installed."
        print_info "Install Rust from: https://rustup.rs/"
        exit 1
    fi
    
    if ! command -v skate &> /dev/null; then
        print_warning "skate 🛹 is not installed. This tool requires skate to function."
        print_info "Install skate 🛹 from: https://github.com/charmbracelet/skate"
        print_info "On macOS: brew install charmbracelet/tap/skate"
        print_info "Continuing installation anyway..."
    fi
    
    print_success "Dependencies checked"
}

setup_directories() {
    print_info "Setting up directories..."
    
    # Create install directory
    mkdir -p "$INSTALL_DIR"
    
    # Create build directory
    mkdir -p "$BUILD_DIR"
    
    print_success "Directories created"
}

clone_or_update() {
    print_info "Getting source code..."
    
    if [ -d "$BUILD_DIR/.git" ]; then
        print_info "Repository exists, updating..."
        cd "$BUILD_DIR"
        git pull origin main
    else
        print_info "Cloning repository..."
        if [ -d "$BUILD_DIR" ]; then
            rm -rf "$BUILD_DIR"
        fi
        git clone "$REPO_URL" "$BUILD_DIR"
        cd "$BUILD_DIR"
    fi
    
    print_success "Source code ready"
}

build_project() {
    print_info "Building project (this may take a while on first build)..."
    
    cd "$BUILD_DIR"
    
    # Set up incremental compilation
    export CARGO_INCREMENTAL=1
    export CARGO_TARGET_DIR="$BUILD_DIR/target"
    
    # Build in release mode
    if cargo build --release; then
        print_success "Build completed successfully"
    else
        print_error "Build failed"
        exit 1
    fi
}

install_binary() {
    print_info "Installing binary..."
    
    if [ -f "$BUILD_DIR/target/release/$PROJECT_NAME" ]; then
        cp "$BUILD_DIR/target/release/$PROJECT_NAME" "$INSTALL_DIR/"
        chmod +x "$INSTALL_DIR/$PROJECT_NAME"
        print_success "Binary installed to $INSTALL_DIR/$PROJECT_NAME"
    else
        print_error "Binary not found at expected location"
        exit 1
    fi
}

check_path() {
    print_info "Checking PATH configuration..."
    
    if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
        print_warning "$INSTALL_DIR is not in your PATH"
        echo
        print_info "To use skatos from anywhere, add the following line to your shell profile:"
        echo
        print_colored "$YELLOW" "    echo 'export PATH=\"\$HOME/.local/bin:\$PATH\"' >> ~/.bashrc"
        echo
        print_info "For zsh users, use ~/.zshrc instead of ~/.bashrc"
        print_info "Then reload your shell or run: source ~/.bashrc"
        echo
        print_info "Alternative: You can run skatos directly with: $INSTALL_DIR/skatos"
    else
        print_success "$INSTALL_DIR is already in your PATH"
    fi
}

verify_installation() {
    print_info "Verifying installation..."
    
    if [ -x "$INSTALL_DIR/$PROJECT_NAME" ]; then
        print_success "Installation verified"
        echo
        print_info "Installation complete! 🎉"
        echo
        print_info "Try running: skatos --help"
        print_info "Or if not in PATH: $INSTALL_DIR/skatos --help"
    else
        print_error "Installation verification failed"
        exit 1
    fi
}

cleanup() {
    print_info "Cleaning up temporary files..."
    # We keep the build directory for incremental builds
    print_success "Cleanup complete (build cache preserved for future updates)"
}

main() {
    echo
    print_colored "$BLUE" "🚀 Installing Skatos 🛹 - Environment File Generator"
    echo
    
    check_dependencies
    setup_directories
    clone_or_update
    build_project
    install_binary
    check_path
    verify_installation
    cleanup
    
    echo
    print_success "Installation completed successfully!"
    echo
}

# Handle interruption
trap 'print_error "Installation interrupted"; exit 1' INT TERM

main "$@"