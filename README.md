# Link

- [rust](https://www.rust-lang.org/)

- [crate](https://crates.io/)

Video：

- [BiliBili: Rust 编程语言入门教程（Rust 语言/Rust 权威指南配套）](https://www.bilibili.com/video/BV1hp4y1k7SV?spm_id_from=333.337.search-card.all.click)

Book:

- [English](https://doc.rust-lang.org/book/)
- [Chinese](https://kaisery.github.io/trpl-zh-cn/)
- [Demo](https://doc.rust-lang.org/stable/rust-by-example/)

# 基础知识

## 常见编程概念

### 变量和可变性

#### 不可变变量

- 使用 `let` 关键字申明变量；

- 变量默认是不可改变的（immutable），优势：

  - 如果一部分代码假设一个值永远也不会改变，而另一部分代码改变了这个值，第一部分代码就有可能以不可预料的方式运行。这种 bug 的起因难以跟踪，尤其是第二部分代码只是有时会改变值；
  - Rust 编译器保证安全性，如果声明一个值不会变，它就真的不会变，所以你不必自己跟踪它。这意味着你的代码更易于推导。

- 声明变量时不用注明值的类型，会自动推断；

- 例子：

  ```rust
  fn main() {
      let x = 5;
      x = 6;  // error[E0384]: cannot assign twice to immutable variable `x`
  }
  ```

#### 可变变量

- 申明变量前，在变量名之前加 `mut` 来使其可变；

- 重新赋值时需要保证值类型的统一；

- 例子：

  ```rust
  fn main() {
      let mut x = 5;
      x = '6';  // error[E0308]: mismatched types: expected integer, found `char`
  }
  ```

#### 常量

- 使用 `const` 关键字申明常量；

- 在程序生命周期中，常量在其声明的作用域内一直有效；

- 命名规范是：字母全大写，单词之间使用下划线分割；

- 常量与变量的区别：

  - 不允许对常量使用 `mut`，常量永远都是不可变的；
  - 声明常量时必须注明值的类型；
  - 常量可以在任何作用域中声明，包括全局作用域；
  - 常量只能绑定到常量表达式，无法绑定到函数的调用结果或只能在运行时才计算出的值。

- 例子：

  ```rust
  const MAX_POINT: u32 = 99_999;

  fn main() {
      const MIN_POINT: u32 = 1_000;
      println!("the value of MAX_POINT is: {}", MAX_POINT);
      println!("the value of MIN_POINT is: {}", MIN_POINT);
  }
  ```

#### Shadowing

- 可以定义一个与之前变量同名的新变量，新变量会 Shadowing 之前声明的同名变量；

- 与 `mut` 的区别：

  - 如果没有使用 `let` 关键字，重新给非 mut 变量赋值就会导致编译时错误；
  - 当再次使用 `let` 时，实际上创建了一个新变量，可以改变值的类型，并且复用这个名字。

- 优势：

  - 当对变量进行类型转换时，不用重新命名，比如 `spaces_str` 和 `spaces_num` 。

- 劣势：

  - 如果不小心给不同变量起了同样的名字，难道不会引发 bug 吗？

- 例子：

  ```rust
  fn main() {
      let spaces = "    ";
      let spaces = spaces.len();
      println!("the length of spaces is:{}", spaces);

      let x = 10;
      let x = x + 1;

      {
          let x = x * 2;
          println!("The value of x in the inner scope is: {}", x);  // 22
      }

      println!("The value of x is: {}", x);  // 11
  }
  ```
