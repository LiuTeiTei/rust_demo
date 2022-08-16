# 错误处理

+ Rust 通过错误处理来提高可靠性；
+ Rust 将错误分为两种：
  + 可恢复，例如文件未找到，可再次尝试；
  + 不可恢复，bug，例如访问的索引超过返回；
+ 大多数语言并不区分这两种错误，采用类似异常这样方式统一处理，而 Rust 没有类似异常的机制，而是分别处理：
  + `Result<T, E>` 类型，用于处理可恢复的错误；
  + `panic!` 宏，在程序遇到不可恢复的错误时停止执行。



## panic!

**当 `panic!` 宏执行时**

+ 程序会打印一个错误信息；

+ 展开 unwind，清理调用栈 stack；

  + 默认情况下，程序会展开调用栈，Rust 沿着调用栈往回走，清理每个遇到的函数中的数据，此时程序工作量会比较大；

  + 也可以通过设置，立即终止调用栈，不进行清理直接停止程序，内存稍后由 OS 进行清理；

  + 在 `Cargo.toml` 中通过设置，将展开改为终止，可以使二进制文件更小：

    ```rust
    [profile.release]
    panic = 'abort'
    ```

+ 退出程序。



**使用 `panic!` 的 backtrace**

+ `panic!` 可能出现在写的代码中，也可能出现在所依赖的代码中，可通过调用 `panic!` 的函数的回溯信息来定位引起问题的代码；
+ 将 `RUST_BACKTRACE` 环境变量设置为任何不是 0 的值可获取 backtrace；
+ 为了获取带有这些信息的 backtrace，必须启用 debug 标识，当不使用 `--release` 参数运行 `cargo build` 或 `cargo run` 时 debug 标识会默认启用。



## Result 枚举

**定义**

+ 和 Option 枚举一样，Result 及其变体由 prelude 导入作用域；

  ```rust
  enum Result<T, E> {
    Ok<T>,
    Err<E>,
  }
  ```

+ `T` 代表成功时返回的 `Ok` 成员中的数据的类型；

+ `E` 代表失败时返回的 `Err` 成员中的错误的类型；

+ `T` 和 `E` 是泛型类型参数：

  ```rust
  use std::fs::File;
  
  fn main() {
      // file 的类型是 std::result::Result<std::fs::File, std::io::Error>
      let f = File::open("hello.txt");
  }
  ```



使用 match 表达式处理 Result

+ 匹配：

  ```rust
  let f = match f {
      Ok(file) => file,
      Err(error) => panic!("Problem opening the file: {:?}", error),
  };
  ```

+ 匹配不同的错误：

  ```rust
  use std::fs::File;
  use std::io::ErrorKind;
  
  fn main() {
      let f = File::open("hello.txt");
  
      let f = match f {
          Ok(file) => file,
          Err(error) => match error.kind() {
              ErrorKind::NotFound => match File::create("hello.txt") {
                  Ok(fc) => fc,
                  Err(e) => panic!("Problem creating the file: {:?}", e),
              },
              other_error => {
                  panic!("Problem opening the file: {:?}", other_error)
              }
          },
      };
  }
  ```

+ 用闭包简化 match：

  ```rust
  let f = File::open("hello.txt").unwrap_or_else(|error| {
      if error.kind() == ErrorKind::NotFound {
          File::create("hello.txt").unwrap_or_else(|error| {
              panic!("Problem creating the file: {:?}", error);
          })
      } else {
          panic!("Problem opening the file: {:?}", error);
      }
  });
  ```

+ unwrap 方法：

  ```rust
  let f = File::open("hello.txt").unwrap();
  ```

  + match 表达式的一个辅助方法；
  + 如果 `Result` 值是成员 `Ok`，`unwrap` 会返回 `Ok` 中的值。如果 `Result` 是成员 `Err`，`unwrap` 会为我们调用 `panic!`；
  + 错误方法不能自定义。

+ expect 方法：

  ```rust
  let f = File::open("hello.txt").expect("Failed to open hello.txt");
  ```

  + 与 unwrap 类似，但能指定错误信息。