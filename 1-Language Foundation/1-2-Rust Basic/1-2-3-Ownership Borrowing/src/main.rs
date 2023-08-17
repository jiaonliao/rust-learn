/// 所有权与借用
/// 所有的程序都必须和计算机内存打交道，如何从内存中申请空间来存放程序的运行内容，如何在不需要的时候释放这些空间，成了重中之重，也是所有编程语言设计的难点之一。
/// 在计算机语言不断演变过程中，出现了三种流派：
/// - 垃圾回收机制
/// - 手动内存管理
/// - 所有权机制
/// Rust 选择了第三种，所有权机制，这种机制在编译时就能够保证内存安全，同时也不会造成运行时的性能损失。
fn main() {
    // #############所有权原则#############
    // - Rust 中的每一个值都有一个被称为其 所有者（owner）的变量。
    // - 值有且只有一个所有者。
    // - 当所有者（变量）离开作用域，这个值将被丢弃。

    // #############变量作用域#############
    // 作用域是一个变量在程序中有效的范围, 假如有这样一个变量：
    // let s = "hello";
    // 那么这个变量的作用域就是从声明开始，到当前语句块结束，例如下面的代码：
    {  // s is not valid here, it’s not yet declared
        let s = "hello"; // s is valid from this point forward
        println!("{}", s); // this will print `hello`
    } // this scope is now over, and s is no longer valid
    // println!("{}", s); // this will throw an error: use of undeclared variable `s`

    // #############简单介绍 String 类型#############
    // 我们之前使用过字符串字面值，例如：let s = "hello";，这种字符串类型被称为字符串切片，它是一个不可变的引用，它的大小在编译时就已经确定了。
    // 字符串字面量有以下特性：
    // - 字符串字面量是不可变的
    // - 并非所有字符串的值都能在编码时确定
    // 例如我们要使用户自由输入然后将其存储到内存中，此时就需要使用到 String 类型，该类型被分配到堆上
    let mut str_mut = String::from("hello");
    // ::是一种调用操作符，表示调用String中的from方法，由于String存储在堆上是动态的，你可以这样修改他
    str_mut.push_str(", world!");
    println!("str_mut: {}", str_mut);

    // #############变量绑定背后的数据交互#############
    {
        let x = 5;
        let y = x;
    }
    // 上面的代码中，x和y都是i32类型，当我们将x赋值给y时，实际上是将x的值复制了一份给y，这种情况下，x和y都是独立的，互不影响。
    // 但是当我们将String类型赋值给另一个变量时，情况就不一样了，例如：
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("s1: {}", s1); // 这里会报错，因为s1已经失效了
    // 这里的s1和s2都是String类型，但是当我们将s1赋值给s2时，实际上是将s1的指针、长度和容量复制给了s2，而不是将s1的值复制给了s2，
    // 这种情况下，s1和s2都指向了同一块内存，当s1离开作用域时，s2仍然可以访问到这块内存，但是s1已经失效了，这种情况下，我们称s1被移动了，而不是被复制了。
    // 实际上，这种情况就叫做所有权的转移，转移所有权后Rust将会认为s1不再有效，也不会再尝试释放它的内存。
    // 在此之前，我们将变量赋值称为变量绑定就是因为在Rust体系种，这种操作其实是将值的所有权绑定于变量上。

    // #############Copy#############
    // 有一些类型是可以被复制的，例如整型、浮点型、布尔型、字符型、元组等，这些类型在赋值时会进行复制，而不是转移所有权。
    // 这种类型的所有权不会发生转移，因为这些类型的大小在编译时就已经确定了，所以他们的值都是存储在栈上的，而栈上的值是可以被复制的。
    // 这些类型都实现了Copy trait，而所有实现了Copy trait的类型都可以在变量绑定时进行复制


    // #############函数传值与返回#############
    // 在函数中，参数的传递也是发生了所有权的转移，例如
    {
        let s = String::from("hello");  // s 进入作用域

        takes_ownership(s);             // s 的值移动到函数里 ...
        // ... 所以到这里不再有效

        let x = 5;                      // x 进入作用域

        makes_copy(x);                  // x 应该移动函数里，
    // 但 i32 是 Copy 的，所以在后面可继续使用 x
    fn takes_ownership(some_string: String) { // some_string 进入作用域
        println!("{}", some_string);
    } // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

        fn makes_copy(some_integer: i32) { // some_integer 进入作用域
            println!("{}", some_integer);
        } // 这里，some_integer 移出作用域。不会有特殊操作
    }
    // 从上面的代码中可以看出，当我们将s传递给函数时，实际上是将s的所有权转移给了函数，当函数执行完毕后，s就会被释放，而x则不会，因为i32是Copy的。
    // 同样的，返回值也有所有权的转移
    {
        let s1 = gives_ownership();         // gives_ownership 将返回值
        // 移给 s1

        let s2 = String::from("hello");     // s2 进入作用域

        let s3 = takes_and_gives_back(s2);  // s2 被移动到
    // takes_and_gives_back 中,
    // 它也将返回值移给 s3
    fn gives_ownership() -> String {             // gives_ownership 将返回值移动给
        // 调用它的函数

        let some_string = String::from("hello"); // some_string 进入作用域.

        some_string                              // 返回 some_string 并移出给调用的函数
    }

        // takes_and_gives_back 将传入字符串并返回该值
        fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域

            a_string  // 返回 a_string 并移出给调用的函数
        }
    }

    // #############引用#############
    // 如果将一个值的所有权来回转移，那么程序会变得十分混乱，产生的心智负担也比较重
    // 为了解决这个问题，Rust提供了引用的概念，引用允许你使用值但不获取其所有权，这样就不会发生所有权的转移了


    // #############引用与解引用#############
    // 常规引用是一个指针类型，指向了对象存储的内存地址。在下面代码中，我们创建一个 i32 值的引用 y，然后使用解引用运算符来解出 y 所使用的值
    {
        let x = 5;
        let y = &x;
        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
    // 在上面的代码中，我们创建了一个 i32 类型的引用 y，然后使用解引用运算符 * 来解出 y 所使用的值，这里的 * 是解引用运算符，而不是乘法运算符。
    // 解引用运算符会将引用所指向的对象解出来，这里的 *y 就是将 y 所指向的对象解出来，也就是 x 的值 5。

    // ##############不可变引用#############
    {
        fn main() {
            let s1 = String::from("hello");

            let len = calculate_length(&s1);

            println!("The length of '{}' is {}.", s1, len);
        }

        fn calculate_length(s: &String) -> usize {
            s.len()
        }
    }
    // 在上面的代码中，我们创建了一个 String 类型的引用 s，然后将 s 传递给了 calculate_length 函数，这里的 &s 就是一个不可变引用。
    // 不可变引用允许你使用它的值，但不能修改它的值。
    // 当作用域结束时，也不会释放引用所指向的对象，因为引用并不拥有它所指向的对象的所有权。
    // 如果尝试修改它的值
    {
        fn main() {
            let s = String::from("hello");

            change(&s);
        }

        fn change(some_string: &String) {
            // some_string.push_str(", world"); // 错误！
        }
    }

    // ##############可变引用#############
    // 只需要将不可变引用更改为可变引用即可使上述代码通过编译
    {
        fn main() {
            let mut s = String::from("hello");

            change(&mut s);
        }

        fn change(some_string: &mut String) {
            some_string.push_str(", world");
        }
    }
    // 需要注意的是
    // 可变引用有一个很大的限制：在特定作用域中的特定数据有且只有一个可变引用。
    // 这个限制的好处是：Rust 可以在编译时就避免数据竞争。
    // 我们可以使用大括号解决这个问题。
    {
        let mut s = String::from("hello");

        {
            let r1 = &mut s;
        } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用

        let r2 = &mut s;
    }
    // 而且，可变引用与不可变引用不能同时存在
    {
        let mut s = String::from("hello");

        let r1 = &s; // 没问题
        let r2 = &s; // 没问题
        let r3 = &mut s; // 大问题

        println!("{}, {}, and {}", r1, r2, r3);
    }
    // 引用的作用域从声明的地方开始一直持续到最后一次使用为止。
    // 例如，下面的代码是可以编译通过的，因为 r1 和 r2 的作用域不会重叠
    {
        let mut s = String::from("hello");

        let r1 = &s; // 没问题
        let r2 = &s; // 没问题
        println!("{} and {}", r1, r2);
        // 此位置之后 r1 和 r2 不再使用

        let r3 = &mut s; // 没问题
        println!("{}", r3);
    }
    // 这种特性被称为NLL，即Non-Lexical Lifetimes(NLL)


    // ##############悬垂引用(Dangling References)#############
    // 悬垂引用是指当一个引用指向了其所指向的对象被释放的内存。
    // {
    //     fn main() {
    //         let reference_to_nothing = dangle();
    //     }

    // fn dangle() -> &String { // dangle 返回一个字符串的引用,但是在返回时，就离开了其值的作用域
    //     let s = String::from("hello");
    //     &s
    // }
    // }
    // 如果要使上述代码生效，可将引用改为所有权的转移
    {
        fn main() {
            let s = no_dangle();
        }

        fn no_dangle() -> String {
            let s = String::from("hello");
            s
        }
    }
    // 总的来说，引用有以下规则
    // 1.在任意给定时间，要么只能有一个可变引用，要么只能有多个不可变引用。
    // 2.引用必须总是有效的。


    println!("Hello, world!");
}
