## 環境変数を設定する

Reingを動かすためには環境変数を設定する必要があります。

また、Reingは起動時のカレントディレクトリにある`.env`ファイルを読み込みます。
設定例が[/.env.sample](/.env.sample)にあります。

### 必須

- `ADMIN_USERNAME`
  - ログインする際のIDを指定します
- `ADMIN_PASSWORD`
  - ログインする際のパスワードを指定します
- `PROFILE_IMAGE_URL`
  - 例: `https://pbs.twimg.com/profile_images/932824218454016000/lvpcqMk4_400x400.jpg`
  - トップページに表示される回答者のアイコン画像のURLを指定します
- `DATABASE_URL`
  - 例: `postgresql://username:password@127.0.0.1:5432/database-name`
  - **Herokuを使う場合は設定する必要はありません**
- `TWITTER_SCREEN_NAME` or `PROFILE_USERNAME`
  - トップページに表示される回答者の名前を指定します
  - `TWITTER_SCREEN_NAME`が指定されている場合，サーバー起動のタイミングでTWITTERのAPIを叩いてユーザー名を取得します
  - `TWITTER_SCREEN_NAME`が指定されていない場合は，`PROFILE_USERNAME`を回答者の名前として使用します

### 任意

- 共通
  - `APPLICATION_DOMAIN`
    - 例: `hidden-brook-48005.herokuapp.com`
    - サーバーのドメイン名を指定します
    - 質問投稿通知メールやTwitterに貼るリンクなどを生成するのに使われます
- Twitter関連
  - `TWITTER_CONSUMER_KEY`
    - TwitterアプリケーションのConsumer keyを指定します
    - Twitterアプリケーションを作成するには、[Twitter Application Management](https://apps.twitter.com/)にログインして、Create New Appをします
  - `TWITTER_CONSUMER_SECRET`
    - 同上
  - `TWITTER_ACCESS_TOKEN`
    - 同上
  - `TWITTER_ACCESS_SECRET`
    - 同上
- 通知メール関連
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
