# UNIX V6 デバイスのパーサー

![select](https://github.com/rsk-ymst/v6sh/assets/81950820/c444d568-6531-480d-8c5b-cf308c5d4451)

### v6sh
- main.rs        : エントリポイント
- v6sh/mod.rs    : 関連モジュールを統括するrootモジュール
- v6sh/inode.rs  : inodeに関連する構造体及び関数が格納されている
- v6sh/parser.rs : ストレージデバイス解析に関連する構造体及び関数が格納されている

### 開発環境
- rustc 1.68.1 (8460ca823 2023-03-20)
- cargo 1.68.1 (115f34552 2023-02-26)

### ビルド＆実行
```
cargo run --release
```
