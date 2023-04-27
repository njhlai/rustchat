# rustchat

## About
rustchat is a toy chat app with a Rust backend, as an exercise to understand how Rust works. Very heavily based on [this article](https://outcrawl.com/rust-react-realtime-chat).

rustchat currently comes with a simple [Next.js](https://nextjs.org/) [React](https://react.dev/) frontend.

## Requirements
- [Rust](https://www.rust-lang.org/). Recommended to use Rust through [rustup](https://rustup.rs/).
- [Node.js](https://nodejs.org/): ver >= 16.X.
- [npm](https://www.npmjs.com/)

## Usage
To start the server, run:
```sh
cd core && cargo run
```
To start the Next.js React frontend, first install dependencies:
```sh
cd react && npm install
```
Then, run:
```sh
npm run start
```
Use any browser and navigate to <https://localhost:3000/>.
