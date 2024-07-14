# Rust Rocket Backend Application

This is a sample backend application built using Rust and the Rocket framework. The application demonstrates how to set up and use Rocket for building web servers in Rust.

## Table of Contents

- [Installation](#installation)
- [Configuration](#configuration)
- [Usage](#usage)
- [Scripts](#scripts)
- [Project Structure](#project-structure)
- [Contributing](#contributing)
- [License](#license)

## Installation

1. **Clone the repository:**

    ```bash
    git clone https://github.com/trantu1243/social-media-backend.git
    cd social-media-backend
    ```

2. **Install Rust:**

    Make sure you have [Rust](https://www.rust-lang.org/) installed. You can install Rust using `rustup`:

    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

3. **Install dependencies:**

    The Rocket framework and other dependencies are specified in the `Cargo.toml` file. To install them, run:

    ```bash
    cargo build
    ```

## Configuration

1. **Copy the example environment file:**

    The application requires some environment variables to run. Copy the `.env.example` file to `.env`:

    ```bash
    cp .env.example .env
    ```

2. **Edit the `.env` file:**

    Open the `.env` file and update the values as needed. For example:

    ```env
    ROCKET_ENV=development
    DATABASE_URL=postgres://user:password@localhost/dbname
    SECRET_KEY=your_secret_key
    ```

## Usage

1. **Run the application:**

    To run the application in development mode, use:

    ```bash
    cargo run
    ```

    The server will start and listen on the port specified in the `Rocket.toml` file or the default port `8000`.

2. **Build the application:**

    To build the application for production, use:

    ```bash
    cargo build --release
    ```

3. **Run the tests:**

    To run the tests, use:

    ```bash
    cargo test
    ```

## Scripts

- **`cargo build`**: Compiles the Rust code.
- **`cargo run`**: Runs the application in development mode.
- **`cargo build --release`**: Compiles the Rust code for production.
- **`cargo test`**: Runs the tests.

## Project Structure

