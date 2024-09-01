# rust-renshu-cho

[![GitHub Actions workflow badge][github-actions-badge]][github-actions-url]

[github-actions-badge]: https://github.com/nukopy/toy-tcpip/actions/workflows/ci.yml/badge.svg?branch=main
[github-actions-url]: https://github.com/nukopy/toy-tcpip/actions/workflows/ci.yml?query=branch:main

「Rust の練習帳」（Ken Youens-Clark 著、中山 光樹 訳、2024/01、O'REILLY Japan）の写経リポジトリ

## Links

- 書籍
  - [O'REILLY Japan: Rust の練習帳](https://www.oreilly.co.jp/books/9784814400584/)
- サンプルコード
  - [github.com/kyclark/command-line-rust](https://github.com/kyclark/command-line-rust)
- 正誤表（日本語版のサポートサイト）
  - [github.com/oreilly-japan/command-line-rust-ja](https://github.com/oreilly-japan/command-line-rust-ja)

## Environment

- OS: macOS Sonoma 14.4
- CPU: Apple M3 Max (arm64, 16 cores)
- Rust 1.80.1

```sh
$ rustup show
Default host: aarch64-apple-darwin
rustup home:  /Users/nukopy/.rustup

stable-aarch64-apple-darwin (default)
rustc 1.80.1 (3f5fd8dd4 2024-08-06)
```

## Commands

### Test

- Run all tests in all members in workspace

```sh
cargo test
```

- Run all tests in a specific member

```sh
cargo test -p <member>
```

## 進捗

### Rust の練習帳

https://www.oreilly.co.jp/books/9784814400584/

- [x] 1. イントロダクション
- [x] 2. echo コマンド
- [x] 3. cat コマンド
- [x] 4. head コマンド
- [ ] 5. wc コマンド
- [ ] 6. uniq コマンド
- [ ] 7. find コマンド
- [ ] 8. cut コマンド
- [ ] 9. grep コマンド
- [ ] 10. comm コマンド
- [ ] 11. tail コマンド
- [ ] 12. fortune コマンド
- [ ] 13. cal コマンド
- [ ] 14. ls コマンド
- [ ] 付録 A. clap の新しい API

### The Rust Programming Language

https://doc.rust-lang.org/book/title-page.html

- [ ] 1. Getting Started
- [ ] 2. Programming a Guessing Game
- [ ] 3. Common Programming Concepts
- [ ] 4. Understanding Ownership
- [ ] 5. Using Structs to Structure Related Data
- [ ] 6. Enums and Pattern Matching
- [ ] 7. Managing Growing Projects with Packages, Crates, and Modules
- [ ] 8. Common Collections
- [ ] 9. Error Handling
- [ ] 10. Generic Types, Traits, and Lifetimes
- [x] 11. [Writing Automated Tests](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [ ] 12. An I/O Project: Building a Command Line Program
- [ ] 13. Functional Language Features: Iterators and Closures
- [ ] 14. More about Cargo and Crates.io
- [ ] 15. Smart Pointers
- [ ] 16. Fearless Concurrency
- [ ] 17. Object Oriented Programming Features of Rust
- [ ] 18. Patterns and Matching
- [ ] 19. Advanced Features
- [ ] 20. Final Project: Building a Multithreaded Web Server
- [ ] 21. Appendix
