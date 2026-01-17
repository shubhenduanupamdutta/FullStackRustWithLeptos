# Code Along for the LinkedIn Course - Full-Stack Web Application with Rust and Leptos

---

## WebAssembly

---

In 2017, WebAssembly (Wasm) was introduced as a new binary instruction format for a stack-based virtual machine. It is designed to be a portable compilation target for programming languages, enabling high-performance applications on web pages. WebAssembly allows developers to run code written in multiple languages at near-native speed by taking advantage of common hardware capabilities. It started being supported by all major web browsers, including Chrome, Firefox, Safari, and Edge.

- Aims to provide near-native performance for web applications while still being as portable and secure as JavaScript.
- Provides and instruction set which closely mimics the instruction set a native CPU might have.
- Memory management strategy determined by the programming language uses, not the runtime.
- Supported as a compilation target by most major languages including Go, Rust, C++, and others.

Ultimately, WebAssembly opens the doors for programmers to build browser based application in any programming language that can be compiled to WebAssembly. And those applications can run at near-native speeds.

---

## Why Full-Stack with Rust?

---

- **Front end** - All the benefits of WebAssembly including performance, security, and portability.
- **Front end and Back End** - Memory and type safety guarantees of Rust.
- **Back end** - Rust back-end frameworks are some of the highest performing frameworks available.
- **Language Isomorphism (One Programming Language Across the Entire Stack)**
  - Front-end and Back-end developers become more interchangeable.
  - Facilitates sharing and moving of code throughout the stack.
  - Enables isomorphic functions!

---

## Full-Stack Frameworks in Rust

---

- **Yew**
- **Dioxus**
- **Sycamore**
- **Perseus**
- **Leptos** ← (Chosen for this course)

---

## Why Leptos?

---

- Full stack in one project.
- Isomorphic functions.
- Modeled after SolidJS - a popular JavaScript framework.
- Allows for Actix-Web and Axum backends.
- Fantastic developer experience and tooling.

---

## Some necessary tools

---

### Cargo-Leptos

- Convenience utility for building and running Leptos applications.
- Features hot reloading, among other things.
- Install with the following command:

  ```bash
  cargo install cargo-leptos
  ```

### New rust target - wasm32-unknown-unknown

- `rustup target add wasm32-unknown-unknown`
- Features optimizations for WebAssembly.
- Why `unknown-unknown`?: Typically for rust-build targets an operating system and CPU architecture are specified. But in this case, target are browsers which are not specific to any operating system or CPU architecture.
  - First `unknown` indicates no specific operating system.
  - Second `unknown` indicates no specific CPU architecture.
- This target is necessary for compiling Rust code to WebAssembly.

---

## Creating a New Leptos Project

---

- **Don't use `cargo new` or `cargo init`! Even a barebones Leptos setup requires a few dependencies and features configured.**
- **Use the Leptos starter template - it comes pre-configured with everything you need to get started.**

  ```bash
  cargo leptos new --git leptos-rs/start
  ```

- Then you can use `cargo leptos watch` to run the development server with hot reloading.

---

## Rust Project Structure

---

### Types of Crates

- Binary Crate - An executable program. In Leptos projects, the server code is typically organized as a binary crate.
- Library Crate - A reusable library. In Leptos projects, the shared code and

### A Rust package (project directory) in general can contain

- At most one library crate.
- Any number of binary crates.

---

## Leptos Project Structure

---

### One Library - `lib.rs`

- Represents front-end logic
- Compiled to WebAssembly

### One binary - `main.rs`

- Represents back-end logic
- Compiled to bare metal
- Hosts APIs and static files

Both crates are inside the package represented by the project directory, and the `Cargo.toml` file at the root of the project directory configures both crates.

In `Cargo.toml` in the section `package.metadata.leptos`, where fields specific to leptos are configured. For now, let's focus on two fields: `bin-features` and `lib-features`.

- `bin-features` - Features to enable for the binary crate. Here, we enable the `ssr` feature which adds support for server-side rendering. Any features mentioned here will be available for backend logic to use.
- `lib-features` - Features to enable for the library crate. Here, we enable the `hydrate` feature which adds support for hydrating server-rendered HTML on the client side. Any features mentioned here will be available for front-end logic to use.

---

## SSR Feature

---

- Enabled by bin crate
- Dependencies don't need to compile to WASM
- Not part of the binary sent to the user's browser
- Backend logic will access to dependencies mentioned in SSR feature

---

## Hydrate Feature

---

- Enabled by lib crate
- Dependencies must compile to WASM and will be sent to the user's browser
  - This might limit potential dependencies
  - More dependencies means larger client bundle size

---

## Dependency Strategies

---

### Front-end specific dependencies

- Set optional to true
- Add to `hydrate` feature, using `dep:<dependency_name>`

### Back-end specific dependencies

- Set optional to true
- Add to `ssr` feature, using `dep:<dependency_name>`

---

## Tailwind CSS

---

### Provides predefined utility classes for rapid UI development

- Spend less time creating your own
- Well-loved by front-end developers

### Compile a CSS file based on

- The classes used in your rust code
- The content of an input file (this is typically a CSS file with Tailwind directives)
