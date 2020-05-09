# panfp-blog-backend 
panfp博客浏览器后端,用rust的actix-web框架编写；

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
