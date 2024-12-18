# Contribution Guide

## Building from Source

As prerequisites, you will need [Rust], [Node], [pnpm], [Depot], and [tauri-cli] installed.

First, generate the Typescript bindings for the frontend:

```
cd rs/crates/repo-quest
cargo run -p repo-quest --bin export-bindings
```

You need to do this any time a `#[derive(Type)]` type is changed.

Next, in the same directory, run Tauri:

```
cargo tauri dev
```

That should be it!

## Codebase Structure

The codebase is a [Tauri] app. The backend is in the `rs/` directory, split primarily between `rs/crates/rq-core` (most of the functionality as a library) and `rs/crates/repo-quest` (the Tauri binary). 

The frontend is a standard single-page web-app written with [React], in the `js/` directory. The frontend app is built using [Vite], and we use [pnpm] to manage dependencies. All these tools are orchestrated by our own tool [Depot].

The backend and frontend talk to each other via Tauri commands and events. The type structure of commands/events is exported from Rust to Typescript via [tauri-specta].

# Application Architecture

RepoQuest does two things: (1) it orchestrates quests, and (2) it displays quest metadata to the user. (TODO: pick up from here explaining these things.)

[pnpm]: https://pnpm.io/
[Node]: https://nodejs.org/en
[Rust]: https://www.rust-lang.org/
[Depot]: https://github.com/cognitive-engineering-lab/depot
[tauri-cli]: https://crates.io/crates/tauri-cli
[Tauri]: https://v2.tauri.app/
[React]: https://react.dev/
[Vite]: https://vite.dev/
[tauri-specta]: https://github.com/specta-rs/tauri-specta/
