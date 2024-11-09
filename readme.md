# 🌸 SakuraServe

このアプリケーションは、ポータブルな機能を備えたウェブサーバーで、インストール不要で簡単に使用できます。

## ✨ 特徴

- **🚀 高性能:** 非同期操作と効率的なライブラリを使用して高いパフォーマンスを実現。
- **🔄 非同期処理:** `Tokio` による非同期操作をサポート。
- **🔒 セキュリティ:** セッション管理やパスワードのハッシュ化など、セキュリティ対策を実装。
- **📈 拡張性:** 新機能やモジュールの追加が容易。
- **💬 WebSocketsサポート:** リアルタイムの双方向通信をサポート。

## 🛠️ 技術スタック

- [Rust](https://www.rust-lang.org/) – プログラミング言語
- [Actix Web](https://actix.rs/) – ウェブフレームワーク
- [Serde](https://serde.rs/) – シリアライズおよびデシリアライズ
- [Tokio](https://tokio.rs/) – 非同期ランタイム
- [Utoipa](https://github.com/juhaku/utoipa) – OpenAPI仕様の自動生成
- [Winres](https://crates.io/crates/winres) – Windowsリソースの操作

## ⚙️ インストール

### 🔧 前提条件

- Rustがインストールされていること（バージョン `1.56.0` 以上）。[公式サイト](https://www.rust-lang.org/tools/install)からインストール可能。
- [Rustup](https://rustup.rs/) がインストールされていること。
- **Windowsの場合:** `rc.exe`（リソースコンパイラ）を含む **Windows SDK** がインストールされていること。

### 📃 インストール手順

1. リポジトリをクローンします：

    ```bash
    git clone https://github.com/あなたのユーザー名/あなたのリポジトリ.git
    ```

2. プロジェクトディレクトリに移動します：

    ```bash
    cd あなたのリポジトリ
    ```

3. プロジェクトをビルドします：

    ```bash
    cargo build --release
    ```

   ビルド時に、`build.rs` を使用して以下のコマンドでアイコンが自動的にビルドされます：

    ```bash
    rc /nologo /fo resources\res.lib resources\resources.rc
    ```

   **注意:** `rc.exe` がPATHに含まれていることを確認してください。通常、Visual StudioまたはWindows SDKと一緒にインストールされます。

### 💡 `rc.exe` とは何ですか？

`rc.exe` は **Resource Compiler** です。Windowsのアプリケーションで使用されるリソース（アイコン、バージョン情報など）をコンパイルするために使用されます。

#### `rc.exe` の場所

通常、`rc.exe` は以下のいずれかの場所にあります：

- `C:\Program Files (x86)\Windows Kits\10\bin\<バージョン>\x64`
- または Visual Studio のインストールディレクトリ内。

#### `rc.exe` を PATH に追加する方法

1. **システムのプロパティ** を開きます（スタートメニューから「環境変数」と検索して「環境変数の編集」を選択します）。
2. **環境変数** ボタンをクリックします。
3. **システム環境変数** の中から `Path` を選択し、**編集** をクリックします。
4. **新規** をクリックして、`rc.exe` のパス（例: `C:\Program Files (x86)\Windows Kits\10\bin\<バージョン>\x64`）を追加します。
5. **OK** をクリックしてすべてのダイアログを閉じます。

## 🖼️ アイコンのビルド

アイコンを正しくビルドするために、以下の手順を実行してください：

1. **Resource Compiler (rc.exe)** がインストールされていることを確認します。これは[Windows SDK](https://developer.microsoft.com/ja-jp/windows/downloads/windows-sdk/)に含まれています。

2. リソースファイル `resources.rc` をプロジェクトの `resources/` ディレクトリに配置します。

3. リソースをビルドするために以下のコマンドを実行します：

    ```bash
    rc /nologo /fo resources\res.lib resources\resources.rc
    ```

   このコマンドは、リソースファイル `resources.rc` をコンパイルし、実行ファイルに組み込まれるリソースライブラリ `res.lib` を生成します。

4. `Cargo.toml` に設定された `build.rs` がプロジェクトのビルド時に自動的にこのコマンドを実行します。`build.rs` が `rc.exe` を呼び出すように正しく設定されていることを確認してください。

## 🚀 使用方法

### サーバーの起動

ビルド後、以下のコマンドでサーバーを起動できます：

```bash
cargo run --release
```

または、実行可能ファイルを直接使用します：

```bash
./target/release/SakuraServe
```

### ⚙️ 設定

サーバーの設定は `.env` ファイルまたは環境変数を通じて行います。

#### `.env` ファイルの例

```env
PORT=8080
DATABASE_URL=postgres://user:password@localhost/dbname
```

## 🌐 API 説明とエンドポイント例

- **POST /api/auth/signUp**

  ユーザーを登録します。

  **ペイロードサンプル:**

  ```json
  {
    "username": "yamada",
    "login": "yamada123",
    "password": "mypassword",
    "email": "yamada@example.com",
    "phone_number": "08012345678",
    "date_of_birth": "1990-01-01",
    "avatar_url": "https://example.com/avatar.png",
    "country": "Japan",
    "region": "Kanto",
    "city": "Tokyo",
    "address": "Shibuya",
    "zip_code": "150-0001"
  }
  ```

  **レスポンス:**

  ```json
  {
    "message": "User registered successfully"
  }
  ```

- **POST /api/auth/signIn**

  ユーザーをログインさせます。

  **ペイロードサンプル:**

  ```json
  {
    "login": "yamada123",
    "password": "mypassword"
  }
  ```

  **リクエストの例:**

  ```bash
  curl http://localhost:8080/api/auth/signIn
  ```

  **レスポンス:**

  ```json
  {
    "message": "Logged in successfully"
  }
  ```

- **POST /api/auth/logout**

  ログアウトします。

  **リクエストの例:**

  ```bash
  curl http://localhost:8080/api/auth/logout
  ```

  **レスポンス:**

  ```json
  {
    "message": "Logged out successfully"
  }
  ```

- **GET /api/users/profile**

  ログインしたユーザーのプロフィール情報を取得します。

  **リクエストの例:**

  ```bash
  curl http://localhost:8080/api/users/profile
  ```

  **レスポンス:**

  ```json
  {
    "id": "uuid",
    "username": "yamada",
    "login": "yamada123",
    "password_hash": "******************",
    "email": "yamada@example.com",
    "phone_number": "08012345678",
    "date_of_birth": "1990-01-01",
    "avatar_url": "https://example.com/avatar.png",
    "registration_date": "2023-01-01T00:00:00Z",
    "last_login_date": "2023-01-02T00:00:00Z",
    "country": "Japan",
    "region": "Kanto",
    "city": "Tokyo",
    "address": "Shibuya",
    "zip_code": "150-0001",
    "credit_cards": [
      {
        "id": "uuid",
        "cardholder_name": "Yamada Taro",
        "card_number": "**** **** **** 1234",
        "expiration_date": "12/25",
        "is_primary": true
      }
    ]
  }
  ```

- **POST /api/users/profile**

  ユーザーのプロフィールを更新します。

  **ペイロードサンプル:**

  ```json
  {
    "username": "yamada",
    "password": "newpassword",
    "email": "yamada@example.com",
    "phone_number": "08012345678",
    "date_of_birth": "1990-01-01",
    "avatar_url": "https://example.com/avatar.png",
    "country": "Japan",
    "region": "Kanto",
    "city": "Tokyo",
    "address": "Shibuya",
    "zip_code": "150-0001",
    "credit_cards": [
      {
        "cardholder_name": "Yamada Taro",
        "card_number": "1234567812345678",
        "expiration_date": "12/25",
        "is_primary": true
      }
    ]
  }
  ```

  **レスポンス:**

  ```json
  {
    "message": "Profile updated successfully"
  }
  ```

- **PUT /api/users/profile**

  プロフィール情報を部分的に更新します。

  **ペイロードサンプル:**

  ```json
  {
    "username": "yamada",
    "phone_number": "08012345678"
  }
  ```

  **リクエストの例:**

  ```bash
  curl -X PUT http://localhost:8080/api/users/profile -H "Content-Type: application/json" -d '{"username": "yamada", "phone_number": "08012345678"}'
  ```

  **レスポンス:**

  ```json
  {
    "message": "Profile updated successfully"
  }
  ```

- **GET /api/users/{id}**

  特定のIDのユーザー情報を取得します。

  **リクエストの例:**

  ```bash
  curl http://localhost:8080/api/users/{id}
  ```

  **レスポンス:**

  ```json
  {
    "id": "uuid",
    "avatar_url": "https://example.com/avatar.png",
    "username": "yamada"
  }
  ```

- **POST /api/users/creditcard/add**

  ユーザーのクレジットカードを追加します。

  **ペイロードサンプル:**

  ```json
  {
    "cardholder_name": "Yamada Taro",
    "card_number": "1234567812345678",
    "expiration_date": "12/25",
    "is_primary": true
  }
  ```

  **レスポンス:**

  ```json
  {
    "message": "Credit card added successfully"
  }
  ```

- **DELETE /api/users/creditcard/{id}**

  特定のIDのクレジットカードを削除します。

  **リクエストの例:**

  ```bash
  curl -X DELETE http://localhost:8080/api/users/creditcard/{id}
  ```

  **レスポンス:**

  ```json
  {
    "message": "Credit card deleted successfully"
  }
  ```

- **GET /api/store/carts/**

  ログインしたユーザーのカートの情報を取得します。

  **リクエストの例:**

  ```bash
  curl http://localhost:8080/api/store/carts/
  ```

  **レスポンス:**

  ```json
  [
    {
      "count": 14,
      "product": {
        "uuid": "1a9d61c4-2f58-41e9-b5a8-c8397a57f7c4",
        "pathurl": "/products/smartphone-samsung-galaxy-s23",
        "article": "S23-256GB-BLACK",
        "price": "999.99",
        "rating": 4,
        "reviews": 256,
        "currency": "USD",
        "discount": 10.0,
        "is_new": true,
        "image": "https://example.com/images/smartphone-samsung-galaxy-s23.jpg",
        "name": "Samsung Galaxy S23",
        "brand": "Samsung",
        "tags": ["smartphone", "electronics", "android"],
        "description": "The Samsung Galaxy S23 is a high-performance smartphone featuring a stunning display, advanced camera system, and long-lasting battery life."
      }
    },
    {
      "count": 14,
      "product": {
        "uuid": "6e0a9419-6b4c-4887-8e50-b26707b88f58",
        "pathurl": "/products/apple-macbook-pro-16",
        "article": "MBP16-M1PRO-2023",
        "price": "2499.99",
        "rating": 5,
        "reviews": 134,
        "currency": "USD",
        "discount": 5.0,
        "is_new": false,
        "image": "https://example.com/images/apple-macbook-pro-16.jpg",
        "name": "Apple MacBook Pro 16\" (2023)",
        "brand": "Apple",
        "tags": ["laptop", "electronics", "macos"],
        "description": "The 2023 MacBook Pro 16-inch with the M1 Pro chip offers unparalleled performance, an exceptional display, and seamless integration with Apple’s ecosystem."
      }
    }
  ]
  ```

- **POST /api/store/carts/add/{product_id}**

  カートに商品を追加します。

  **リクエストの例:**

  ```bash
  curl -X POST http://localhost:8080/api/store/carts/add/{product_id}
  ```

  **レスポンス:**

  ```json
  {
    "message": "Product added to cart successfully"
  }
  ```

- **DELETE /api/store/carts/{product_id}**

  カートから特定の商品を削除します。

  **リクエストの例:**

  ```bash
  curl -X DELETE http://localhost:8080/api/store/carts/{product_id}
  ```

  **レスポンス:**

  ```json
  {
    "message": "Product removed from cart successfully"
  }
  ```

- **GET /api/store/favorites/**

  ログインしたユーザーのお気に入り商品の情報を取得します。

  **リクエストの例:**

  ```bash
  curl http://localhost:8080/api/store/favorites/
  ```

  **レスポンス:**

  ```json
  [
    {
      "uuid": "1a9d61c4-2f58-41e9-b5a8-c8397a57f7c4",
      "pathurl": "/products/smartphone-samsung-galaxy-s23",
      "article": "S23-256GB-BLACK",
      "price": "999.99",
      "rating": 4,
      "reviews": 256,
      "currency": "USD",
      "discount": 10.0,
      "is_new": true,
      "image": "https://example.com/images/smartphone-samsung-galaxy-s23.jpg",
      "name": "Samsung Galaxy S23",
      "brand": "Samsung",
      "tags": ["smartphone", "electronics", "android"],
      "description": "The Samsung Galaxy S23 is a high-performance smartphone featuring a stunning display, advanced camera system, and long-lasting battery life."
    },
    {
      "uuid": "6e0a9419-6b4c-4887-8e50-b26707b88f58",
      "pathurl": "/products/apple-macbook-pro-16",
      "article": "MBP16-M1PRO-2023",
      "price": "2499.99",
      "rating": 5,
      "reviews": 134,
      "currency": "USD",
      "discount": 5.0,
      "is_new": false,
      "image": "https://example.com/images/apple-macbook-pro-16.jpg",
      "name": "Apple MacBook Pro 16\" (2023)",
      "brand": "Apple",
      "tags": ["laptop", "electronics", "macos"],
      "description": "The 2023 MacBook Pro 16-inch with the M1 Pro chip offers unparalleled performance, an exceptional display, and seamless integration with Apple’s ecosystem."
    }
  ]
  ```

- **POST /api/store/favorites/add/{product_id}**

  お気に入り商品に商品を追加します。

  **リクエストの例:**

  ```bash
  curl -X POST http://localhost:8080/api/store/favorites/add/{product_id} -H "Content-Type: application/json"
  ```

  **レスポンス:**

  ```json
  {
    "message": "Product added to favorites successfully"
  }
  ```

- **DELETE /api/store/favorites/{product_id}**

  お気に入り商品から特定の商品を削除します。

  **リクエストの例:**

  ```bash
  curl -X DELETE http://localhost:8080/api/store/favorites/{product_id}
  ```

  **レスポンス:**

  ```json
  {
    "message": "Product removed from favorites successfully"
  }
  ```

### **GET `/api/store/orders/`**
現在のユーザーの注文一覧を取得するためのエンドポイントです。

**ペイロードサンプル:**
```json
[
  {
    "order_id": "注文のuuid",
    "user_id": "ユーザーのuuid",
    "items": [
      {
        "uuid": "1a9d61c4-2f58-41e9-b5a8-c8397a57f7c4",
        "pathurl": "/products/smartphone-samsung-galaxy-s23",
        "article": "S23-256GB-BLACK",
        "price": "999.99",
        "rating": 4,
        "reviews": 256,
        "currency": "USD",
        "discount": 10.0,
        "is_new": true,
        "image": "https://example.com/images/smartphone-samsung-galaxy-s23.jpg",
        "name": "Samsung Galaxy S23",
        "brand": "Samsung",
        "tags": ["smartphone", "electronics", "android"],
        "description": "The Samsung Galaxy S23 is a high-performance smartphone featuring a stunning display, advanced camera system, and long-lasting battery life."
      },
      {
        "uuid": "6e0a9419-6b4c-4887-8e50-b26707b88f58",
        "pathurl": "/products/apple-macbook-pro-16",
        "article": "MBP16-M1PRO-2023",
        "price": "2499.99",
        "rating": 5,
        "reviews": 134,
        "currency": "USD",
        "discount": 5.0,
        "is_new": false,
        "image": "https://example.com/images/apple-macbook-pro-16.jpg",
        "name": "Apple MacBook Pro 16\" (2023)",
        "brand": "Apple",
        "tags": ["laptop", "electronics", "macos"],
        "description": "The 2023 MacBook Pro 16-inch with the M1 Pro chip offers unparalleled performance, an exceptional display, and seamless integration with Apple’s ecosystem."
      }
    ],
    "total_price": 0.0,
    "created_at": "作成日",
    "discount": null,
    "promo_code": null,
    "delivery_address": "配達先住所",
    "payment_card_number": "カード番号",
    "order_status": "注文状況"
  }
]
```

### **POST `/api/store/orders/create`**
新しい注文を作成するためのエンドポイントです。

**ペイロードサンプル:**
  ```json
    {
      "product_ids": ["uuid1", "uuid2"],
      "discount": 0.0,
      "promo_code": "PROMO2024",
      "delivery_address": "設定住所",
      "payment_card_number": "4111 1111 1111 1111",
      "order_status": "Received"
    }
  ```

**レスポンス:**
  ```json
    {
      "message": "Order created successfully"
    }
  ```

### **DELETE `/api/store/orders/{order_id}`**
注文をその ID を基づいて削除するためのエンドポイントです。

**レスポンス:**
  ```json
    {
      "message": "Order deleted successfully"
    }
  ```

## ⚠️ エラーハンドリング

- **400 Bad Request**：無効なリクエストが提供された場合
- **401 Unauthorized**：ログインされていない場合
- **404 Not Found**：特定の商品が見つからない場合
- **500 Internal Server Error**：サーバ内部で問題が発生した場合

## ✅ テスト

プロジェクトのテストを実行するには、以下のコマンドを使用します：

```bash
cargo test
```

## 🤝 コントリビューション

コミュニティからの貢献を歓迎します。貢献方法は以下の通りです：

1. リポジトリをフォークします。
2. 新しいブランチを作成します（例：`git checkout -b feature/新機能`）。
3. 変更を加え、コミットします（例：`git commit -m '新機能を追加'`）。
4. ブランチに変更をプッシュします（例：`git push origin feature/新機能`）。
5. プルリクエストを作成します。

## 📄 ライセンス

このプロジェクトはMITライセンスの下でライセンスされています。詳細は [LICENSE](LICENSE) ファイルを参照してください。

## 🛣️ 連絡先

クシススミタ – [your.email@example.com](mailto:your.email@example.com)

プロジェクトリンク: [https://github.com/あなたのユーザー名/あなたのリポジトリ](https://github.com/あなたのユーザー名/あなたのリポジトリ)

## 📝 Cargo.toml

参考のため、`Cargo.toml` の内容を以下に示します：

```toml
[package]
name = "web-server"
version = "1.0.0"
edition = "2021"
build = "build.rs"
authors = ["クシススミタ"]
description = "This application is a web server with portable functionality, allowing it to be used without installation and with ease of use."

[[bin]]
name = "SakuraServe"
path = "src/main.rs"

[dependencies]
actix-web = { version = "4.0", features = ["cookies"] }
actix-files = "0.6"
actix-session = { version = "0.10.1", features = ["cookie-session"] }
serde = { version = "1.0.164", features = ["derive"] }
serde_json = "1.0"
bcrypt = "0.15.1"
dotenv = "0.15"
env_logger = "0.11.5"
log = "0.4"
webbrowser = "1.0.2"
tokio = { version = "1", features = ["full"] }
uuid = { version = "1", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
utoipa = "5.2.0"
utoipa-swagger-ui = { version = "8", features = ["actix-web"] }
winres = "0.1.12"
windows = "0.58.0"
```

## 🔗 追加の推奨事項

- **🖼️ スクリーンショットやGIF:** サーバーにウェブインターフェースがある場合や、便利なデモがある場合は、スクリーンショットやアニメーションを追加してください。
- **📚 ドキュメント:** プロジェクトが大規模な場合、別途ドキュメントを作成し、READMEからリンクすることを検討してください。
- **🛠️ ビルドステータス:** ビルドステータスやテストカバレッジなどのバッジ（badges）を追加すると良いでしょう。CI/CDサービスを利用している場合に便利です。
