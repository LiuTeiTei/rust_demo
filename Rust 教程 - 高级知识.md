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

+  `let` 语句意味着 `expensive_closure` 包含一个匿名函数的**定义**，不是调用匿名函数的**返回值**。



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
  + 被限制为只接受获取一个 `u32` 值并返回一个 `u32` 值的闭包。



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

    

