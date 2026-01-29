# Article Recorder API 接口文档

本文档详细说明了 Article Recorder 后端服务提供的所有 API 接口。

## 基础信息
- **Base URL**: `http://localhost:3000` (开发环境)
- **Content-Type**: `application/json`
- **认证方式**: 承载令牌 (Bearer Token)，在请求头中添加 `Authorization: Bearer <JWT_TOKEN>`。

---

## 1. 认证接口 (Authentication)

### 用户注册
- **URL**: `/api/auth/register`
- **方法**: `POST`
- **描述**: 创建新用户。
- **请求体**:
  ```json
  {
    "username": "admin",
    "password": "strongpassword"
  }
  ```
- **响应**:
  - `201 Created`: 注册成功。
  - `400 Bad Request`: 用户名已存在。

### 用户登录
- **URL**: `/api/auth/login`
- **方法**: `POST`
- **描述**: 验证凭据并获取 JWT Token。
- **请求体**:
  ```json
  {
    "username": "admin",
    "password": "strongpassword"
  }
  ```
- **响应**:
  - `200 OK`:
    ```json
    {
      "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
    }
    ```
  - `401 Unauthorized`: 用户名或密码错误。

---

## 2. 文章管理接口 (Article Management)
*以下接口均需要 `Authorization` 请求头。*

### 获取我的文章列表
- **URL**: `/api/articles`
- **方法**: `GET`
- **描述**: 获取当前登录用户创建的所有文章。
- **响应**:
  ```json
  [
    {
      "id": 1,
      "author_id": "...",
      "slug": "hello-world",
      "title": "我的第一篇文章",
      "content": "内容...",
      "is_public": 1,
      "created_at": "2024-01-29T10:00:00Z",
      "updated_at": "2024-01-29T10:00:00Z"
    }
  ]
  ```

### 创建文章
- **URL**: `/api/articles`
- **方法**: `POST`
- **描述**: 发布一篇新文章。
- **请求体**:
  ```json
  {
    "slug": "rust-is-awesome",
    "title": "Rust 入门指南",
    "content": "Rust 是一门非常强大的语言...",
    "is_public": true
  }
  ```
- **响应**: `201 Created`

### 更新文章
- **URL**: `/api/articles/:id`
- **方法**: `PUT`
- **描述**: 修改文章内容或可见性。仅作者可操作。
- **请求体 (可选字段)**:
  ```json
  {
    "title": "更新后的标题",
    "content": "更新后的内容",
    "is_public": false
  }
  ```
- **响应**: `200 OK`, `403 Forbidden` (非作者), `404 Not Found`。

### 删除文章
- **URL**: `/api/articles/:id`
- **方法**: `DELETE`
- **描述**: 永久删除文章。仅作者可操作。
- **响应**: `204 No Content`, `404 Not Found`。

---

## 3. 内容访问接口 (Public Access)

### 根据 Slug 获取文章
- **URL**: `/:slug`
- **方法**: `GET`
- **描述**: 通过自定义 URL (slug) 访问内容。
- **规则**:
  - **公开文章**: 任何人均可直接访问。
  - **私密文章**: 仅作者携带 Token 访问时可见，否则返回 `404 Not Found`。
- **响应**:
  - `200 OK`:
    ```json
    {
      "id": 1,
      "slug": "rust-is-awesome",
      "title": "Rust 入门指南",
      "content": "...",
      "is_public": 1,
      ...
    }
    ```
  - `404 Not Found`: 文章不存在或无权访问。
