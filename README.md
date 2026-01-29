

## 3. 技术栈选型 (企业级标准)

为了实现你要求的“按路由记录文章”功能，建议使用以下组合：

* **Web 框架：** [Axum](https://github.com/tokio-rs/axum) (基于 Tokio，目前最流行的企业级选择，底层使用了你提到的 `matchit`)。
* **数据库 ORM：** [SQLx](https://github.com/launchbadge/sqlx) (编译时检查 SQL 语法) 或 [SeaORM](https://www.google.com/search?q=https://www.sea-orm.com/) (更像 TypeORM)。
* **序列化：** [Serde](https://serde.rs/) (处理 JSON 的神器)。
* **鉴权：** [Jsonwebtoken](https://github.com/Keats/jsonwebtoken)。

---

## 4. 项目实战文档：文章记录系统

### 目录结构建议

```text
.
├── src/
│   ├── main.rs          # 程序入口
│   ├── routes/          # 路由处理器 (类似 NestJS 的 Controller)
│   ├── models/          # 数据库模型
│   ├── middleware/      # JWT 鉴权中间件
│   └── database.rs      # 数据库连接池配置
├── Cargo.toml           # 依赖管理 (类似 package.json)
├── Dockerfile           # 镜像打包配置
└── .github/workflows/   # CI/CD 自动化

```

### 核心配置文件 `Cargo.toml`

```toml
[package]
name = "article-service"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "0.7"
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
sqlx = { version = "0.7", features = ["runtime-tokio", "tls-rustls", "postgres", "macros"] }
jsonwebtoken = "9.0"
bcrypt = "0.15" # 密码加密
tower-http = { version = "0.5", features = ["cors"] } # 处理跨域

```

---

## 5. Docker 镜像与 GitHub Action 自动化

### Dockerfile (多阶段构建)

为了让镜像体积最小化（通常只有 20MB 左右），我们需要使用多阶段构建。

```dockerfile
# 阶段 1: 编译阶段
FROM rust:1.75-slim as builder
WORKDIR /app
COPY . .
RUN cargo build --release

# 阶段 2: 运行阶段
FROM debian:bookworm-slim
WORKDIR /app
# 从编译阶段拷贝二进制文件
COPY --from=builder /app/target/release/article-service /app/article-service
# 安装必要的运行库
RUN apt-get update && apt-get install -y libssl-dev ca-certificates && rm -rf /var/lib/apt/lists/*
EXPOSE 3000
CMD ["./article-service"]

```

### GitHub Actions 配置

在 `.github/workflows/docker-build.yml` 中创建，实现推送到 GitHub 后自动构建镜像。

```yaml
name: Docker Build & Push

on:
  push:
    branches: [ "main" ]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Login to GitHub Container Registry
        uses: docker/login-action@v2
        with:
          registry: ghcr.io
          username: ${{ github.actor }}
          password: ${{ secrets.GITHUB_TOKEN }}

      - name: Build and push
        uses: docker/build-push-action@v4
        with:
          push: true
          tags: ghcr.io/${{ github.repository }}/article-service:latest

```

---

## 6. 核心逻辑实现：权限与公开访问

在 Axum 中，你可以利用 `Extractors` (提取器) 来实现权限校验。

> **逻辑点：** > 1. 公开路由：通过 `slug` 查询数据库，只需满足 `is_public == true`。
> 2. 编辑路由：中间件解析 JWT 拿到 `user_id`，查询 `author_id == user_id`。

---

### 下一步行动建议

1. **创建仓库：** 在 GitHub 创建一个名为 `rust-article-recorder` 的仓库。
2. **本地运行：** 执行 `cargo new article-service` 初始化项目。
3. **尝试 Hello World：** 先用 `Axum` 跑通一个最简单的接口。

**需要我为你写一个简单的 Axum + JWT 鉴权中间件的 Rust 代码示例吗？**