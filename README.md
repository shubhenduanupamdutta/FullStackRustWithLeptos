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
