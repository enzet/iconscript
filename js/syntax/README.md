# Iconscript Syntax Highlighting

This directory contains syntax highlighting files for iconscript files.

## Vim

### Installation

1. Copy `iconscript.vim` to your Vim syntax directory (e.g. `~/.vim/syntax` or
   `~/.config/nvim/syntax`).

2. Add filetype detection to your `~/.vimrc`:
   ```vim
   au BufRead,BufNewFile *.iconscript set filetype=iconscript
   ```

## VS Code

1. Copy `iconscript-vscode` to your VS Code extensions folder:
   ```bash
   cp -r syntax/iconscript-vscode ~/.vscode/extensions/iconscript.iconscript-0.1.0
   ```
