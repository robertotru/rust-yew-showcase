# Rust-Yew Web App Showcase

This repository demonstrates how modern web applications can be built using **Rust** and the **Yew framework**, showcasing the potential of Rust as both a backend and frontend language. The purpose of this project is to explore the **pros and cons** of using Rust for web development and to compare it with a widely adopted solution like **React** in JavaScript.

## Purpose
With the rise of WebAssembly and frameworks like Yew, Rust is becoming a viable option for developing both performance-critical backends and interactive frontends. This project aims to:
1. Highlight **how Rust can be used for modern web development**.
2. Discuss **advantages and trade-offs** of using Rust for frontend and backend tasks.
3. Provide a comparison point with React, a mature and widely adopted JavaScript library.

## What's Inside
This repository contains two implementations of a simple web application:
- **Rust with Yew**: A fully type-safe web application written in Rust, compiled to WebAssembly.
- **React with JavaScript**: A standard implementation of the same application using React.

### Features of Both Applications
- Header and Footer components
- A LikeButton component with interactive state management
- Simple structure to demonstrate core concepts of web development

---

## How to Run

### Rust (Frontend with Yew)
Required tools:
1. Install Rust: https://rustup.rs/
2. Install WebAssembly: `cargo install wasm-pack`
3. Be sure that `wasm32-unknown-unknown` is installed using `rustup target add wasm32-unknown-unknown`
4. Install Trunk: `cargo install trunk`

The following are specific of the project
1. Navigate to the `rust-yew` directory.
3. Run the app: `trunk serve`

### JavaScript (Frontend with React)
Required tools:
1. Install Node.js: https://nodejs.org/

The following are specific of the project
2. Navigate to the `react-app` directory.
3. Install dependencies: `npm install`
4. Start the app: `npm start`

---

## Pros and Cons of Rust for Web Development

### **Advantages**
1. **Type Safety**: Rust’s strict type system catches many errors at compile time, ensuring more reliable code.
2. **Performance**: Rust compiles to highly optimized WebAssembly binaries, offering near-native performance for web apps.
3. **Security**: Rust's memory safety guarantees help avoid common bugs like null pointer dereferencing and buffer overflows.
4. **Unified Language**: Using Rust for both frontend and backend eliminates the need to learn multiple languages.

### **Trade-offs**
1. **Developer Experience**: Yew and Rust’s tooling are less mature compared to JavaScript ecosystems.
2. **Build Size**: WebAssembly binaries tend to be larger than equivalent JavaScript bundles.
3. **Learning Curve**: Rust is more challenging to learn than JavaScript due to its strictness and advanced features.
4. **Ecosystem**: The Rust web development ecosystem is growing but still lacks the vast libraries and frameworks available for JavaScript.