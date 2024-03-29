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



**使用 match 表达式处理 Result**

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



## 传播错误

+ 当编写一个可能会失败的函数时，除了在这个函数中处理错误外，还可以选择让调用者知道这个错误并决定该如何处理：

  ```rust
  use std::fs::File;
  use std::io::Read;
  
  fn read_username_from_file() -> Result<String, io::Error> {
      let f = File::open("hello.txt");
  
      let mut f = match f {
          Ok(file) => file,
          Err(e) => return Err(e),
      };
  
      let mut s = String::new();
  
      match f.read_to_string(&mut s) {
          Ok(_) => Ok(s),
          Err(e) => Err(e),
      }
  }
  
  fn main() {
    read_username_from_file()
  }
  ```

  + `fs::read_to_string` 的函数：将文件读取到一个字符串。它会打开文件、新建一个 String、读取文件的内容，并将内容放入 String，接着返回它。

+ `?` 运算符是传播错误的一种简写：

  ```rust
  fn read_username_from_file() -> Result<String, io::Error> {
      let mut f = File::open("hello.txt")?;
      let mut s = String::new();
      f.read_to_string(&mut s)?;
      Ok(s)
  }
  ```

  + 如果 `Result` 的值是 `Ok`，这个表达式将会返回 `Ok` 中的值而程序将继续执行；
  + 如果值是 `Err`，`Err` 中的值将作为整个函数的返回值，就好像使用了 `return` 关键字一样，这样错误值就被传播给了调用者。

+ 可以在 `?` 之后直接使用链式方法调用：

  ```rust
  fn read_username_from_file() -> Result<String, io::Error> {
      let mut s = String::new();
      File::open("hello.txt")?.read_to_string(&mut s)?;
      Ok(s)
  }
  ```

+ `?` 运算符只能用于返回 Result 的函数；

  + 被 `?` 所应用的错误，会隐式被 from 函数处理；
  + 当 `?` 调用 from 时，它所接收的错误类型会被转化为当前函数返回类型所定义的错误类型；
  + form 函数是一个 trait，由 std::convert::From 提供；
  + 只要每个错误类型实现了转换为所返回的错误类型的 from 函数，就可以针对不同错误原因返回同一种错误类型；

+ main 函数返回类型是 `()`，不能直接使用`?` 运算符：

  ```rust
  use std::error::Error;
  use std::fs::File;
  
  fn main() -> Result<(), Box<dyn Error>> {
      let f = File::open("hello.txt")?;
      Ok(())
  }
  ```

  + main 函数返回类型也可以是 Result<T, E>；
  + `Box<dyn Error>` 是 trait 对象，任何可能的错误类型。



# 泛型

+ 可以提高代码的复用能力，处理重复代码问题；
+ 泛型是具体类型或其他属性的抽象替代。可以简单理解为，带泛型的代码不是最终的代码，而是一种模板，里面有一些占位符，编译器再编译时会将占位符替换成具体的类型；
+ 使用泛型的代码和使用具体类型的代码运行速度时一样的，Rust 在编译时会进行单态化；
  + 单态化：编译时将泛型替换成具体类型的过程。
+ 一般使用 `T` 作为泛型数据类型，表示 Type。



## 函数

+ 一般将参数类型和返回类型定义为 `T`：

  ```rust
  fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
      let mut largest = &list[0];
  
      for item in list {
          if item > largest {
              largest = item
          }
      }
  
      largest
  }
  
  fn main() {
      let number_list = vec![1, 2, 35, 9, 444];
      let largest_number = largest(&number_list);
      println!("the largest number is {}", largest_number);
  
      let char_list = vec!['y', 'm', 'a', 'q'];
      let largest_char = largest(&char_list);
      println!("The largest char is {}", largest_char);
  }
  ```

  

## Struct

+ 一般将字段定义为 `T`：

  ```rust
  struct Point<T> {
      x: T,
      y: T,
  }
  
  fn main() {
      let integer = Point { x: 5, y: 10 };
      let float = Point { x: 1.0, y: 4.0 };
  }
  ```

+ 也可以定义不同类型但仍然是泛型的 `Point` 结构体：

  ```rust
  struct Point<T, U> {
      x: T,
      y: U,
  }
  
  fn main() {
      let both_integer = Point { x: 5, y: 10 };
      let both_float = Point { x: 1.0, y: 4.0 };
      let integer_and_float = Point { x: 5, y: 4.0 };
  }
  ```



## Enum

+ 将枚举的变体定义为 `T`：

  ```rust
  enum Option<T> {
      Some(T),
      None,
  }
  ```

+ 枚举也可以拥有多个泛型类型：

  ```rust
  enum Result<T, E> {
      Ok(T),
      Err(E),
  }
  ```



## 方法

+ 为 struct 或 enum 实现方法时，可以在定义中使用泛型；

+ 把  `T` 放在 impl 关键字后，表示在类型 `T` 上实现方法：

  ```rust
  impl<T> Point<T> {
      fn x(&self) -> &T {
          &self.x
      }
  }
  ```

+ 也可以只针对具体类型实现方法：

  ```rust
  impl Point<f32> {
      fn distance_from_origin(&self) -> f32 {
          (self.x.powi(2) + self.y.powi(2)).sqrt()
      }
  }
  ```

+ struct 中的泛型类型参数可以和方法的泛型类型参数不同：

  ```rust
  struct Point<X1, Y1> {
      x: X1,
      y: Y1,
  }
  
  impl<X1, Y1> Point<X1, Y1> {
      fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
          Point {
              x: self.x,
              y: other.y,
          }
      }
  }
  
  fn main() {
      let p1 = Point { x: 5, y: 10.4 };
      let p2 = Point { x: "Hello", y: 'c' };
      let p3 = p1.mixup(p2);
  
      println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
  }
  ```



# Trait

+ trait 告诉 Rust 编译器某个特定类型拥有可能与其他类型共享的功能；
+ 可以通过 trait 以一种抽象的方式定义共享的行为；
+ 可以使用 trait bounds 指定泛型是任何拥有特定行为的类型。



## 定义 trait

+ trait 定义，是把方法签名组合起来放在一起，来定义一个实现某些目的所必需的一组行为；

+ 使用 `trait` 关键字来声明一个 trait，后面跟着 trait 的名字：

  ```rust
  pub trait Summary {
      fn summarize(&self) -> String;
  }
  ```

+ trait 体中可以有多个方法，每个方法签名占一行，且以 `;` 结尾；

+ 定义的时候，可以只有方法签名，没有具体实现；

+ 实现该 trait 的类型必须提供具体的方法实现。



## 实现 trait

+ 也是使用 `impl` 关键字，但与为类型实现方法不太一样：

  ```rust
  // 为类型实现方法
  impl <类型名> {
    // 函数具体实现
  }
  
  // 
  impl <Trait名> for <类型名> {
    // 函数签名
  }
  ```

+ trait 必须和类型一起引入作用域以便使用额外的 trait 方法：

  ```rust
  // lib.rs
  pub trait Summary {
      fn summarize(&self) -> String;
  }
  
  pub struct NewsArticle {
      pub headline: String,
      pub location: String,
      pub author: String,
      pub content: String,
  }
  
  // 定义关联函数
  impl NewsArticle {
      pub fn summarize(article: NewsArticle) -> String {
          format!("{}: {}", article.author, article.content)
      }
  }
  
  // 定义 trait
  impl Summary for NewsArticle {
      fn summarize(&self) -> String {
          format!("{}, by {} ({})", self.headline, self.author, self.location)
      }
  }
  
  pub struct Tweet {
      pub username: String,
      pub content: String,
      pub reply: bool,
      pub retweet: bool,
  }
  
  impl Summary for Tweet {
      fn summarize(&self) -> String {
          format!("{}: {}", self.username, self.content)
      }
  }
  ```

  ```rust
  // main.rs
  use trait_demo::{NewsArticle, Summary, Tweet};
  
  fn main() {
      let tweet = Tweet {
          username: String::from("username"),
          content: String::from("content"),
          reply: false,
          retweet: false,
      };
      println!("tweet: {}", tweet.summarize()); // tweet: username: content
  
      let article = NewsArticle {
          headline: String::from("headline"),
          location: String::from("location"),
          author: String::from("author"),
          content: String::from("content"),
      };
      println!("article {}", article.summarize()); // article headline, by author (location)
      println!("article {}", NewsArticle::summarize(article)); // article author: content
  }
  ```



### 实现 trait 的约束

+ 这个类型或者这个 trait 是在本地 crate 中定义的，才能在某个类型上实现某个 trait；
+ 不能为外部类型实现外部 trait：
  + 这个限制是被称为**相干性**的程序属性的一部分；
  + 更具体的说是**孤儿规则**，因为父类型不存在；
  + 这条规则确保了其他人编写的代码不会破坏你代码，反之亦然。没有这条规则的话，两个 crate 可以分别对相同类型实现相同的 trait，而 Rust 将无从得知应该使用哪一个实现。




# 生命周期

+ Rust 的每个引用都有自己的生命周期；
+ 生命周期使引用保持有效的作用域；
+ 大多数情况下生命周期是隐式的、可以被推断的；
+ 当引用的生命周期可能以不同的方式互相关联时，需要手动标注生命周期；



## 避免悬垂引用

+ 例如：

  ```rust
  fn main() {
      {
          let r;
          {
              let x = 5;
              r = &x;  // error[E0597]: `x` does not live long enough
          }
          println!("r: {}", r);
      }
  }
  
  ```

+ Rust 编译器有一个**借用检查器** borrow checker，用于比较作用域来确保所有的借用都是有效的。



## 生命周期标注语法

+ 生命周期的标注不会改变引用的生命周期长度；

+ 当指定了泛型生命周期参数，函数可以接收带有任何生命周期的引用；

+ 标注例子：

  ```rust
  &i32        // 引用
  &'a i32     // 带有显式生命周期的引用
  &'a mut i32 // 带有显式生命周期的可变引用
  ```

  + 参数名称以 `'` 开头，通常小写且非常短，例如 `'a`；
  + 标注位置在引用的 `&` 后面，使用空格将标注和引用类型分开。

+ 单个的生命周期注解本身没有多少意义，因为生命周期的标注，是为了描述了多个引用的生命周期间的关系；



## 函数签名的生命周期标注

+ 函数签名中的生命周期标注：

  ```rust
  fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
      if x.len() > y.len() {
          x
      } else {
          y
      }
  }
  ```

  + 泛型生命周期参数需要声明在函数名和参数列表间的 `<>` 中；
  + 不会改变真实的生命周期长度，只是告诉借用检查器而已；
  + `'a` 得到的实际的生命周期是 x 和 y 中生命周期比较短的那一个；

+ 指定生命周期参数的正确方式依赖函数实现的具体功能：

  ```rust
  fn longest<'a>(x: &'a str, y: &str) -> &'a str {
      x
  }
  ```

  + 如果函数总是返回第一个参数，就不需要为参数 `y` 指定一个生命周期。

+ 从函数返回一个引用，返回值的生命周期参数需要与一个参数的生命周期参数相匹配：

  ```rust
  fn longest<'a>(x: &str, y: &str) -> &'a str {
      let result = String::from("really long string");
      result.as_str()  // error[E0515]: cannot return reference to local variable `result`
  }
  ```

  + 如果返回的引用没有指向任何一个参数，那么它指向一个函数内部创建的值。这就是一个悬垂引用，该值将会在函数结束时离开作用域；

  + 这种情况，最好的解决方案是返回一个有所有权的数据类型而不是一个引用，这样函数调用者就需要负责清理这个值了。


+ 生命周期语法是用于将函数的多个参数与其返回值的生命周期进行关联的。一旦他们形成了某种关联，Rust 就有了足够的信息来允许内存安全的操作，并阻止会产生悬垂指针亦或是违反内存安全的行为。



## 结构体定义的生命周期标注

+ Struct 里可以包括**自持有**的类型和**引用**的类型；

+ 如果是引用的类型，需要在每个引用上添加生命周期标注：

  ```rust
  struct ImportantExcerpt<'a> {
      part: &'a str,
  }
  
  fn main() {
      let novel = String::from("Call me Ishmael. Some years ago...");
      let first_sentence = novel.split('.').next().expect("Could not find a '.'");
      let i = ImportantExcerpt {
          part: first_sentence,
      };
  }
  ```



## 生命周期的省略

+ 一般来说：

  + 每个引用都有生命周期；
  + 需要为使用生命周期的函数或 struct 指定生命周期参数。
+ 在 Rust 引用分析中所编入的模式称为**生命周期省略规则**：

  + 这些规则无需开发者来遵守；
  + 这些规则是一系列特定的场景，由编译器会考虑；
  + 如果代码符合这些场景，就无需显示指定生命周期。
+ 省略规则并不提供完整的推断：
  + 如果应用规则后，变量的生命周期仍然是模棱两可的话，编译器会给出一个错误；
  + 需要显示添加生命周期标注，表明引用间的相互关系。
+ 输入生命周期：生命周期出现在函数或者方法的参数中；
+ 输出生命周期：生命周期出现在函数或者方法的返回值中；
+ 编译器采用3条规则在没有显示标注生命周期的情况下，来确定引用的生命周期：
  + 规则1：每个引用类型的参数都有它自己的生命周期；
  + 规则2：如果只有一个输入生命周期参数，那么它被赋予所有输出生命周期参数；
  + 规则3：如果有多个输入生命周期参数，但其中一个参数是 `&self` 或 `&mut self`，说明是个对象的方法，那么 `self` 的生命周期会被赋给所有输出生命周期参数。
  + 规则1适用于输入生命周期，规则2、3适用于输出生命周期。如果编译器检查完这三条规则后仍然存在没有计算出生命周期的引用，编译器将会停止并生成错误。这些规则适用于 `fn` 定义，以及 `impl` 块。



## 方法定义的生命周期标注

+ 在 Struct 上使用生命周期实现方法，语法和泛型参数的语法一样；

+ 在哪生命和使用生命周期参数，依赖于：生命周期参数是否和字段、方法的参数或者返回值有关；

+ Struct 字段的生命周期名在 `impl` 名后声明，在 `struct` 名后使用，这些生命周期是 Struct 类型的一部分；

+ `impl` 块内的方法签名中：

  + 引用必须绑定于 struct 字段引用的生命周期，或者引用是独立的也可以；
  + 生命周期省略规则经常使得方法中的生命周期标注不是必须的。

+ 例子：

  ```rust
  struct ImportantExcerpt<'a> {
      part: &'a str,
  }
  
  impl<'a> ImportantExcerpt<'a> {
      fn level(&self) -> i32 {
          3
      }
  }
  
  impl<'a> ImportantExcerpt<'a> {
      fn announce_and_return_part(&self, announcement: &str) -> &str {
          println!("Attention please: {}", announcement);
          self.part
      }
  }
  
  fn main() {
      let novel = String::from("Call me Ishmael. Some years ago...");
      let first_sentence = novel.split('.').next().expect("Could not find a '.'");
      let i = ImportantExcerpt {
          part: first_sentence,
      };
  }
  ```



## 静态生命周期

+ `'static` 是一个特殊的生命周期，能够存活于整个程序期间；

+ 所有的字符串字面值都拥有 `'static` 生命周期：

  ```rust
  let s: &'static str = "I have a static lifetime.";
  ```

  + 因为字符串的文本被直接储存在程序的二进制文件中的，而这个文件总是可用的，因此所有的字符串字面值都是 `'static` 的。

+ 将引用指定为 `'static` 之前，需要确定这个引用是否真的在整个程序的生命周期里都有效，而不是为了解决悬垂引用或者可用的生命周期不匹配。



# 自动化测试

+ Rust 中的测试就是函数，测试函数是用来验证非测试代码是否和预期一样；
+ 测试函数体通常执行 3 个操作：
  + 准备数据或状态；
  + 运行被测试的代码；
  + 断言结果。



## 测试函数

+ 测试函数需要使用 test 属性进行标注；
  + 属性 attribute 是一段 Rust 代码片段的元数据；
  + 在函数上加 `#[test]` 就可以把函数变成测试函数。
+ 运行测试：
  + 使用 `cargo test` 命令会运行所有的测试函数；
  
  + Rust 会构建一个 Test Runner 可执行文件，会运行标注了 test 的函数，并报告其运行是否成功；
  
  + 当使用 cargo 创建 libary 项目的时候，会生成一个 test module，里面有一个 test 函数：
  
    ```rust
    #[cfg(test)]
    mod tests {
        #[test]
        fn it_works() {
            let result = 2 + 2;
            assert_eq!(result, 4);
        }
    }
    ```
  
  + 可以在项目中添加任意数量的 test module 或测试函数。
+ 测试失败：
  + 测试函数触发了 `panic!` 就表示失败：
  
    ```rust
    #[test]
      fn error() {
          panic!("Make this test fail");
      }
    ```
  
  + 每个测试运行在一个独立的线程中，当主线程看到某个测试线程挂了，就将那个测试标记为失败。



## assert 断言

+ 使用 `assert!` 宏来检查结果：

  + 由标准库提供，用来确定某个状态是否为 `true`；

  + true：测试通过；

  + false：调用 `panic!` 宏，测试失败；

  + 例子：

    ```rust
    struct Rectangle {
        width: u32,
        hight: u32,
    }
    
    impl Rectangle {
        fn can_hold(&self, other: Rectangle) -> bool {
            self.width > other.width && self.hight > other.hight
        }
    }
    
    #[cfg(test)]
    mod tests {
        use super::*;
        
        #[test]
        fn larger_can_hold_small() {
            let lager = Rectangle { width: 8, hight: 7 };
            let small = Rectangle { width: 5, hight: 1 };
            assert!(lager.can_hold(small))
        }
    
        #[test]
        fn larger_cannot_hold_small() {
            let lager = Rectangle { width: 8, hight: 7 };
            let small = Rectangle { width: 5, hight: 1 };
            assert!(!small.can_hold(lager))
        }
    }
    ```

+ 使用 `assert_eq!` 和 `assert_ne!` 宏来测试相等：

  + 由标准库提供，用来确定两个参数是否相等或不等；

  + 实际上使用的就是 `==` 和 `!=`；

  + 断言失败失败时会自动打印出两个参数的值；

    + 使用 debug 格式打印参数；
    + 因此要求参数需要实现 `PartialEq` 和 `Debug` Traits；
    + 所有的基本类型和大部分标准库类型都实现了这些 Trait，对于自定义的结构体和枚举，需要实现 `PartialEq` 才能断言他们的值是否相等；需要实现 `Debug` 才能在断言失败时打印他们的值。

  + 例子：

    ```rust
    pub fn add_two(a: i32) -> i32 {
        a + 2
    }
    
    #[cfg(test)]
    mod tests {
        use crate::add_two;
    
        #[test]
        fn it_add_two() {
            assert_eq!(4, add_two(2));
            assert_ne!(4, add_two(2));
        }
    }
    ```

+ 添加自定义错误信息：

  + 可以向 `assert!`、`assert_eq!` 和 `assert_ne!` 宏传递一个可选的失败信息参数；

  + 在测试失败时这些自定义信息和失败信息会一同打印出来；

  + `assert!` 第一个参数是必填的，自定义信息是可选的第二个参数；

  + `assert_eq!` 和 `assert_ne!` 的前两个参数数必填的，自定义信息是可选的第三个参数；

  + 自定义参数会被传递给 `format!` 宏，可以使用 `{}` 占位符；

  + 例子：

    ```rust
    pub fn greeting(name: &str) -> String {
        String::from("Hello!")
    }
    
    #[cfg(test)]
    mod tests {
        use super::*;
    
        #[test]
        fn greeting_contains_name() {
            let result = greeting("Carol");
            assert!(
                result.contains("Carol"),
                "Greeting did not contain name, value was `{}`",
                result
            );
        }
    }
    ```



## 验证错误处理

+ 除了检查代码是否返回期望的正确的值之外，有时还需要检查代码是否按照期望处理错误；

+ 使用 should_panic 检查 panic：

  + 函数 panic 则测试通过；

  + 函数没有 panic 则测试失败；

  + 例子：

    ```rust
    pub struct Guess {
        value: i32,
    }
    
    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 {
                panic!(
                    "Guess value must be greater than or equal to 1, got {}.",
                    value
                );
            } else if value > 100 {
                panic!(
                    "Guess value must be less than or equal to 100, got {}.",
                    value
                );
            }
    
            Guess { value }
        }
    }
    
    #[cfg(test)]
    mod tests {
        use super::*;
    
        #[test]
        #[should_panic]
        fn greater_than_100() {
            Guess::new(200);
        }
    }
    ```

+ 为了使 `should_panic` 测试结果更精确，可以给 `should_panic` 属性增加一个可选的 `expected` 参数：

  + 将检查错误信息中是否包含传入的参数文本；

  + 例子：

    ```rust
    #[cfg(test)]
    mod tests {
        use super::*;
    
        #[test]
        #[should_panic(expected = "Guess value must be less than or equal to 100")]
        fn greater_than_100() {
            Guess::new(200);
        }
    }
    ```



## 将 Result<T, E> 用于测试

+ 无需 panic，可使用 Result<T, E> 作为返回类型编写测试：

  + 返回 Ok 测试通过；
  + 返回 Err 测试失败；

+ 无需在测试函数上标注 `should_panic`；

+ 例子：

  ```rust
  #[cfg(test)]
  mod tests {
      #[test]
      fn it_works() -> Result<(), String> {
          if 2 + 2 == 4 {
              Ok(())
          } else {
              Err(String::from("two plus two does not equal four"))
          }
      }
  }
  ```



## 控制测试如何运行

+ `cargo test` 在测试模式下编译代码并运行生成的测试二进制文件；

+ 可以指定命令行参数来改变 `cargo test` 的默认行为：

  + 默认行为：并行运行所有参数，并截获测试运行过程中产生的输出，阻止他们被显示出来，使得阅读测试结果相关的内容变得更容易；
  + 针对本身的的命令行参数，紧跟在 `cargo test` 后面：`cargo test --help`；
  + 针对测试可执行程序的命令行参数，放在 ` -- ` 后面：`cargo test -- --help`；

+ 并行运行测试：

  + 默认情况下，使用多个线程并行运行多个测试，这样运行速度快；
  + 但需要确保测试之间不会相互依赖，且不依赖某个共享状态（环境、工作目录、环境变量等等）；

+ `--test-threads` 参数：

  + 不想测试并行运行，或者想要更加精确的控制线程的数量；
  + 传递 `--test-threads` 参数和希望使用线程的数量给测试二进制文件；
  + 例如：`cargo test -- --test-threads=1`；

+ 显示函数输出：

  + 默认情况下，当测试通过时，Rust 的 test 库会截获打印到标准输出的所有内容；
  + 例如，在测试中调用了 `println!` 而测试通过了，将不会在终端看到 `println!` 的输出，只会看到说明测试通过的提示行。如果测试失败了，则会看到所有标准输出和其他错误信息；
  + 可以通过 `cargo test -- --show-output` 来打印出通过测试的函数输出；

+ 通过指定名字来运行部分测试：

  + 将测试名称作为 `cargo test` 参数，可以选择运行哪些测试；
  + 运行单个测试：指定测试名，例如 `cargo test add_two_and_two`；
  + 运行多个测试：指定测试名的一部分，模块名称也可以，例如 `cargo test add`；

+ 忽略某些测试：

  + 使用 `ignore` 属性来标记耗时的测试并排除他们：

    ```rust
    #[cfg(test)]
    mod tests {
        #[test]
        fn it_works() {
            assert_eq!(2 + 2, 4);
        }
    
        #[test]
        #[ignore]
        fn expensive_test() {
            // 需要运行一个小时的代码
        }
    }
    ```

  + 运行被忽略的测试： `cargo test -- --ignored`；



## 测试的组织

+ Rust 对测试的分类：

  + 单元测试：
    + 小、专注；
    + 一次对一个模块进行隔离的测试；
    + 可以测试 private 接口；
  + 集成测试：
    + 在库外部，和其他外部代码一样使用你的代码；
    + 可以在每个测试中使用到多个模块；
    + 只能使用 public 接口；
+ cfg：configuration 配置。
  + 告诉 Rust 下面的内容只有在指定的配置选项下才被包含；
  + 例如配置选项 test，由 Rust 提供，用来编译和运行测试，只有 `cargo test` 指令的时候才会编译代码，包括模块中的 help 函数和 `#[test]` 标注的函数。




### 单元测试

  + 单元测试的目的是在与其他部分隔离的环境中测试每一个单元的代码，以便于快速而准确的某个单元的代码功能是否符合预期；

  + 单元测试与他们要测试的代码共同存放在位于 *src* 目录下相同的文件中；

  + 规范是在每个文件中创建包含测试函数的 `tests` 模块，并使用 `cfg(test)` 标注模块；

  + Rust 允许测试私有函数：

    ```rust
    pub fn add_two(a: i32) -> i32 {
        internal_adder(a, 2)
    }
    
    fn internal_adder(a: i32, b: i32) -> i32 {
        a + b
    }
    
    #[cfg(test)]
    mod tests {
        use super::*;
    
        #[test]
        fn internal() {
            assert_eq!(4, internal_adder(2, 2));
        }
    }
    ```



### 集成测试

+ 集成测试完全位于被测试库外的另一个文件夹；

+ 集成测试的目的是测试库的多个部分能否一起正常工作；

+ 一些单独能正确运行的代码单元集成在一起也可能会出现问题，所以集成测试的覆盖率也是很重要的；

+ *tests* 目录：

  + 需要在项目根目录创建一个 *tests* 目录，与 *src* 同级；

  + *tests* 目录下的每一个测试文件都会被 Rust 当作单独的 crate 来编译，需要将被测试库导入；

  + 不需要 `#[cfg(test)]` 注解，*tests* 目录会被特殊对待，只有在运行 `cargo test` 的时候才会编译 *tests* 目录下的文件；

  + 例子：

    ```rust
    use automated_test;
    
    #[test]
    fn it_add_two() {
        assert_eq!(4, automated_test::add_two(2));
    }
    ```

+ 运行指定的集成测试：

  + 运行一个特定的集成测试：`cargo test 函数名`；
  + 运行某个测试文件内的所有测试：`cargo test --test 文件名`；

+ 集成测试中的子模块：

  + *tests* 目录下每个文件会被编译成单独的 crate；
  + *tests* 目录中的文件不能像 *src* 中的文件那样共享相同的行为；
  + 可以创建 *tests/common/mod.rs* 不是创建 *tests/common.rs*，这是一种 Rust 的命名规范，这样命名告诉 Rust 不要将 `common` 看作一个集成测试文件；
  + *tests* 目录中的子目录不会被作为单独的 crate 编译或作为一个测试结果部分出现在测试输出中；

+ 针对 binary crate 的集成测试：

  + 如果项目是 binary crate 并且只包含 *src/main.rs* 而没有 *src/lib.rs*：
    + 不能在 *tests* 目录下创建集成测试；
    + 无法把 *src/main.rs* 中定义的函数导入作用域；
  + 只有 libary crate 才能向其他 crate 暴露函数；
  + binary crate 意味着单独运行。



# 二进制程序的编写原则

## 关注分离

指导原则：

+ 将程序拆分成 *main.rs* 和 *lib.rs*，并将程序的业务逻辑放入 *lib.rs* 中；
+ 当命令行解析逻辑比较小时，可以保留在 *main.rs* 中；
+ 当命令行解析开始变得复杂时，也同样将其从 *main.rs* 提取到 *lib.rs* 中。



经过这些过程之后保留在 `main` 函数中的责任应该被限制为：

- 使用参数值调用命令行解析逻辑；
- 设置任何其他的配置；
- 调用 *lib.rs* 中的 `run` 函数；
- 如果 `run` 返回错误，则处理这个错误。



这个模式的一切就是为了关注分离：

+ *main.rs* 处理程序运行，而 *lib.rs* 处理所有的真正的任务逻辑；
+ 因为不能直接测试 `main` 函数，这个结构通过将所有的程序逻辑移动到 *lib.rs* 的函数中使得我们可以测试他们，仅仅保留在 *main.rs* 中的代码将足够小以便阅读就可以验证其正确性。



## 测试驱动开发

TDD - Test Driven Development

+ 编写一个失败的测试，并运行它以确保它失败的原因是你所期望的；
+ 编写或修改足够的代码来使新的测试通过；
+ 重构刚刚增加或修改的代码，并确保测试仍然能通过；
+ 从步骤 1 开始重复。



## 标准错误 VS 标准输出

+ 标准输出：stdout，`println!()`；
  + 可以重定向到一个文件。
+ 标准错误：stderr，`eprintln!()`；
  + 不能重定向到一个文件，只会输出到终端上。



# 函数式语言功能

## 闭包 Closures

可以捕获其所在环境的匿名函数：

+ 匿名函数；
+ 可以保存在一个变量中，或作为参数传递给其他函数；
+ 可以在一个地方创建闭包，然后在不同的上下文中执行闭包运算；
+ 可以从其定义的作用域捕获值。



### 定义

+ ```rust
  let expensive_closure = |num| {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
  };
  ```

+ 闭包定义是 `expensive_closure` 赋值的 `=` 之后的部分；

+ 闭包的定义以一对竖线`|`开始，在竖线中指定闭包的参数；

  + 之所以选择这个语法是因为它与 Smalltalk 和 Ruby 的闭包定义类似；
  + 如果有多于一个参数，可以使用逗号分隔，比如 `|param1, param2|`。

+ 参数之后是存放闭包体的大括号；

  + 如果闭包体只有一行则大括号可以省略；
  + 大括号之后闭包的结尾，需要用于 `let` 语句的分号。

+ `let` 语句意味着 `expensive_closure` 包含一个匿名函数的**定义**，不是调用匿名函数的**返回值**。



### 调用

+ ```rust
  println!("Today, do {} pushups!", expensive_closure(intensity));
  ```

+ 调用闭包类似于调用函数，指定存放闭包定义的变量名并后跟包含期望使用的参数的括号。



### 类型推断

+ 闭包不要求标注参数和返回值的类型；

  + 函数中需要类型注解是因为他们是暴露给用户的显式接口的一部分，需要严格定义以达成共识；
  + 闭包并不用于这样暴露在外的接口，且储存在变量中，不用命名或暴露给库的用户调用。

+ 闭包通常很短，只在狭小的上下文中工作，编译器能可靠的推断参数和返回值的类型；

+ 可以手动添加类型标注；

  + ```rust
    let expensive_closure = |num: u32| -> u32 {
      println!("calculating slowly...");
      thread::sleep(Duration::from_secs(2));
      num
    };
    ```

+ 与函数定义的区别：

  ```rust
  fn  add_one_v1   (x: u32) -> u32 { x + 1 }
  let add_one_v2 = |x: u32| -> u32 { x + 1 };
  let add_one_v3 = |x|             { x + 1 };
  let add_one_v4 = |x|               x + 1  ;
  ```

+ 函数的定义最终只会为参数/返回值推断出唯一具体的类型：

  ```rust
  let example_closure = |x| x;
  
  let s = example_closure(String::from("hello")); 
  let n = example_closure(5);
  // error[E0308]: mismatched types
  // expected struct `String`, found integer
  ```

  + 第一次使用 `String` 值调用 `example_closure` 时，编译器推断 `x` 和此闭包返回值的类型为 `String`；
  + 接着这些类型被锁定进闭包 `example_closure` 中，如果尝试对同一闭包使用不同类型则会得到类型错误。



### Fn Trait

+ 一个持有闭包及其调用结果的 struct：

  + 只会在需要结果时执行闭包；

  + 可以缓存结果；

  + 该模式为记忆化（memoization）或延迟计算（lazy evaluation）。

+ 如何让 struct 持有闭包：

  + 由于 struct 的定义需要知道所有字段的类型，因此需要指明闭包的类型；
  + 每个闭包实例都有自己唯一的匿名类型，即使两个闭包签名完全一样；
  + 借助泛型和 Trait Bound。

+ Fn Trait：

  + 由标准库提供；
  + 所有闭包至少实现了以下 trait 之一：Fn、FnMut、FnOnce。

+ 例子：

  ```rust
  use std::thread;
  use std::time::Duration;
  
  struct Cacher<T>
  where
      T: Fn(u32) -> u32,
  {
      calculation: T,
      value: Option<u32>,
  }
  
  impl<T> Cacher<T>
  where
      T: Fn(u32) -> u32,
  {
      fn new(calculation: T) -> Cacher<T> {
          Cacher {
              calculation,
              value: None,
          }
      }
  
      fn value(&mut self, arg: u32) -> u32 {
          match self.value {
              Some(v) => v,
              None => {
                  let v = (self.calculation)(arg);
                  self.value = Some(v);
                  v
              }
          }
      }
  }
  
  fn generate_workout(intensity: u32, random_number: u32) {
      let mut expensive_result = Cacher::new(|num| {
          println!("calculating slowly...");
          thread::sleep(Duration::from_secs(2));
          num
      });
  }
  ```

  + `Cacher` 结构体的字段是私有的，因为我们希望 `Cacher` 管理这些值，而不是任由调用代码潜在的直接改变他们；
  + `Cacher::new` 函数获取一个泛型参数 `T`，它定义于 `impl` 块上下文中并与 `Cacher` 结构体有着相同的 trait bound；
  + `Cacher::new` 返回一个在 `calculation` 字段中存放了指定闭包和在 `value` 字段中存放了 `None` 值的 `Cacher` 实例；
  + 当调用代码需要闭包的执行结果时，不同于直接调用闭包，它会调用 `value` 方法。这个方法会检查 `self.value` 是否已经有了一个 `Some` 的结果值；如果有，它返回 `Some` 中的值并不会再次执行闭包；
  + 如果 `self.value` 是 `None`，则会调用 `self.calculation` 中储存的闭包，将结果保存到 `self.value` 以便将来使用，并同时返回结果值。

+ Cacher 实现的限制：

  +  `Cacher` 实例假设对于 `value` 方法的任何 `arg` 参数值总是会返回相同的值；
  +  被限制为只接受获取一个 `u32` 值并返回一个 `u32` 值的闭包。



### 使用闭包捕获上下文

+ 闭包可以访问定义它的作用域内的变量，而普通函数则不能：

  ```rust
  fn main() {
      let x = 4;
      // error[E0434]: can't capture dynamic environment in a fn item
      // help: use the `|| { ... }` closure form instead
      fn equal_to_x(z: i32) -> bool {
          z == x
      }
      let y = 4;
      assert!(equal_to_x(y))
  }
  
  fn main() {
      let x = 4;
      let equal_to_x = |z| z == x;
      let y = 4;
      assert!(equal_to_x(y))
  }
  ```

+ 会产生内存开销；

+ 闭包可以通过三种方式捕获其环境：

  + `FnOnce` 消费从周围作用域捕获的变量。为了消费捕获到的变量，闭包必须获取其所有权并在定义闭包时将其移动进闭包，其名称的 `Once` 部分代表了闭包不能多次获取相同变量的所有权的事实，所以它只能被调用一次； 
  + `FnMut` 获取可变的借用值并且可以改变其环境；
  + `Fn` 从其环境获取不可变的借用值。

+ 创建闭包时，通过闭包对环境值的使用，Rust 推断出具体使用那个 trait：

  + 所有闭包都实现了 `FnOnce`；
  + 没有移动捕获变量的实现了`FnMut`；
  + 无需可变访问捕获变量的实现 了 `Fn`。

+ `move` 关键字：

  + 强制闭包获取其使用的环境值的所有权；

  + 通常用于将闭包传递给新线程以便将数据移动到新线程中；

  + ```rust
    fn main() {
        let x = vec![1, 2, 3];
        let equal_to_x = move |z| z == x;
        // error: value borrowed here after move
        println!("can't use x here: {:?}", x);
        let y = vec![1, 2, 3];
        assert!(equal_to_x(y))
    }
    ```

+ 最佳实践：大部分需要指定一个 `Fn` 系列 trait bound 的时候，可以从 `Fn` 开始，而编译器会根据闭包体中的情况告诉你是否需要 `FnMut` 或 `FnOnce`。



## 迭代器 Iterator

+ 迭代器模式：对一系列项进行某些处理；

+ 迭代器负责遍历每一项和决定遍历何时结束；

+ Rust 中迭代器是惰性的：除非调用消费迭代器的方法，否则迭代器本身没有任何效果；

+ 简单例子：

  ```rust
  fn main() {
      let v1 = vec![1, 2, 3];
      let v1_iter = v1.iter();
  
      for value in v1_iter {
          println!("Got {}", value)
      }
  }
  ```



### Iterator trait & next

+ 所有迭代器都实现了 Iterator trait；

+ Iterator trait 定义于标准库：

  ```rust
  pub trait Iterator {
      type Item;
      fn next(&mut self) -> Option<Self::Item>;
      // 此处省略了方法的默认实现
  }
  ```

  + `type Item` 和 `Self::Item` 定义了 trait 的关联类型；
  + 实现 Iterator trait 需要定义一个 `Item` 类型，这个 `Item` 类型被用作 `next` 方法的返回值类型，也就是迭代器的返回类型。

+ Iterator trait 仅要求实现一个 `next` 方法：

  + 每次返回迭代器中的一项；

  + 返回结果包裹在 Some 中；

  + 迭代结束，返回 None；

  + 可以直接在迭代器上调用 `next` 方法：

    ```rust
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];
        let mut v1_iter = v1.iter();
    
        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }
    ```

+ 常用迭代方法：

  + `iter` 方法：在不可变引用上创建迭代器；
  + `into_iter` 方法：创建的迭代器会获得所有权；
  + `iter_mut` 方法：迭代可变的引用。



### 消耗迭代器

+ 在标准库中，Iterator trait 默认实现了一下方法，其中一些方法会使用到 `next` 方法，因此实现 Iterator trait 时必须实现 `next` 方法；

+ 调用 `next` 的方法叫做**消耗型迭代器**；

  + 因为调用这些方法会把迭代器消耗殆尽。

+ 例如 `sum` 方法，会取得迭代器的所有权，通过反复调用 `next` 方法遍历所有元素。每次迭代，把当前元素添加到一个总和中，迭代结束返回总和：

  ```rust
  #[test]
  fn iterator_sum() {
      let v1 = vec![1, 2, 3];
      let sum: i32 = v1.iter().sum();
  
      assert_eq!(sum, 6);
  }
  ```

+ 例如 `collect` 方法，会将结果收集到指定集合类型中。



### 产生迭代器

+ 定义在 Iterator trait 上的另外一些方法叫做**迭代器适配器**；

  + 把迭代器转换为不同类型的迭代器；
  + 可以通过链式调用使用多个迭代器适配器来完成复杂操作，且可读性高。

+ 例如 `map` 方法，接收一个闭包，闭包作用于每个元素，最后产生一个新的迭代器：

  ```rust
  #[test]
  fn iterator_map() {
      let v1 = vec![1, 2, 3];
      let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
  
      assert_eq!(v2, vec![2, 3, 4]);
  }
  ```



### 使用闭包捕获上下文

+ `filter` 方法：

  + 是迭代器适配器的一种，接收一个闭包；

  + 这个闭包在遍历迭代器的每个元素时返回 bool 类型；

  + 如果闭包返回 ture，当前元素将会包含在 `filter` 产生的迭代器中；

  + 如果闭包返回 false，当前元素将不会包含在 `filter` 产生的迭代器中；

  + ```rust
    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }
    
    fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    }
    
    #[cfg(test)]
    mod tests {
        use super::*;
    
        #[test]
        fn filters_by_size() {
            let shoes = vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker"),
                },
                Shoe {
                    size: 13,
                    style: String::from("sandal"),
                },
                Shoe {
                    size: 10,
                    style: String::from("boot"),
                },
            ];
    
            let in_my_size = shoes_in_size(shoes, 10);
    
            assert_eq!(
                in_my_size,
                vec![
                    Shoe {
                        size: 10,
                        style: String::from("sneaker")
                    },
                    Shoe {
                        size: 10,
                        style: String::from("boot")
                    },
                ]
            );
        }
    }
    ```



### 创建自定义迭代器

+ 实现 `next` 方法；

+ ```rust
  struct Counter {
      count: u32,
  }
  
  impl Counter {
      fn new() -> Counter {
          Counter { count: 0 }
      }
  }
  
  impl Iterator for Counter {
      type Item = u32;
      fn next(&mut self) -> Option<Self::Item> {
          if self.count < 5 {
              self.count += 1;
              Some(self.count)
          } else {
              None
          }
      }
  }
  
  #[cfg(test)]
  mod test2 {
      use super::*;
  
      #[test]
      fn calling_next_directly() {
          let mut couter = Counter::new();
          assert_eq!(couter.next(), Some(1));
          assert_eq!(couter.next(), Some(2));
          assert_eq!(couter.next(), Some(3));
          assert_eq!(couter.next(), Some(4));
          assert_eq!(couter.next(), Some(5));
          assert_eq!(couter.next(), None);
      }
  
      #[test]
      fn using_other_iterator_trait_methods() {
          let sum: u32 = Counter::new()
              .zip(Counter::new().skip(1))
              .map(|(a, b)| a * b)
              .filter(|x| x % 3 == 0)
              .sum();
          assert_eq!(sum, 18);
      }
  }
  ```



### 性能比较

+ 零开销抽象（zero-cost abstractions）：
  + 使用抽象时不会引入额外的性能开销；
  + 迭代器是其中之一。

