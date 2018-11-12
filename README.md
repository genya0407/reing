# Reing

Reingは [Peing - 質問箱 -](https://peing.net) のクローンアプリケーションです。

匿名で質問を受け付けることができ、回答をTwitterに流すことができます。

## How to develop

ローカル環境での動かし方について述べます。

### Rustの環境を作る

[rustup](https://rustup.rs/) などを使ってRustの環境を作ります。

開発に使われているコンパイラのバージョンは `rustc 1.30.0-nightly (f49f6e73a 2018-09-23)` です。

### PostgreSQLサーバーを立てる

何らかの手段でPostgreSQLサーバーを立てます。

オススメは[scripts/launch_db.sh](/scripts/launch_db.sh) を使ってDockerでサーバーを立てることです。

### 環境変数を設定する

前節で説明した環境変数を設定します。

実行時にターミナルから与えても良いですが、 `.env` で環境変数を設定することができます。
`.env.sample`に環境変数の設定の例があるので、これを用いるのが簡単だと思います。

```
$ cp .env.sample .env
$ vim .env
```

環境変数の意味は[docs/env.md](/docs/env.md)で解説されています。

### マイグレーション

マイグレーションを行うためには [Diesel CLI](https://github.com/diesel-rs/diesel/tree/master/diesel_cli) がインストールされている必要があります。

```
$ cd reing
$ DATABASE_URL='postgresql://username:password@127.0.0.1:5432/database-name' diesel migration run
```

### ビルド&起動

```
$ cargo run
```

## How to contribute

もし機能追加などをしてくださる奇特な方がいらっしゃれば大歓迎です。

まずissueを立てて機能の要求などを相談してくれるとありがたいです。
