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
