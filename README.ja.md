# actix-identity-sample について

Actix Identityを使って、Web上でIDとパスワードによるユーザ認証を行うサンプルプログラムです。

# 動作確認環境

* OS
    * Ubuntu 20.04.6 LTS
* Rust(rustc, cargo)のバージョン
    * 1.79.0

# Webサーバの起動

以下のコマンドを実行すると、8080番のポートで受け付けるWebサーバが起動します。

```
cargo run --release
```

# Webサイトの各パスについて

| パス | 認証要否 | 内容 |
| --- | --- | --- |
| ``/`` | 不要 | 誰でもアクセスできるページ |
| ``/login`` | 不要 | ログイン用ページ。認証成功したら``/welcome``にリダイレクトする |
| ``/logout`` | 必要 | ログアウト用ページ。``/``にリダイレクトする |
| ``/welcome`` | 必要 | 認証後にユーザIDを表示するページ |
| ``/secret`` | 必要 | 認証ユーザしかアクセスできないページ |

リクエストがユーザ認証済かの判定は、[AuthMiddleware](src/middleware.rs)で行います。

# Author
 
showchan33

# License
"actix-identity-sample" is under [GPL license](https://www.gnu.org/licenses/licenses.en.html).
