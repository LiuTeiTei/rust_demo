# Link

- [rust](https://www.rust-lang.org/)

- [crate](https://crates.io/)

Video：

- [BiliBili: Rust 编程语言入门教程（Rust 语言/Rust 权威指南配套）](https://www.bilibili.com/video/BV1hp4y1k7SV?spm_id_from=333.337.search-card.all.click)

Book:

- [English](https://doc.rust-lang.org/book/)
- [Chinese](https://kaisery.github.io/trpl-zh-cn/)
- [Demo](https://doc.rust-lang.org/stable/rust-by-example/)



# 变量和可变性

## 不可变变量

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



## 可变变量

- 申明变量前，在变量名之前加 `mut` 来使其可变；

- 重新赋值时需要保证值类型的统一；

- 例子：

  ```rust
  fn main() {
      let mut x = 5;
      x = '6';  // error[E0308]: mismatched types: expected integer, found `char`
  }
  ```



## 常量

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



## Shadowing

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



# 数据类型

- Rust 是静态编译语言，在编译时必须知道所有变量的类型；

- 基于使用的值，编译器通常能推断出变量的具体类型；

- 但如果可能的类型比较多，例如把 String 转为 Number 的 parse 方法，就必须人为的添加类型标注，否则编译报错：

  ```rust
  fn main() {
      let guess = "42".parse().expect("not a number");  // error[E0282]: type annotations needed: consider giving `guess` a type
  }
  ```



## 标量类型

- 一个标量类型代表一个单个的值；
- Rust 有四种标量类型：
  - 整型；
  - 浮点型；
  - 布尔型；
  - 字符类型。



### 整型

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



### 浮点型

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



### 布尔型

- Rust 中的布尔类型使用 `bool` 表示；
- 布尔类型有两个可能的值：true 和 false；
- 一个字节大小。



### 字符类型

- Rust 中的字符类型使用 `char` 表示，用来描述语言中最基础的单个字符；
- 字面量使用单引号，使用双引号声明字符串字面量；
- 一个字符代表了一个 Unicode 标量值，可以比 ASCII 表示更多内容：
  - 拼音字母，中文、日文、韩文等字符、emoji、零长度的空白字符；
  - 从 U+0000 到 U+D7FF，以及 U+E000 到 U+10FFFF 在内的值。
- 四个字节大小。



## 复合类型

- 可以将多个值组合成一个类型；
- Rust 有两个原生的复合类型：
  - 元组（tuple）；
  - 数组（array）。



### 元组类型

- 可以将多个类型的多个值放在一个类型里面；
- 长度固定，一旦声明，其长度不会增大或缩小。

**声明：**

- 圆括号，值用逗号分开；
- 元组中的每一个位置都有一个类型，这些不同值的类型也不必是相同的；

**访问：**

- 使用模式匹配进行解构；
- 点标记法，后面接元素的索引号；

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("the value of tup is: {}, {}", x, tup.1);
}
```



### 数组类型

- 可以将多个值放在一个类型里面，但每个元素的类型必须相同；
- 长度固定，一旦声明，其长度不会增大或缩小。

**声明：**

- 声明：中括号，值用逗号分开；
- 数组的类型用 `[类型;长度]` 这种形式表示；
- 如果数组中的每个元素值相同，可以通过 `[初始值;长度]` 这种形式声明；

**访问：**

- 使用索引访问数组的元素；
- 数组是在堆栈上分配的已知固定大小的单个连续内存块，如果访问的索引超过了数组的长度，那么：
  - 对于简单直接的数组访问编译会报错，稍稍复杂点的编译会通过；
  - 运行会报错，Rust 不允许继续访问相应地址的内存；

**使用场景：**

- 想要在栈（stack）而不是在堆（heap）上为数据分配空间；
- 想要确保总是有固定数量的元素；
- 数组没有 Vector 灵活：
  - Vector 由标准库提供，数组由 prelude 提供；
  - Vector 长度可变；
  - 不确定时用 Vector。

```rust
fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let _a = [1, 2, 3, 4, 5];
    let b = [5; 3];

    println!("the value of array is: {}, {}", a[0], b[2]);
}
```



# 函数

- 使用 `fn` 关键字申明函数；

- Rust 代码中的函数和变量名使用 _snake case_ 规范风格：

  - 所有字母都是小写并使用下划线分隔单词；

  - 例如：

    ```rust
    fn another_function() {
        println!("Another function.");
    }
    ```

- 函数可以先调用，后定义，Rust 不关心函数定义于何处，只要定义了就行；

- 函数必须声明每个形参的类型，形参之间用逗号分隔：

  ```rust
  fn print_value(x: i32, y: char) {
      println!("The value of x is: {}", x);
      println!("The value of y is: {}", y);
  }
  ```



## 语句和表达式

- Rust 是一门基于表达式（expression-based）的语言；

- 函数体由一系列的语句和一个可选的结尾表达式构成；

- 语句（Statements）是执行一些操作但不返回值的指令；表达式（Expressions）计算并产生一个值：

  - 语句是执行一些动作的指令；
    - 例如，`let y = 6;` 是一个语句；
    - 函数定义也是语句。
  - 表达式会计算出一个值；
    - `5 + 6` 是一个表达式并计算出值 11；
    - 函数调用是一个表达式；
    - 宏调用是一个表达式；
    - 用大括号创建的一个新的块作用域也是一个表达式；
    - 表达式的结尾没有分号，如果在表达式的结尾加上分号，它就变成了语句，而语句不会返回值。



## 返回值

- 函数可以向调用它的代码返回值；

- 在箭头 `->` 后声明函数返回值的类型，但不可以对返回值命名；

- **在 Rust 中，函数的返回值就是函数体最后一个表达式的值**；

- 若想要提前返回，可以使用使用 `return` 关键字并指定一个值；

- 大部分函数隐式的返回最后的表达式；

- ```rust
  fn plus_five(x: i32) -> i32 {
      // NOTE: there is no ';'
      x + 5
  }
  ```

# 控制流

## if 表达式

- `if` 表达式允许根据条件执行不同的代码分支；

- 与 JavaScript 的 `if` 表达式不同点在于：

  - `if` 后跟的条件不需要用括号括起来；

  - Rust 不会尝试自动地将非布尔值转换为布尔值，必须总是显式地使用布尔值作为 `if` 的条件；

  - 例如：

    ```rust
    fn main() {
        let number = 3;
    
        if number < 5 {
            println!("condition was true");
        } else {
            println!("condition was false");
        }
    }
    ```

- 可以将 `else if` 表达式与 `if` 和 `else` 组合来实现多重条件：

  ```rust
  fn main() {
      let number = 6;

      if number % 4 == 0 {
          println!("number is divisible by 4");
      } else if number % 3 == 0 {
          println!("number is divisible by 3");
      } else if number % 2 == 0 {
          println!("number is divisible by 2");
      } else {
          println!("number is not divisible by 4, 3, or 2");
      }
  }
  ```

  - Rust 只会执行第一个条件为真的代码块，一旦它找到一个以后，不会检查剩下的条件；
  - 如果使用了多于一个 else if 语句，建议使用 match；

- 因为 `if` 是一个表达式，我们可以在 `let` 语句的右侧使用它：

  ```rust
  fn main() {
      let condition = true;
      let number = if condition { 5 } else { "six" };
      println!("The value of number is: {}", number);  // 5
  
      // `if` and `else` have incompatible types: expected integer, found `char` rustc(E0308)
      let error = if condition { 5 } else { '6' };
  }
  ```

  - `number` 变量将会绑定到表示 `if` 表达式结果的值上；
  - 代码块的值是其最后一个表达式的值，而数字本身就是一个表达式；
  - Rust 是强类型语言，在编译的时候就必须要知道变量的类型，所以这种写法时 if 后和 else 里面返回值的类型必须保持一致，否则会报错。每个可能成为结果的分支的返回值的类型必须是一样的



## 循环

### loop

- `loop` 关键字告诉 Rust 一遍又一遍地执行一段代码直到你明确要求停止；

- 如果存在嵌套循环，`break` 和 `continue` 应用于此时最内层的循环：

  - 循环中的 `break` 关键字来告诉程序何时停止循环；

  - 循环中的 `continue` 关键字告诉程序跳过这个循环迭代中的任何剩余代码，并转到下一个迭代；

  - 可以选择在一个循环上指定一个循环标签，loop label，然后将标签与 `break` 或 `continue` 一起使用，使这些关键字应用于已标记的循环而不是最内层的循环：

    ```rust
    fn main() {
        let mut count = 0;
    
        'counting_up: loop {
            println!("count = {}", count);
            let mut remaining = 10;
    
            loop {
                println!("remaining = {}", remaining);
                if remaining == 9 {
                    break;
                }
                if count == 2 {
                    break 'counting_up;
                }
    
                remaining -= 1;
            }
    
            count += 1;
        }
    
        println!("End count = {}", count)
    }
    ```

- loop 可以有返回值：

  ```rust
  fn main() {
      let mut counter = 0;
  
      let result = loop {
          counter += 1;
  
          if counter == 10 {
              break counter * 2;
          }
      };
  
      println!("The result is {}", result);  // 20
  }
  ```



### while

- 每次执行循环体之前判断一次条件；

- 这个循环类型可以通过组合 `loop`、`if`、`else` 和 `break` 来实现；

- 例子：

  ```rust
  fn main() {
      let mut number = 3;
  
      while number != 0 {
          println!("{}!", number);
          number -= 1;
      }
  
      println!("LIFTOFF!!!");
  }
  ```



### for

- `while` 循环容易出错且低效；

  - `while` 循环有时使程序更慢，因为编译器增加了运行时代码来对每次循环进行条件检查。

- 可以使用 `for` 循环来对一个集合的每个元素执行一些代码：

  ```rust
  fn main() {
      let a = [10, 20, 30, 40, 50];
      let mut index = 0;

      while index < 5 {
          println!("the value is: {}", a[index]);

          index += 1;
      }
  }

  // 用 for 重写 while
  fn main() {
      let a = [10, 20, 30, 40, 50];

      for element in a {
          println!("the value is: {}", element);
      }
  }
  ```

- 大部分 Rustacean 也会使用 `for` 循环，例如 `while` 中发射的例子：

  ```rust
  fn main() {
      for number in (1..4).rev() {
          println!("{}!", number);
      }
      println!("LIFTOFF!!!");
  }
  ```

  - `Range`是标准库提供的类型，用来生成从一个数字开始到另一个数字之前结束的所有数字的序列。



# 注释

- 一般注释：

  - 仅做注释用，在编译时编译器会忽略它们；

  - 分为行注释和块注释；

  - 例如：

    ```rust
    // 这是行注释的例子
    
    /*
        这是块注释的例子
    */
    
    /*
     * 这是另外一种注释——块注释。一般而言，行注释是推荐的注释格式，
     * 不过块注释在临时注释大块代码特别有用。/* 块注释可以 /* 嵌套, */ */
     */
    ```

- 文档注释：

  - 可以通过 `cargo doc` 命令生成 HTML 帮助文档；

  - 分为内部文档注释和外部文档注释；

  - 内部文档注释（Inner doc comment）：

    - 单行注释（以 `///` 开头）；
    - 块注释（用 `/** ... */` 分隔）；
    - 内部文档注释是对它**之后**的项做注释，与使用 `#[doc="..."]` 是等价的。

  - 外部文档注释（Outer doc comment）：

    - 单行注释（以 `//!` 开头）；
    - 块注释（用 `/*! ... */` 分隔）；
    - 内部文档注释是对它**之后**的项做注释，与使用 `#[doc="..."]` 是等价的。



# 所有权

+ 所有权系统是 Rust 最为与众不同的特性，对语言的其他部分有着深刻含义；
+ 它让 Rust 无需垃圾回收（garbage collector）即可保障内存安全；
+ 所有程序都必须管理其运行时使用计算机内存的方式：
  + 一些语言中具有垃圾回收机制，在程序运行时不断地寻找不再使用的内存；
  + 一些语言中，程序员必须亲自分配和释放内存；
  + Rust 选择了第三种方式，通过所有权系统管理内存：
    + Rust 把内存管理相关的工作都提前到了编译时；
    + 编译器在编译时会根据一系列的规则进行检查；
    + 如果违反了任何这些规则，程序都不能编译；
    + 在运行时，所有权系统的任何功能都不会减慢程序。



**所有权的主要目的：** 

+ 所有权的主要目的就是为了管理 Heap 数据：
  + 跟踪哪部分代码正在使用 Heap 上的哪些数据；
  + 最大限度的减少 Heap 上的重复数据的数量；
  + 清理 Heap 上不再使用的数据确保不会耗尽空间；
+ 一旦理解了所有权，就不需要经常考虑 Stack 和 Heap 了；



## 栈（Stack）vs 堆（Heap）

在像 Rust 这样的系统编程语言中，值是位于 Stack 上还是 Heap 上在更大程度上影响了语言的行为，以及为何必须做出这样的抉择；



### 存储数据

栈和堆都是代码在运行时可供使用的内存，但是它们的结构不同，导致存储数据的方式也不同。

Stack：

+ Stack 以放入值的顺序存储值并以相反顺序取出值，也就是先进后出；
+ 增加数据叫做 **进栈**，移出数据叫做 **出栈**；
+ **栈中的所有数据都必须占用已知且固定的大小**；
+ 在编译时大小未知或大小可能变化的数据，要改为存储在堆上。

Heap：

+ Heap 组织性差一些；
+ **当向堆放入数据时，需要请求一定大小的空间，内存分配器在堆的某处找到一块足够大的空位**，把它标记为已使用，并返回一个表示该位置地址的指针；
+ 这个过程称作“在堆上分配内存”，简称为 **分配**；
+ 将数据推入栈中并不被认为是分配；
+ 指针的大小是已知并且固定的，因此可以将指针存储在栈上，不过当需要实际数据时，必须访问指针。

差异：

+ 入栈比在堆上分配内存要快：
  + 因为入栈时分配器无需为存储新数据去搜索内存空间；
  + 其位置总是在栈顶。
+ 在堆上分配内存则需要更多的工作：
  + 因为分配器必须首先找到一块足够存放数据的内存空间，并接着做一些记录为下一次分配做准备。



### 访问数据

+ 访问 Heap 上的数据比访问 stack 上的数据慢：
  + 因为必须通过指针来访问；
  + 由于缓存机制，现代处理器在内存中跳转越少就越快；
+ 处理器在处理的数据彼此较近的时候（比如在 Stack 上）比较远的时候（比如可能在 Heap 上）能更好的工作；
+ 在 Heap 上分配大量的空间也会耗时。



### 函数调用

+ 当调用一个函数时，传递给函数的值（包括可能指向 Heap 上数据的指针）和函数的局部变量被压入 Stack 中；
+ 当函数结束时，这些值被弹出 Stack。



## 所有权规则

+ Rust 中的每一个值都有一个被称为其 **所有者 - owner** 的变量；
+ 值在任一时刻有且只有一个所有者；
+ 当所有者（变量）离开作用域，这个值将被丢弃。



### 作用域

+ 当变量 **进入作用域** 时，它就是有效的；

+ 这一直持续到它 **离开作用域** 为止；

+ 作用域的关系跟其他编程语言是类似的：

  ```rust
  fn main() {
      // s 在这里无效, 它尚未声明
      let s = "hello"; // 从此处起，s 是有效的
  
      // 使用 s
  } // 此作用域已结束，s 不再有效
  ```



### String 类型

+ 标量类型都是已知大小的，可以存储在 Stack 中，并且当离开作用域时被移出 Stack；

+ 如果代码的另一部分需要在不同的作用域中使用相同的值，可以快速简单地 Copy 它们来创建一个新的独立实例；

+ 字符串字面值是不可变的，被硬编码进程序里，通常存放在 Stack 中；

+ 但并非所有字符串的值都能在编写代码时就知道，例如获取用户输入并存储，为此，Rust 有第二个字符串类型，`String`；

+ 这个类型管理在 Heap 上分配，能够存储在编译时未知大小的文本；

+ 可以使用 `from` 函数基于字符串字面值来创建 `String`：

  ```rust
  fn main() {
      let mut s = String::from("hello");
      s.push(',');
      s.push_str(" word!");
      println!("{}", s)
  }
  ```



### 内存与分配

为什么 `String` 可变而字面值却不行呢？区别在于两个类型对内存的处理上。

字符串字面值：

+ 在编译时就知道其内容，所以文本被直接硬编码进最终的可执行文件中；
+ 这使得字符串字面值快速且高效，不过这些特性都只得益于字符串字面值的不可变性。

 `String` 类型：

+ 为了支持一个可变、可增长的文本片段，需要在 Heap 上分配一块在编译时未知大小的内存来存放内容；
+ 因此：
  - 必须在运行时向内存分配器请求内存，例如调用 `String::from()` 方法；
  - 当我们处理完 `String` 时，需要一个将内存返回给分配器的方法：
    - 在有 GC（垃圾回收）的语言中，GC 会自动记录并清除不再使用的内存；
    - 在没有 GC 的语言中，需要使用者主动识别出不再使用的内存并调用代码显式释放。如果忘记回收了会浪费内存、如果过早回收了会出现无效变量、如果重复回收会造成 bug；
    - Rust 采取了一个不同的策略，内存在拥有它的变量离开作用域后就被自动释放。当变量离开作用域，Rust 为我们调用一个特殊的 `drop` 函数。



## 变量与数据的交互方式

在Rust 中，多个变量可以采取不同的方式与同一数据进行交互。



### Move

当作用于整型这类简单数据类型时：

```rust
let x = 5;
let y = x;
```

+ 因为整数是有已知固定大小的简单值，所以这两个 5 被放入了 Stack 中；



当作用于 `String` 这类复杂数据类型时：

```rust
let s1 = String::from("string move");
let s2 = s1;
```

+ `String` 由三部分组成：

  + <img src='images/String-memory.svg' alt='String-memory' width=300 height=300 align='left'/>
  + 一个指向存放字符串内容内存的指针，一个长度，和一个容量；
  + 这一组数据存储在 Stack 上；
  + 字符串内容内存则存放在 Heap 上；

+ 当将 `s1` 赋值给 `s2`，`String` 的数据被复制了，从 Stack 上拷贝了它的指针、长度和容量，但没有复制指针指向的 Heap 上数据（如果 Rust 这么做了，那么操作 `s2 = s1` 在 Heap 上数据比较大的时候会对运行时性能造成非常大的影响）；

+ 当变量离开作用域后，Rust 自动调用 `drop` 函数并清理变量的 Heap 内存。这就有了一个问题：当 `s2` 和 `s1` 离开作用域，他们都会尝试释放相同的内存。这是一个叫做 **二次释放** 的错误，会导致内存污染，也可能会导致潜在的安全漏洞；

+ 因此为了确保内存安全，在 `let s2 = s1` 之后，Rust 认为 `s1` 不再有效，因此 Rust 不需要在 `s1` 离开作用域后清理任何东西：

  ```rust
  // error[E0382]: borrow of moved value: `s1`
  println!("s1: {}", s1);
  ```

+ 其他语言中可能有 **浅拷贝** 和 **深拷贝**，那么拷贝指针、长度和容量而不拷贝数据可能听起来像浅拷贝，不过因为 Rust 同时使第一个变量无效了，这个操作被称为 **Move**，而不是浅拷贝；

+ 上面的例子可以解读为 `s1` 被 Move 到了 `s2` 中，只有 `s2` 是有效的，当其离开作用域，它就释放自己的内存；

+ Rust 永远也不会自动创建数据的深拷贝，因此，任何自动的复制可以被认为对运行时性能影响较小。



### Clone

如果我们确实需要深度复制 `String` 中 Heap 上的数据，而不仅仅是 Stack上的数据，可以使用一个叫做 `clone` 的通用函数：

```rust
let s11 = String::from("string clone");
let s22 = s11.clone();
println!("s11: {}, s22: {}", s11, s22);
```

+ 当出现 `clone` 调用时，一些特定的代码被执行，而且这些代码可能相当消耗资源。



### Copy

Copy 其实是只在 Stack 上数据的 Move：

```rust
let x = 5;
let y = x;
```

+ 虽然没有调用 `clone`，不过 `x` 依然有效且没有被 Move 到 `y` 中；
+ 因为像整型这样的在编译时已知大小的类型被整个存储在 Stack 上，所以拷贝其实际的值是快速的，这意味着没有理由在创建变量 `y` 后使 `x` 无效；



Copy trait

+ Rust 有一个叫做 `Copy trait` 的特殊注解，可以用在类似整型这样的存储在 Stack 上的类型上；
+ 如果一个类型实现了 `Copy trait` ，那么一个旧的变量在将其赋值给其他变量后仍然可用；
+ Rust 不允许自身或其任何部分实现了 `Drop trait` 的类型使用 `Copy trait`；
+ 任何一组简单标量值的组合都可以实现 Copy，任何不需要分配内存或某种形式资源的类型都可以实现 Copy：
  + 所有整数类型；
  + 布尔类型；
  + 所有浮点数类型；
  + 字符类型；
  + 元组，当且仅当其包含的类型也都实现 `Copy` 的时候。



## 所有权与函数

### 参数

+ 将值传递给函数在语义上与给变量赋值相似；
+ 向函数传递值可能会移动或者复制，就像赋值语句一样。



函数参数如何进入和离开作用域：

```rust
fn main() {
    let s = String::from("hello");
    takes_ownership(s);
    // println!("s is invalidate: {}", s);  // error[E0382]: borrow of moved value: `s`

    let x = 5;
    makes_copy(x);
    println!("x still validate: {}", x);
}

fn takes_ownership(some_string: String) {
    println!("takes_ownership: {}", some_string)
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
```

+ 因为 `s` 是个 String 类型，传入函数是 Move，在调用 `takes_ownership` 后使用 `s` 时，Rust 会抛出一个编译时错误：

  ```rust
  // error[E0382]: borrow of moved value: `s`
  println!("s is invalidate: {}", s);
  ```

+ 因为 `x` 是一标量类型，传入函数是 Copy，在调用 `makes_copy` 后可以正常使用 `x` ：

  ```rust
  println!("x still validate: {}", x);
  ```

+ 在 takes_ownership 函数中：

  + 函数体的花括号开始处，some_string 进入作用域；
  + 函数体的花括号结束处，some_string 移出作用域并调用 `drop` 方法，占用的内存被释放。

+ 在 makes_copy 函数中：

  + 函数体的花括号开始处，some_integer 进入作用域；
  + 函数体的花括号结束处，some_string 移出作用域，没有其他特殊操作。



### 返回值

+ 返回值也可以转移所有权。



函数的返回值如何移交所有权：

```rust
fn main() {
    let s1 = gives_ownership();

    let s2: String = String::from("hello");
    let s3 = takes_and_gives_back(s2);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
```

+ 在 gives_ownership 函数中，会将返回值 Move 到调用它的函数；

+ 在 takes_and_gives_back 函数中，取得传入变量的所有权，并 Move 到调用它的函数；

+  `s2` 的所有权通过函数已经 Move 到 s3 上，所以调用函数后使用 `s2` 时，Rust 会抛出一个编译时错误：

  ```rust
  // error[E0382]: borrow of moved value: `s2`
  println!("s2 is invalidate: {}", s2);  
  ```



## 引用

+ 如果想要函数使用一个值但不获取所有权，可以使用**引用 - reference**；

+ 引用类似指针，它是一个地址，可以由此访问储存于该地址的属于其他变量的数据；

+ 与指针不同的是，引用确保指向某个特定类型的有效值；

+ 使用 `&` 符号表明引用，允许使用值但不获取其所有权：

  ```rust
  fn main() {
      let s = String::from("hello");
      let len = calculate_len(&s);
      println!("The length of {} is {}", s, len);
  }
  
  fn calculate_len(some_string: &String) -> usize {
      some_string.len()
  }
  ```

  + `&s` 语法让我们创建一个指向值 `s` 的引用，但是并不拥有它；
  + 因为并不拥有这个值的所用权，所以当引用停止使用时，它所指向的值也不会被丢弃；
  + 例如，在 calculate_len 函数中，函数体的花括号结束处，s 移出作用域但其所指向的值不会被清理。

+ 我们将创建一个引用的行为称为**借用 - borrowing**。



### 不可变引用

+ 无法修改借用的变量；

+ 例如：

  ```rust
  fn changeError(some_string: &String) {
      // error[E0596]: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
      some_string.push_str(", world");
  }
  ```

+ 正如变量默认是不可变的，引用也一样，默认不允许修改引用的值。



### 可变引用

+ 通过添加 `mut` 来创建可变引用；

+ 例如：

  ```rust
  fn main() {
      let mut variable_s = String::from("hello");
      change(&mut variable_s);
      println!("The changed string is {}", variable_s);
  }
  
  fn change(some_string: &mut String) {
      some_string.push_str(", world");
  }
  ```

  + 声明变量时将 `variable_s` 改为 `mut`；
  + 调用 `change` 函数的地方创建一个可变引用 `&mut variable_s`；
  + 函数签名改为接受一个可变引用 `some_string: &mut String`。

+ 限制：

  + 在同一时间只能有一个对某一特定数据的可变引用：

    ```rust
    fn main() {
        let mut s = String::from("hello");
        let r1 = &mut s;
        // error[E0499]: cannot borrow `s` as mutable more than once at a time
        let r2 = &mut s;
        println!("{}, {}", r1, r2);
    }
    ```

    + 这个限制的好处是 Rust 可以在编译时就避免数据竞争；
    + 数据竞争类似于竞态条件，它可由这三个行为造成：
      - 两个或更多指针同时访问同一数据；
      - 至少有一个指针被用来写入数据；
      - 没有同步数据访问的机制。

  + 可以通过创建新的作用域，来允许非同时的创建多个可变引用：

    ```rust
    fn main() {
        let mut s = String::from("hello");
        {
            let r1 = &mut s;
            println!("{}", r1);
        }
        let r2 = &mut s;
        println!("{}", r2);
    }
    ```

  + 不能同时拥有一个可变引用和一个不可变引用：

    ```rust
    fn main() {
        let mut s = String::from("hello");
        let r1 = &s;
        let r2 = &s;
        // error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
        let r3 = &mut s;
        println!("{}, {}, and {}", r1, r2, r3);
    }
    ```

  + 一个引用的作用域从声明的地方开始一直持续到最后一次使用为止：

    ```rust
    fn main() {
        let mut s = String::from("hello");
        let r1 = &s;
        let r2 = &s;
        println!("{} and {}", r1, r2); // 此位置之后 r1 和 r2 不再使用，因此可以再成功声明一个 r3
        let r3 = &mut s;
        println!("{}", r3);
    }
    ```

    + 因为最后一次使用不可变引用（`println!`)，发生在声明可变引用之前，所以以上代码是可以编译通过的；
    + 不可变引用 `r1` 和 `r2` 的作用域在 `println!` 最后一次使用之后结束，这也是创建可变引用 `r3` 的地方，它们的作用域没有重叠，所以代码是可以编译的。



### 悬垂引用

+ 在具有指针的语言中，很容易通过释放内存时保留指向它的指针而错误地生成一个悬垂指针；

+ 所谓悬垂指针是其指向的内存可能已经被分配给其它持有者；

+ 在 Rust 中编译器确保引用永远也不会变成悬垂状态，当你拥有一些数据的引用，编译器确保数据不会在其引用之前离开作用域；

+ 当我们尝试创建一个悬垂引用时，Rust 会通过一个编译时错误来避免：

  ```rust
  // error[E0106]: missing lifetime specifier
  fn dangle() -> &String {
      let s = String::from("hello");
      &s
  }
  ```

+ 解决：

  ```rust
  fn no_dangle() -> String {
      let s = String::from("hello");
      s
  }
  ```

  + 所有权被移动出去，值没有被释放；



## Slice 类型

+ slice 允许你引用集合中一段连续的元素序列，而不用引用整个集合；
+ slice 是一类引用，所以它没有所有权。



一个有 bug 的例子：

```rust
fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    s.clear();
    println!("{}", word); // 5
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
```

+ 清空了字符串 `s`，但 `word` 依旧是5，毫无意义；
+ 因为 `first_word` 返回了一个独立的 `usize`，它只在 `&String` 的上下文中才是一个有意义的数字。因为它是一个与 `String` 相分离的值，无法保证将来它仍然有效；
+ 这个程序编译时没有任何错误，而且在调用 `s.clear()` 之后使用 `word` 也不会出错。因为 `word` 与 `s` 状态完全没有联系，所以 `word `仍然包含值 `5`；
+ 我们不得不时刻担心 `word` 的索引与 `s` 中的数据不再同步。



### 字符串 Slice

+ 基础语法：

  ```rust
  &String[starting_index..ending_index]
  ```

  + 其中 `starting_index` 是 slice 的第一个位置；

  + `ending_index` 则是 slice 最后一个位置的后一个值；

  + 在其内部，slice 的数据结构存储了 slice 的开始位置和长度，长度对应于 `ending_index` 减去 `starting_index` 的值；

  + 例如

    ```rust
    let s = String::from("hello world");
    
    let hello = &s[0..5];
    let world = &s[6..11];
    ```

+ 语法糖：

  ```rust
  // 等价
  let hello = &s[0..5];
  let hello = &s[..2];
  
  let world = &s[6..11];
  let world = &s[3..s.len()];
  let world = &s[3..];
  
  let whole = &s[..];
  ```

+ 改写 bug 例子：

  ```rust
  fn main() {
      let mut s = String::from("hello world");
      let word = first_word(&s);
      // error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
      s.clear();
      println!("{}", word);
  }
  
  fn first_word(s: &String) -> &str {
      let bytes = s.as_bytes();
      for (i, &item) in bytes.iter().enumerate() {
          if item == b' ' {
              return &s[..i];
          }
      }
  
      &s[..]
  }
  ```

  + 编译时有报错提示了。



**字符串字面值就是 slice：**

  ```rust
  let s = "Hello, world!";
  ```

  + 这里 s 的类型就是 `&str`，是一个指向二进制程序特定位置的 slice；
  + 因为 `&str` 是一个不可变引用，所以字符串字面值是不可变的。



**字符串 slice 作为参数：**

```rust
fn first_word(s: &String) -> &str { ... }
// 改为
fn first_word(s: &str) -> &str { ... }
```

+ 如果有一个字符串 slice，可以直接传递它；
+ 如果有一个 `String`，则可以传递整个 `String` 的 slice 或对 `String` 的引用;
+ 定义一个获取字符串 slice 而不是 `String` 引用的函数，使得我们的 API 更加通用并且不会丢失任何功能。



### 数组 Slice

```rust
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];
```

+ 跟字符串 slice 的工作方式一样，通过存储第一个集合元素的引用和一个集合总长度。



# struct 结构体

+ struct 是自定义的数据类型，为相关联的值命名，打包成有意义的组合；
+ 和元组一样，结构体的每一部分可以是不同类型；
+ 不同于元组，结构体需要命名各部分数据以便能清楚的表明其值的意义。



## 定义与实例化

**定义：**

+ 使用 `struct` 关键字并为整个结构体提供一个名字；

+ 结构体的名字需要描述它所组合的数据的意义；

+ 在大括号中，定义每一部分数据的名字和类型；

+ 例子：

  ```rust
  struct Use {
      active: bool,
      username: String,
      email: String,
      sign_in_count: u64,
  }
  ```



**实例化：**

+ 创建一个实例需要以结构体的名字开头；

+ 接着在大括号中使用 `key: value` 键-值对的形式提供字段，其中 key 是字段的名字，value 是需要存储在字段中的数据值；

+ 实例中字段的顺序不需要和它们在结构体中声明的顺序一致；

+ 一旦 struct 的实例是可变的，实例中所有的字段都是可变的，Rust 并不允许只将某个字段标记为可变；

+ 使用点标记法取值和赋值；

+ 例子：

  ```rust
  fn main() {
      let mut user1 = User {
          email: String::from("user1@example.com"),
          username: String::from("user1username"),
          active: true,
          sign_in_count: 1,
      };
  
      let email = user1.email;
      user1.email = String::from("user1anothere@example.com");
      println!(
          "user1 prev email:{}, user1 cur email:{}",
          email, user1.email
      );
  }
  ```



**作为函数返回值：**

+ 可以在函数体的最后一个表达式中构造一个结构体的新实例，来隐式地返回这个实例；

+ 当参数名与字段名完全相同时，可以使用字段初始化简写语法；

+ 例子：

  ```rust
  fn build_user(email: String, username: String) -> User {
      User {
          active: true,
          username,
          email,
          sign_in_count: 1,
      }
  }
  
  fn main() {
      let user2 = build_user(
          String::from("user2@example.com"),
          String::from("user2username"),
      );
      println!("user2 email:{}", user2.email);
  }
  ```



**更新语法：**

+ 使用旧实例的大部分值，但改变其部分值来创建一个新的结构体实例；

+ `..` 语法指定了剩余未显式设置值的字段应有与给定实例对应字段相同的值；

+ 例子：

  ```rust
  fn main() {
      let user2 = User {
          active: user1.active,
          username: user1.username,
          email: String::from("user2@example.com"),
          sign_in_count: user1.sign_in_count,
      };
    
      // 等价于
      let user2 = User {
          email: String::from("user2@example.com"),
          ..user1
      };
  }
  ```

  + 与 JS 语法不同，`..user1` 必须放在最后，以指定其余的字段应从 `user1` 的相应字段中获取其值。
  
+ 结构更新语法就像带有 `=` 的赋值，因为它移动了数据；
  
  + 例如：
  
    ```rust
    println!("user1 email:{}", user1.email);
    
    // error[E0382]: borrow of moved value: `user1.username`
    println!("user1 username:{}", user1.username);
    ```
  
    + 在这个例子中，在创建 `user2` 后不能再使用 `user1.username`，因为 `user1` 的 `username` 字段中的 `String` 被移到 `user2` 中；
    + 如果我们给 `user2` 的 `email` 和 `username` 都赋予新的 `String` 值，从而只使用 `user1` 的 `active` 和 `sign_in_count` 值，那么 `user1` 在创建 `user2` 后仍然有效，`active` 和 `sign_in_count` 的类型是实现 `Copy trait` 的类型。



## tuple struct

+ 给整个 tuple 取一个名字，并使其成为与其他 tuple 不同的类型；

+ 例子：

  ```rust
  struct Color(i32, i32, i32);
  struct Point(i32, i32, i32);
  
  fn main() {
      let black = Color(0, 0, 0);
      let origin = Point(0, 0, 0);
    
      println!("color[0]: {}", color.0);
      println!("point[0]: {}", point.0);
  }
  ```

  + 注意 `black` 和 `origin` 值的类型不同，因为它们是不同的元组结构体的实例；
  + 你定义的每一个结构体有其自己的类型，即使结构体中的字段有着相同的类型；

+ tuple struct 实例类似于 tuple，可以将其解构为单独的部分，也可以使用 `.` 后跟索引来访问单独的值。



## Unit-Like struct

+ 可以定义一个没有任何字段的 struct；

+ 想要在某个类型上实现 trait 但不需要在类型中存储数据；

+ 例子：

  ```rust
  struct AlwaysEqual;
  
  fn main() {
      let subject = AlwaysEqual;
  }
  ```



## 定义结构体方法

+ 方法与函数类似：
  + 它们使用 `fn` 关键字和名称声明；
  + 可以拥有参数和返回值；
  + 同时包含在某处调用该方法时会执行的代码。
+ 与函数是不同的：
  + 方法在 struct、enum、trait 的上下文中被定义；
  + 方法的第一个参数总是 `self`，表示调用该方法的结构体实例。



### 定义

+ 在 `impl` 块中定义方法，将函数定义于 strct 的上下文中，这个 `impl` 块中的所有内容都将与 strct 类型相关联；

+ `&self` 实际上是 `self: &Self` 的缩写；

  + 在一个 `impl` 块中，`Self` 类型是 `impl` 块的类型的别名；
  + 方法的第一个参数必须有一个名为 `self` 的`Self` 类型的参数，所以 Rust 让你在第一个参数位置上只用 `self` 这个名字来缩写；
  + 方法可以选择获得 `self` 的所有权，或者不可变地借用 `self`，或者可变地借用 `self`，就跟其他参数一样。

+ 例子：

  ```rust
  #[derive(Debug)]
  struct Rectangle {
      width: u32,
      height: u32,
  }
  
  impl Rectangle {
      fn area(&self) -> u32 {
          self.width * self.height
      }
  }
  
  fn main() {
      let rect1 = Rectangle {
          width: 30,
          height: 50,
      };
  
      println!(
          "The area of the rectangle is {} square pixels.",
          rect1.area()
      );
  }
  ```

+ 除了 `self`，方法可以定义其他参数：

  ```rust
  impl Rectangle {
      fn can_hold(&self, other: &Rectangle) -> bool {
          self.width > other.width && self.height > other.height
      }
  }
  ```

  

### 关联函数

+ 所有在 `impl` 块中定义的函数被称为 **关联函数**，因为它们与 `impl` 后面命名的类型相关；

+ 可以定义不以 `self` 为第一参数的关联函数，因为它们并不作用于一个结构体的实例，因此不是方法；

+ 例如在 `String` 类型上定义的 `String::from` 函数；

+ 不是方法的关联函数经常被用作返回一个结构体新实例的构造函数：

  ```rust
  impl Rectangle {
      fn square(size: u32) -> Rectangle {
          Rectangle {
              width: size,
              height: size,
          }
      }
  }
  
  fn main {
    let s = Rectangle::square(16);
  }
  ```

+ 每个结构体都允许拥有多个 `impl` 块。



# enums 枚举

+ 枚举允许你通过列举可能的值来定义一个类型；
+ 枚举是一个不同于结构体的**定义自定义数据类型**的方式；



## 定义

+ 定义：

  ```rust
  enum IpAddrKind {
      V4,
      V6,
  }
  ```
  
+ 取值：
  
  ```rust
  let four = IpAddrKind::V4;
  let six = IpAddrKind::V6;
  ```
  
  + 枚举的成员位于其标识符的命名空间中，并使用 `::` 分开；
  
+ 和结构体结合使用：

  ```rust
  fn main() {
      struct IpAddr {
          kind: IpAddrKind,
          address: String,
      }
  
      let home = IpAddr {
          kind: IpAddrKind::V4,
          address: String::from("127.0.0.1"),
      };
      let loopback = IpAddr {
          kind: IpAddrKind::V6,
          address: String::from("::1"),
      };
  }
  ```

+ 有一种更简洁的方式来表达相同的概念：

  ```rust
  enum IpAddr {
      V4(String),
      V6(String),
  }
  
  let home = IpAddr::V4(String::from("127.0.0.1"));
  let loopback = IpAddr::V6(String::from("::1"));
  ```

  + 仅仅使用枚举并将数据直接放进每一个枚举成员，而不是将枚举作为结构体的一部分，这样就不需要一个额外的结构体了；

  + 每一个我们定义的枚举成员的名字也变成了一个构建枚举的实例的函数；

  + 可以将任意类型的数据放入枚举成员中：例如字符串、数字类型、结构体，甚至可以包含另一个枚举：

    ```rust
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    ```

  + 可以参考官方的标准库是如何定义 `IpAddr` 的：[std::net::IpAddr](https://doc.rust-lang.org/std/net/enum.IpAddr.html) 

+ 关联值的枚举的方式和定义多个不同类型的结构体的方式很相像：

  ```rust
  enum Message {
      Quit,
      Move { x: i32, y: i32 },
      Write(String),
      ChangeColor(i32, i32, i32),
  }
  
  /*  类似的  */
  
  struct QuitMessage; // 类单元结构体
  struct MoveMessage {
      x: i32,
      y: i32,
  }
  struct WriteMessage(String); // 元组结构体
  struct ChangeColorMessage(i32, i32, i32); // 元组结构体
  
  ```

  + 使用枚举可以将被组合在一起位于 `Message` 类型下。

+ 与结构体类似，枚举可以使用 `impl` 来定义方法：

  ```rust
  fn main() {
      impl Message {
          fn call(&self) {
              // ...
          }
      }
  
      let m = Message::Write(String::from("hello"));
      m.call();
  }
  ```



## Option

+ Rust 并没有很多其他语言中有的空值功能，Null，但 Rust 有一个可以编码存在或不存在概念的枚举，这个枚举是 `Option<T>`；

+ `Option` 是[标准库](https://doc.rust-lang.org/std/option/enum.Option.html)定义的另一个枚举，表示一个值要么有值要么没值：

  ```rust
  // 官方定义
  enum Option<T> {
      None,
      Some(T),
  }
  
  // 例子
  fn main() {
      let some_number = Some(5);
      let absent_number: Option<i32> = None;
  }
  ```

  + `Option<T>` 枚举是被包含在了 prelude 之中，不需要将其显式引入作用域；
  + 其成员不需要 `Option::` 前缀，可以直接使用 `Some` 和 `None`。

+  `Option<T>` 和 `T`是不同的类型：

  ```rust
  // error[E0277]: cannot add `Option<i8>` to `i8`
  let x: i8 = 8;
  let y: Option<i8> = Some(8);
  let sum = x + y;
  ```

  + 当在 Rust 中拥有一个像 `i8` 这样类型的值时，编译器确保它总是有一个有效的值，可以直接使用而无需做空值检查；
  + 当使用 `Option<i8>` 的时候需要担心可能没有值，编译器会确保我们在使用值之前处理了为空的情况；
  + 在对 `Option<T>` 进行 `T` 的运算之前必须将其转换为 `T`。

+ 为了拥有一个可能为空的值，必须要显式的将其放入对应类型的 `Option<T>` 中。当使用这个值时，必须明确的处理值为空的情况，限制空值的泛滥以增加 Rust 代码的安全性；

+ 为了使用 `Option<T>` 值，需要编写处理每个成员的代码，`match` 表达式可以做到。



## match

+ `match` 表达式是一个处理枚举的控制流结构，它会根据枚举的成员运行不同的代码，这些代码可以使用匹配到的值中的数据；

+ `match` 可由字面值、变量、通配符和许多其他内容构成；

+ Rust 中的匹配是穷尽的，必须穷举到最后的可能性来使代码有效，`match` 确保了所有可能的情况都得到处理；

+ 基本使用：

  ```rust
  enum Coin {
      Penny,
      Nickel,
      Dime,
      Quarter,
  }
  
  fn value_in_cents(coin: Coin) -> u8 {
      match coin {
          Coin::Penny => 1,
          Coin::Nickel => 5,
          Coin::Dime => 10,
          Coin::Quarter => 25,
      }
  }
  ```

  + 每个分支相关联的代码是一个表达式，而表达式的结果值将作为整个 `match` 表达式的返回值；
  + 每个分支可返回任意类型。

+ `match` 可以从枚举成员中提取值：

  ```rust
  enum Coin {
      Penny,
      Nickel,
      Dime,
      Quarter(ChinaProvince),
  }
  
  fn value_in_cents(coin: Coin) -> u8 {
      match coin {
          Coin::Penny => 1,
          Coin::Nickel => 5,
          Coin::Dime => 10,
          Coin::Quarter(province) => {
              let status = province;
              25
          }
      }
  }
  ```
  
+ 匹配 `Option<T>` ：

  ```rust
  fn plus_one(x: Option<i32>) -> Option<i32> {
      match x {
          None => None,
          Some(i) => Some(i + 1),
      }
  }
  
  let five = Some(5);
  let absent_number: Option<i32> = None;
  let six = plus_one(some_number);
  let none = plus_one(absent_number);
  ```

  + `i` 绑定了 `Some` 中包含的值。
  
+ 通配符模式，对一些特定的值采取特殊操作，而对其他的值采取默认操作：

  ```rust
  match dice_roll {
      3 => add_fancy_hat(),
      7 => remove_fancy_hat(),
      other => move_player(other),
  }
  ```

  + 最后一个模式将匹配所有未被特殊列出的值；
  + 必须将通配分支放在最后，因为模式是按顺序匹配的。

+ 匹配任意值而不绑定到该值：

  ```rust
  match dice_roll {
      3 => add_fancy_hat(),
      7 => remove_fancy_hat(),
      _ => reroll(),
  }
  ```

  + 在最后一个分支中明确地忽略了其他的值；
  + 也可以用 `_ => ()` 表示不运行任何代码。

+ `if let` 语法来处理只匹配一个模式的值而忽略其他模式的情况：

  ```rust
  fn main() {
      let config_max = Some(3u8);
      match config_max {
          Some(max) => println!("The maximum is configured to be {}", max),
          _ => (),
      }
      // 行为一致
      if let Some(max) = config_max {
          println!("The maximum is configured to be {}", max);
      }
    
      let mut count = 0;
      if let Coin::Quarter(state) = coin {
          println!("State quarter from {:?}!", state);
      } else {
          count += 1;
      }
  }
  ```

  + 可以将 `if let`  看作 `match` 的语法糖；
  + `if let` 语法获取通过等号分隔的一个模式和一个表达式，模式是 `Some(max)`，`max` 绑定为 `Some` 中的值；
  + 会失去 `match` 强制要求的穷尽性检查；
  + 可以在 `if let` 中包含一个 `else`，`else` 块中的代码与 `match` 表达式中的 `_` 分支块中的代码相同。





























