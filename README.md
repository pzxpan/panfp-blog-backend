# panfp-blog-backend 
panfp博客浏览器后端,用rust的actix-web框架编写,数据库采用postgres；

## 已完成
1.JWT token认证，以client的异步调用方式;
2.文件上传
3.blog数据的管理

## Requirements
- Rust
- Postgres

## Usage
1. 确保postgres数据库能正常访问；
# Install diesel
cargo install diesel_cli --no-default-features --features postgres

# Run db migrations
DATABASE_URL= [你的数据库地址】
diesel migration run

# 运行
cargo run 
