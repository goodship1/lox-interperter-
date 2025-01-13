# Lox Interpreter in Rust

## Overview

This repository contains an implementation of the Lox programming language, as described in Robert Nystrom's book *Crafting Interpreters*. The interpreter is written in Rust, leveraging its performance and memory safety features to create a fast and robust implementation.

Lox is a dynamically-typed, interpreted language with support for object-oriented programming. This implementation follows the tree-walk interpreter approach, providing a foundational understanding of interpreters while delivering a functional and performant tool.

---

## Features

- **Complete Lox Language Implementation**: Supports all core features of the Lox language, including:
  - Variables and scope
  - Control flow (if statements, loops)
  - Functions and closures
  - Classes and inheritance
  - Dynamic typing
- **Error Handling**: Provides detailed error messages for both syntax and runtime errors.
- **Interactive Mode**: Includes a REPL (Read-Eval-Print Loop) for experimenting with Lox code interactively.
- **File Execution**: Execute Lox scripts from a file.
- **Optimized Parsing**: Implements efficient parsing and evaluation strategies, making it suitable for real-world use.

---

## Prerequisites

Before building and running the project, ensure you have the following installed:

- **Rust** (1.70 or later): Install Rust from [rust-lang.org](https://www.rust-lang.org/).

---

## Getting Started

1. **Clone the Repository**:

   ```bash
   git clone https://github.com/yourusername/lox-interpreter-rust.git
   cd lox-interpreter-rust
