/// 复杂类型（复合类型）
/// 复合类型，顾名思义，就是由多个其他类型组成的类型。
fn main() {
    // #############字符串#############
    // rust中，字符串是比较特殊的
    {
        fn main() {
            let my_name = "Pascal";
            // greet(my_name);
        }

        fn greet(name: String) {
            println!("Hello, {}!", name);
        }
    }
    // 这段代码无法编译通过
    // 因为 greet 函数期望的是一个 String 类型的参数，而 my_name 是一个字符串字面量，也就是一个字符串 slice。

    // #############切片(slice)#############
    // 切片是一个引用，它引用了某个集合类型（vector、字符串等）的一部分数据，但是并不拥有它。
    // 切片的类型写作 &[T]，其中的 T 表示切片引用的数据的类型。
    // 例如，字符串 slice 的类型就是 &str。
    {
        let s = String::from("hello world");
        let hello = &s[0..5];
        let world = &s[6..11];
    }
    // 创建切片的语法：
    //     &s[start..end]
    // start 是 slice 开始的位置，end 是 slice 结束的位置（但不包括该位置）。
    // 简写
    //     &s[..2] == &s[0..2]
    //     &s[3..] == &s[3..len]
    //     &s[..] == &s[0..len]
    // 对于中文字符串，取切片时要格外小心
    {
        let s = String::from("你好");
        let hello = &s[0..3];
        // let hello = &s[0..2]; // 错误！
    }
    // 这段代码无法编译通过，因为中文字符的编码方式是 UTF-8，一个中文字符占 3 个字节，所以 &s[0..3] 才能取到一个完整的中文字符，而 &s[0..2] 只能取到 2 个字节，不足以构成一个中文字符，所以会报错。
    {
        main();
        fn main() {
            let mut s = String::from("hello world");

            let word = first_word(&s);

            // s.clear(); // error!

            println!("the first word is: {}", word);
        }
        fn first_word(s: &String) -> &str {
            &s[..1]
        }
    }
    // 这段代码无法编译通过，因为 first_word 函数返回的是一个字符串 slice，而 s.clear() 会清空整个字符串，这样返回的字符串 slice 就不再有效了。
    // 其他切片
    {
        let a = [1, 2, 3, 4, 5];

        let slice = &a[1..3];

        assert_eq!(slice, &[2, 3]);
    }


    // #############字符串字面量是切片#############
    {
        let s = "Hello, world!";
    }
    // 字符串字面量的类型是 &str，它是一个指向二进制程序特定位置的 slice。
    // 这也就是为什么字符串字面量是不可变的；&str 是一个不可变引用。

    // #############什么是字符串?#############
    // 顾名思义，字符串是由字符组成的连续集合，但是在上一节中我们提到过，Rust 中的字符是 Unicode 类型，
    // 因此每个字符占据 4 个字节内存空间，但是在字符串中不一样，字符串是 UTF-8 编码，
    // 也就是字符串中的字符所占的字节数是变化的(1 - 4)，这样有助于大幅降低字符串所占用的内存空间。
    // Rust 在语言级别，只有一种字符串类型： str，它通常是以引用类型出现 &str，也就是上文提到的字符串切片。
    // 虽然语言级别只有上述的 str 类型，但是在标准库里，还有多种不同用途的字符串类型，其中使用最广的即是 String 类型。
    //
    // str 类型是硬编码进可执行文件，也无法被修改，但是 String 则是一个可增长、可改变且具有所有权的 UTF-8 编码字符串，
    // 当 Rust 用户提到字符串时，往往指的就是 String 类型和 &str 字符串切片类型，这两个类型都是 UTF-8 编码。
    //
    // 除了 String 类型的字符串，Rust 的标准库还提供了其他类型的字符串，例如 OsString， OsStr， CsString 和 CsStr 等，
    // 注意到这些名字都以 String 或者 Str 结尾了吗？它们分别对应的是具有所有权和被借用的变量。

    // #############String 与 &str 的转换#############
    // &str 转换为 String
    {
        let s = "Hello, world!";
        let s = s.to_string();
    }
    // String 转换为 &str
    {
        fn main() {
            let s = String::from("hello,world!");
            say_hello(&s);
            say_hello(&s[..]);
            say_hello(s.as_str());
        }

        fn say_hello(s: &str) {
            println!("{}", s);
        }
    }

    // #############操作字符串#############
    // 追加
    {
        // 字符串必须是可变的,因为这里会改变字符串的内容
        let mut s = String::from("Hello");
        // push_str 方法接受字符串 slice
        s.push_str(", world!");
        println!("{}", s);
    }
    // 插入
    {
        // 字符串必须是可变的,因为这里会改变字符串的内容
        let mut s = String::from("Hello");
        // insert 方法接受一个索引值作为参数，所以我们可以插入字符串的任意位置
        s.insert(5, ',');
        println!("{}", s);
    }
    // 替换
    {
        // 这里不用 mut 也可以，因为 replace 方法会返回一个新的字符串
        let s = String::from("Hello");
        // replace 方法接受两个参数，第一个参数是要替换的索引值，第二个参数是要替换成的字符串
        let new_str = s.replace(5..6, "!");
        println!("{}", new_str);
        //replacen
        // 这里不用 mut 也可以，因为 replacen 方法会返回一个新的字符串
        let s = String::from("Hello");
        // replacen 接受三个参数，第一个参数是要替换的索引值，第二个参数是要替换成的字符串，第三个参数是要替换的次数
        let new_str = s.replacen("l", "!", 2);
        println!("{}", new_str);
        //replace_range
        // 此处使用mut关键字，因为replace_range方法会直接修改原字符串
        let mut s = String::from("Hello");
        // replace_range 接受两个参数，第一个参数是要替换的范围，第二个参数是要替换成的字符串
        s.replace_range(1..3, "a");
        println!("{}", s);
    }
    //删除 (Delete)
    {
        // pop -- 删除最后一个字符并返回
        let mut s = String::from("Hello");
        let c = s.pop();
        println!("c = {:?}", c);
        println!("s = {:?}", s);
        // remove -- 删除指定索引的字符并返回
        let mut s = String::from("Hello");
        let c = s.remove(1); //其按照字节来索引，而不是字符，如果索引到了字符的中间，则会报错
        println!("c = {:?}", c);
        println!("s = {:?}", s);
        // clear -- 清空字符串
        let mut s = String::from("Hello");
        s.clear();
        println!("s = {:?}", s);
        // truncate -- 截断字符串，保留指定长度
        let mut s = String::from("Hello");
        s.truncate(2); //其按照字节来索引，而不是字符，如果索引到了字符的中间，则会报错
        println!("s = {:?}", s);
        // retain -- 保留指定条件的字符
        let mut s = String::from("Hello");
        s.retain(|c| c != 'l');
        println!("s = {:?}", s);
        //连接 (Concatenate)
        // + -- 连接字符串
        let s1 = String::from("Hello");
        let s2 = String::from("World");
        let s3 = s1 + &s2;
    }

    // #############元组#############
    // 元组是一个将多个其他类型的值组合进一个复合类型，它的长度是固定的，其中的元素顺序也是固定的
    {
        let tup: (i32, f64, u8) = (500, 6.4, 1);
        let (x, y, z) = tup; // 匹配模式结构元组
        let five_hundred = tup.0; // 通过索引访问元组元素，通过索引访问时，索引从0开始
        println!("The five_hundred is: {}", five_hundred);
        println!("The value of y is: {}", y);
        println!("The value of tup.0 is: {}", tup.0);
    }


    // #############元组的使用示例#############
    // 元组的使用示例
    {
        //函数返回值
        fn calculate_length(s: String) -> (String, usize) {
            let length = s.len(); // len() 返回字符串的长度

            (s, length)
        }
    }


    // #############结构体#############
    // 结构体是一个高级的数据结构，他是具名的，且拥有具名的N个字段

    // #############结构体的定义#############
    // 结构体的定义有如下的几个部分
    // - 通过关键字 struct 声明结构体
    // - 一个确定的名称，用于标记结构体
    // - 几个拥有确定名称和类型的字段
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }
    // 结构体的实例化

    // 使用结构体更新语法从其他实例创建实例
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    // 结构体实例化
    // - 初始化时，每个字段都必须有值
    // - 字段顺序无关紧要

    // 访问结构体中的字段
    // 通过 . 访问结构体中的字段
    println!("user1's email is: {}", user1.email);
    user1.username = String::from("sssss"); // 需要是可变的结构体才能修改字段的值

    // 简化结构体的创建
    fn build_user(email: String, username: String) -> User {
        User {
            email, // 字段初始化简写语法
            username,
            active: true,
            sign_in_count: 1,
        }
    }

    // 结构体更新语法
    {
        let user2 = User {
            email: String::from("sss"),
            ..user1 // 使用 .. 语法从其他实例创建实例
            // ..语法会将未显式设置值的字段设置为与给定实例对应字段的值
        };
        // 此时，user1的值已经被移动到了user2中，所以不能再使用user1
        // println!("user1's email is: {}", user1.email); // error
        // 但是user1中的某些字段的值没有被移动，所以可以继续使用user1
        // println!("user1's email is: {}", user1.username); // ok
    }

    // #############元组结构体#############
    // 元组结构体是一种特殊的结构体，它们有着结构体名称提供的含义，但没有具名的字段
    {
        struct Color(i32, i32, i32);
        struct Point(i32, i32, i32);
        let black = Color(0, 0, 0);
        let origin = Point(0, 0, 0);
    }

    // #############单元结构体#############
    // 单元结构体是一种特殊的结构体，它们没有任何字段
    // 他适用于没有任何状态的类型，但是想要在类型上实现某些行为
    {
        struct UnitStruct;
    }

    // #############枚举#############
    // 枚举允许你通过列举可能的成员来定义一个类型
    {
        enum IpAddrKind {
            V4,
            V6,
        }
    }
    //枚举类型是一个类型，它会包含所有可能的枚举成员, 而枚举值是该类型中的具体某个成员的实例。

    // #############枚举值#############
    // 枚举值是枚举类型的某个成员的实例
    {
        enum IpAddrKind {
            V4(i32, i32, i32, i32),
            V6(String),
        }
        let four = IpAddrKind::V4(127, 0, 0, 1);
        let six = IpAddrKind::V6(String::from("::1"));
    }
    // 任何类型的数据都可以放入枚举成员中


    // #############Option枚举和其对应的Option<T>类型#############
    // 在其他编程语言中，通常有一个null值来表示一个值不存在，但是在Rust中，这个概念是通过Option<T>枚举来实现的
    // Option<T>枚举定义如下
    {
        enum Option<T> {
            Some(T),
            // 代表一个值存在
            None, // 代表一个值不存在
        }
    }
    // 如何使用
    {
        let some_number = Some(5);
        let some_string = Some("a string");
        let absent_number: Option<i32> = None;
        // 不能将Option<T>类型的值直接当做T来使用
        // let x: i32 = absent_number; // error
        // 但是可以通过match来使用
        let x: i32 = match absent_number {
            Some(i) => i,
            None => 0,
        };
    }

    // #############数组#############
    // 数组是一系列同类型的值的集合，它们在内存中是连续存储的，数组的长度是固定的，一旦声明，它们的长度就不能增长或缩小

    // #############创建数组#############
    {
        let a = [1, 2, 3, 4, 5];
        let a: [i32; 5] = [1, 2, 3, 4, 5]; // 指定类型和长度
        let a = [3; 5]; // 等价于 let a = [3, 3, 3, 3, 3];
    }

    // #############访问数组元素#############
    {
        let a = [1, 2, 3, 4, 5];
        let first = a[0];
        let second = a[1];
    }

    // #############数组越界#############
    // Rust不会在运行时检查数组越界，但是会在编译时检查
    // println!("{}", a[10]); // error

    // #############数组的元素非基础类型#############
    {
        // let s = [String::from("array"); 3];// 数组中的元素类型是String,会产生异常，因为String是非基础类型，他没有实现Copy trait
        let s = std::array::from_fn(|i| String::from("array")); // 使用array宏来创建数组
    }

    // #############数组切片#############
    // 数组切片是对数组的引用，它可以引用数组的一个部分，而不需要拷贝
    {
        let a = [1, 2, 3, 4, 5];
        let slice = &a[1..3]; // 从索引1开始，到索引3结束，但不包括索引3
    }

}
