# SQLite Concurrent Insertions with Rust

This project demonstrates how to use SQLite for concurrent insertions and queries using **Rust** and **`rusqlite`**. It uses **Tokio** for async task handling and demonstrates managing connections and interacting with a SQLite database.

## Features

- **SQLite in-memory database** for quick, lightweight operations.
- **Concurrent insertions** using **`tokio`** and async tasks.
- Demonstrates the use of **`jsonb`** for storing JSON data in a SQLite database.
- Ability to **query the database** and retrieve data after performing inserts.
- Designed for learning how to manage **SQLite connections** with concurrency in Rust.

## Prerequisites

To run this project, you will need:

- **Rust** (1.56.0 or later)
- **SQLite** (SQLite version 3.x)

You can install Rust from [rust-lang.org](https://www.rust-lang.org/).
