# Reing

Reingは [Peing - 質問箱 -](https://peing.net) のクローンアプリケーションです。

匿名で質問を受け付けることができ、回答をTwitterに流すことができます。

## How to setup

Reingのサーバーの建て方を説明します。

以下ではインフラとして[Heroku](https://jp.heroku.com/)を使う前提で話を進めますが、Herokuに依存しているわけではありません。お好きな環境にデプロイすることができます。

### Clone repository

このレポジトリを手元にcloneします。

```
$ git clone git@github.com:genya0407/reing.git
```

### Herokuのapplicationを作成する

cloneしたディレクトリに移動します。
そして、Herokuのアプリケーションを作成し、buildpackとデータベースを設定し、デプロイします。

そこそこ時間がかかります。

```
$ cd reing
$ heroku create
$ heroku buildpacks:add https://github.com/emk/heroku-buildpack-rust.git
$ heroku buildpacks:add https://github.com/sgrif/heroku-buildpack-diesel.git
$ heroku addons:create heroku-postgresql
$ git push heroku master
$ heroku run ./bin/diesel migration run
```

### 環境変数を設定する

以下の環境変数を設定します。設定例が[./.env.sample](./.env.sample)にあります。

※herokuでは `heroku config:set KEY=VALUE` で環境変数を設定することができます。

#### ログイン情報

回答者としてログインするための情報。

- `ADMIN_USERNAME`
  - ログインする際のIDを指定します
- `ADMIN_PASSWORD`
  - ログインする際のパスワードを指定します

#### プロフィール設定

- `PROFILE_USERNAME`
  - トップページに表示される回答者の名前を指定します
- `PROFILE_IMAGE_URL`
  - 例: `https://pbs.twimg.com/profile_images/932824218454016000/lvpcqMk4_400x400.jpg`
  - トップページに表示される回答者のアイコン画像のURLを指定します

#### Twitter関連

回答をTwitterに投稿するための情報。

設定されていない場合は、回答がTwitterに投稿されません。

- `TWITTER_CONSUMER_KEY`
  - TwitterアプリケーションのConsumer keyを指定します
  - Twitterアプリケーションを作成するには、[Twitter Application Management](https://apps.twitter.com/)にログインして、Create New Appをします
- `TWITTER_CONSUMER_SECRET`
  - 同上
- `TWITTER_ACCESS_TOKEN`
  - 同上
- `TWITTER_ACCESS_SECRET`
  - 同上

#### 通知メール関連

質問が投稿されたときに通知メールを送信するための情報。

設定されていない場合は、質問が投稿されても通知メールが送信されません。

- `ADMIN_EMAIL`
  - 通知メールを送る先のメールアドレスを指定します
- `MAILER_FROM`
  - 通知メールの `FROM` 欄として使われるメールアドレスを指定します
- `MAILER_DOMAIN`
  - 例: `smtp.gmail.com`
  - メールサーバーのドメインを指定します
- `MAILER_USERNAME`
  - メールサーバーのアカウントのusernameを指定します
- `MAILER_PASSWORD`
  - メールサーバーのアカウントのpasswordを指定します

#### その他

- `APPLICATION_DOMAIN`
  - 例: `hidden-brook-48005.herokuapp.com`
  - サーバーのドメイン名を指定します
  - 質問投稿通知メールやTwitterに貼るリンクなどを生成するのに使われます
- `DATABASE_URL`
  - 例: `postgresql://username:password@127.0.0.1:5432/database-name`
  - **Herokuを使う場合は設定する必要はありません**

## How to develop

ローカル環境での動かし方について述べます。

### Rustの環境を作る

[rustup](https://rustup.rs/) などを使ってRustの環境を作ります。

開発に使われているコンパイラのバージョンは[RustConfig](./RustConfig)に書いてあります。

### PostgreSQLサーバーを立てる

何らかの手段でPostgreSQLサーバーを立てます。

オススメは[./scripts/launch_db.sh](./scripts/launch_db.sh) を使ってDockerでサーバーを立てることです。

### 環境変数を設定する

前節で説明した環境変数を設定します。

実行時にターミナルから与えても良いですが、 `.env` で環境変数を設定することができます。

```
$ cp .env.sample .env
$ vim .env
```

環境変数の内容自体は、前節のものと同じです。

## How to contribute

もし機能追加などをしてくださる奇特な方がいらっしゃれば大歓迎です。

適当にForkしてPull Requestを投げつけてください。
