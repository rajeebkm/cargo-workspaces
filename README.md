# cargo-workspaces

- touch Cargo.toml
```
[workspace]
members = ["blog_api", "blog_frontend", "blog_shared"]

```
- cargo new --vcs none blog_api
- cargo new --vcs none blog_frontend
- cargo new --vcs none --lib blog_shared

- Let's run `cargo build` in the root directory:
- cargo build
- It will generate `target` directory and `Cargo.lock` file in the root directory.

- To build a specific packages:
- cargo build -p `package_name` (Example: cargo build -p blog_api)
