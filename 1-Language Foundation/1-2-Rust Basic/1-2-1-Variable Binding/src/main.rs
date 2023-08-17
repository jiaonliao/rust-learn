struct Struct {
    e: i32,
    f: i32,
}

fn main() {
    // 变量命名 https://course.rs/practice/naming.html[rust]命名规范
    // #############变量绑定#############
    // 看起来与变量赋值一致，为什么要叫绑定呢，这涉及到一个新概念，及`所有权`。
    let var_bind = "`变量绑定`";
    println!("var_bind:{}", var_bind);
    // #############变量可变性#############
    // 默认情况下，变量不可变，如要使变量可变，需添加mut关键字
    let immutable = "不可变变量"; //immutable="b" 此语句会报错
    println!("immutable:{}", immutable);
    let mut mutable = "可变变量";
    println!("pre mutable:{}", mutable);
    mutable = "`可变变量`";
    println!("post mutable:{}", mutable);
    // #############使用下划线前缀忽略未使用的变量#############
    // 如变量未被使用，编译器会抛出警告，如想消除警告，即在变量前添加`_`即可
    let _not_used = 1;
    // #############变量解构#############
    // let 表达式不仅仅用于变量的绑定，还能进行复杂变量的解构：从一个相对复杂的变量中，匹配出该变量的一部分内容
    let (_de1, _de2): (bool, bool) = (true, false);
    // #############解构式赋值#############
    // let (a, b, c, d, e, f);
    // (a, b) = (3, 5);
    // [c, .., d, _] = [1, 2, 3, 4, 5, 6];
    // Struct {
    //     e,
    //     f,
    // } = Struct {
    //     e: 6,
    //     f: 9,
    // }
    // #############变量和常量之间的差异#############
    // 变量的值不能更改可能让你想起其他另一个很多语言都有的编程概念：常量(constant)。与不可变变量一样，常量也是绑定到一个常量名且不允许更改的值，但是常量和变量之间存在一些差异：
    // 1.常量不允许使用 mut。常量不仅仅默认不可变，而且自始至终不可变，因为常量在编译完成后，已经确定它的值。
    // 2.常量使用 const 关键字而不是 let 关键字来声明，并且值的类型必须标注。
    const MAX_POINTS: u32 = 100_000;
    // #############变量遮蔽(shadowing)#############
    // Rust 允许声明相同的变量名，在后面声明的变量会遮蔽掉前面声明的，如下所示
    let shadow = 1;
    let shadow = "2";
    {
        let shadow = 1 + 2;
        println!("shadow : {}", shadow)
    }
    println!("shadow : {}", shadow)
}
