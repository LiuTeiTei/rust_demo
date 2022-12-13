# Cargo & Cargo.io

## 发布配置

release profiles：

+ 预定义的；
+ 可定制的：可以使用不同的配置，对代码编程拥有更多的控制；
+ 每一个 profile 配置都彼此相互独立。



Cargo 有两个主要的配置：

+ dev profile：适用于开发，`cargo build`；
+ release profile：适用于发布`cargo build --release`。



自定义 profile：

+ Cargo 给每个 profile 都提供了默认的配置；

+ 可以在 *Cargo.toml* 里添加 `[profile.xxx]` 区域来自定义 profile；

+ 可以覆盖默认配置：

  ```rust
  [profile.dev]
  opt-level = 0
  
  [profile.release]
  opt-level = 3
  ```

+ 每个配置的设置和其默认值的完整列表：[Cargo 的文档](https://doc.rust-lang.org/cargo/reference/profiles.html)。



## Crate.io

crate 的注册表在 https://crates.io/ 中

+ 分发已注册的包的源代码；
+ 主要托管开源的代码。
