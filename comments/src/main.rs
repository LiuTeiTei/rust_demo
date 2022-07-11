//! # Comment
//! 外部文档注释--描述包含它的项，一般用来编写 Crate 的文档注释。

/// - 内部文档注释 -- `outer_module` 的单行注释1
/** - 内部文档注释 -- `outer_module` 的块注释1 */
fn main() {
    //! - 外部文档注释 -- `outer_module` 的单行注释2
    /*! - 外部文档注释 -- `outer_module` 的块注释2 */

    /// - 内部文档注释 -- `inner_module` 的单行注释
    /** - 内部文档注释 -- `inner_module` 的块注释 */
    pub mod inner_module {}

    // 这是行注释的例子

    /*
        这是块注释的例子
    */

    /*
     * 这是另外一种注释——块注释。一般而言，行注释是推荐的注释格式，
     * 不过块注释在临时注释大块代码特别有用。/* 块注释可以 /* 嵌套, */ */
     */
}
