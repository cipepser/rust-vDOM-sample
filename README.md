# rust-vDOM-sample

仮想DOMって触ったことないなぁと思ったので、
[Rust で仮想DOMを実装する‐１ \- Qiita](https://qiita.com/ne_no_usa/items/29ae8e5bcccfec41a626)を写経してみる。


## 準備

```bash
$ cargo install wasm-pack
$ cargo new --lib vdom
$ npm init
$ npm install -D @wasm-tool/wasm-pack-plugin html-webpack-plugin webpack$ k-cli webpack-dev-server
```

### 他にやったこと

- crate名は`vdom`とした
- `webpack.config.js`を作成
- `index.html`を作成
- `index.js`を作成
- `package.json`の`scripts`に`"start": "./node_modules/.bin/webpack-dev-server"`を追加

### 準備が終わった段階でのディレクトリ構成

```bash
❯ tree -I node_modules
.
├── Cargo.lock
├── Cargo.toml
├── index.html
├── index.js
├── package-lock.json
├── package.json
└── src
    └── lib.rs
```


## 実装

## References
- [Rust で仮想DOMを実装する‐１ \- Qiita](https://qiita.com/ne_no_usa/items/29ae8e5bcccfec41a626)
- [Rust で wasm\-pack するときの準備 \- Qiita](https://qiita.com/ne_no_usa/items/c5552f20d4839fb5b728)
