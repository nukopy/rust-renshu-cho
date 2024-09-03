# rust-renshu-cho

[![MIT license badge][mit-badge]][mit-url]
[![GitHub Actions workflow badge][github-actions-badge]][github-actions-url]

[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/nukopy/rust-renshu-cho/blob/main/LICENSE
[github-actions-badge]: https://github.com/nukopy/toy-tcpip/actions/workflows/ci.yml/badge.svg?branch=main
[github-actions-url]: https://github.com/nukopy/toy-tcpip/actions/workflows/ci.yml?query=branch:main

「Rust の練習帳」（Ken Youens-Clark 著、中山 光樹 訳、2024/01、O'REILLY Japan）の写経リポジトリ

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

## Progress

- [x] 1. イントロダクション
- [x] 2. echo コマンド
- [x] 3. cat コマンド
- [x] 4. head コマンド
- [x] 5. wc コマンド
- [x] 6. uniq コマンド
- [ ] 7. find コマンド
- [ ] 8. cut コマンド
- [ ] 9. grep コマンド
- [ ] 10. comm コマンド
- [ ] 11. tail コマンド
- [ ] 12. fortune コマンド
- [ ] 13. cal コマンド
- [ ] 14. ls コマンド
- [ ] 付録 A. clap の新しい API

## Links

- 書籍情報
  - 書籍リンク
    - [O'REILLY Japan: Rust の練習帳](https://www.oreilly.co.jp/books/9784814400584/)
  - サンプルコード
    - [github.com/kyclark/command-line-rust](https://github.com/kyclark/command-line-rust)
  - 正誤表（日本語版のサポートサイト）
    - [github.com/oreilly-japan/command-line-rust-ja](https://github.com/oreilly-japan/command-line-rust-ja)
- References
  - [The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html)
    - 11. [Writing Automated Tests](https://doc.rust-lang.org/book/ch11-00-testing.html)

## License

This project is licensed under the [MIT license](https://github.com/nukopy/rust-renshu-cho/blob/main/LICENSE).
