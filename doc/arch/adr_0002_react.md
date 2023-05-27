# ADR 0002: Use React as our frontend framework

## Status

Accepted

## Context

We need a framework to build our application on top of. We have a few options:

- [Dioxus](https://dioxuslabs.com)
- Leptose
- ...[List](https://github.com/flosse/rust-web-framework-comparison)
- React

## Decision

As 2D rendering + drag'n'drop is not supported out of the box by any of the rust+wasm frameworks, we will use React as our frontend framework.
Other JS frameworks are not considered as they are not as popular as React.
