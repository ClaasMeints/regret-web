# ADR 0001: Use Rust as the primary language

## Status

Accepted

## Context

We need to choose a language to build our application in. We have a few options:

- C++
- [Rust](https://www.rust-lang.org) (Backend)
- [Node.js](https://nodejs.org/en/)
- [Go](https://golang.org)
- ...

## Decision

We will use Rust as our primary language to build the Backend (engine + server) in.
Rust seems a good alternative to rewrite the original C++/Qt application.
Especially for the engine part, as it is a lot of low-level code.
