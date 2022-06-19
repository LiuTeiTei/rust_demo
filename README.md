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

## 数据类型

- Rust 是静态编译语言，在编译时必须知道所有变量的类型；

- 基于使用的值，编译器通常能推断出变量的具体类型；

- 但如果可能的类型比较多，例如把 String 转为 Number 的 parse 方法，就必须人为的添加类型标注，否则编译报错：

  ```rust
  fn main() {
      let guess = "42".parse().expect("not a number");  // error[E0282]: type annotations needed: consider giving `guess` a type
  }
  ```

### 标量类型

- 一个标量类型代表一个单个的值；
- Rust 有四种标量类型：
  - 整型；
  - 浮点型；
  - 布尔型；
  - 字符类型。

#### 整型

没有小数部分的数字

**Rust 中的整型：**

| 长度    | 有符号  | 无符号  |
| ------- | ------- | ------- |
| 8-bit   | `i8`    | `u8`    |
| 16-bit  | `i16`   | `u16`   |
| 32-bit  | `i32`   | `u32`   |
| 64-bit  | `i64`   | `u64`   |
| 128-bit | `i128`  | `u128`  |
| arch    | `isize` | `usize` |

- 无符号整型以 `u` 开头，范围是 [0, 2^n - 1]；
- 有符号整型以 `i` 开头，范围是 [-2^(n - 1), 2^(n-1) - 1]；
- `isize` 和 `usize` 类型的位数由程序运行的计算机架构所决定：

  - 如果是 64 位计算机，相当于 `i64` 和 `u64`；
  - 主要使用场景是：都某种集合进行索引操作。

- 整数的默认类型是 `i32`。

**整数字面量：**

| 数字字面值                    | 例子          |
| ----------------------------- | ------------- |
| Decimal (十进制)              | `98_222`      |
| Hex (十六进制)                | `0xff`        |
| Octal (八进制)                | `0o77`        |
| Binary (二进制)               | `0b1111_0000` |
| Byte (单字节字符)(仅限于`u8`) | `b'A'`        |

- 除了 Byte 类型以外，所有数值字面量值都允许使用类型后缀，例如 57u8；

**整数溢出：**

例如，有一个 `u8` ，它可以存放从 0 到 255 的值，当你将其修改为 256 时：

- debug 模式编译时，Rust 会检查整数溢出，如果发生溢出，程序在运行时会 panic；
- release 模式编译中，Rust 不检测溢出：
  - 如果发生溢出，Rust 会执行二进制补码包装的操作，也就是环绕操作，256 变成 0，257 变成 1，依此类推；
  - 程序不会 panic。

#### 浮点型

含有小数部分的数字

**Rust 中的浮点型：**

- 以 `f` 开头：

  - `f32` ，32 位，单精度；
  - `f64` ，64 位，双精度。

- 所有的浮点型都是有符号的；
- 浮点数的默认类型是 `f64`：
  - Rust 的浮点型使用了 [IEEE-754](https://zh.m.wikipedia.org/zh/IEEE_754) 的标准；
  - 在现代 CPU 中，`f64` 与 `f32` 速度几乎一样，且精度更高。

**数值运算**

- Rust 中的所有数字类型都支持基本数学运算：加法、减法、乘法、除法和取余；
- 整数除法会向下舍入到最接近的整数；
- [附录 B](https://kaisery.github.io/trpl-zh-cn/appendix-02-operators.html)展示了 Rust 提供的所有运算符。

#### 布尔型

- Rust 中的布尔类型使用 `bool` 表示；
- 布尔类型有两个可能的值：true 和 false；
- 一个字节大小。

#### 字符类型

- Rust 中的字符类型使用 `char` 表示，用来描述语言中最基础的单个字符；
- 字面量使用单引号，使用双引号声明字符串字面量；
- 一个字符代表了一个 Unicode 标量值，可以比 ASCII 表示更多内容：
  - 拼音字母，中文、日文、韩文等字符、emoji、零长度的空白字符；
  - 从 U+0000 到 U+D7FF，以及 U+E000 到 U+10FFFF 在内的值。
- 四个字节大小。

### 复合类型

+ 可以将多个值组合成一个类型；
+ Rust 有两个原生的复合类型：
  + 元组（tuple）；
  + 数组（array）。

#### 元组类型

+ 可以将多个类型的多个值放在一个类型里面；
+ 长度固定，一旦声明，其长度不会增大或缩小。

**声明：**

+ 圆括号，值用逗号分开；
+ 元组中的每一个位置都有一个类型，这些不同值的类型也不必是相同的；

**访问：**

+ 使用模式匹配进行解构；
+ 点标记法，后面接元素的索引号；

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("the value of tup is: {}, {}", x, tup.1);
}
```

#### 数组类型

+ 可以将多个值放在一个类型里面，但每个元素的类型必须相同；
+ 长度固定，一旦声明，其长度不会增大或缩小。

**声明：**

+ 声明：中括号，值用逗号分开；
+ 数组的类型用 `[类型;长度]` 这种形式表示；
+ 如果数组中的每个元素值相同，可以通过 `[初始值;长度]` 这种形式声明；

**访问：**

+ 使用索引访问数组的元素；
+ 数组是在堆栈上分配的已知固定大小的单个连续内存块，如果访问的索引超过了数组的长度，那么：
  + 对于简单直接的数组访问编译会报错，稍稍复杂点的编译会通过；
  + 运行会报错，Rust 不允许继续访问相应地址的内存；

**使用场景：**

+ 想要在栈（stack）而不是在堆（heap）上为数据分配空间；
+ 想要确保总是有固定数量的元素；
+ 数组没有 Vector 灵活：
  + Vector 由标准库提供，数组由 prelude 提供；
  + Vector 长度可变；
  + 不确定时用 Vector。

```rust
fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let _a = [1, 2, 3, 4, 5];
    let b = [5; 3];

    println!("the value of array is: {}, {}", a[0], b[2]);
}
```

