# Rust SQL Parser

A lightweight SQL parser written in Rust, supporting `SELECT` and `CREATE TABLE` statements with expression parsing, order by clauses, and error handling via a Pratt parser approach.

## About

While learning Rust and diving into tokenization, token handling, and parsing concepts, we had a final project focused on SQL parsing using Pratt parser techniques. This project is the result of that effort — a minimal, educational SQL parser implemented from scratch.

✅ Successfully completed and functional!

## Features

- Tokenizer for SQL input
- Pratt parser for expression handling
- Support for `SELECT` statements with:
  - `WHERE` clauses
  - `ORDER BY` (including expressions and `DESC`)
- Support for `CREATE TABLE` statements with:
  - Data types (`INT`, `VARCHAR`, etc.)
  - Constraints (WIP or extendable)
- Error handling for invalid tokens and malformed SQL
