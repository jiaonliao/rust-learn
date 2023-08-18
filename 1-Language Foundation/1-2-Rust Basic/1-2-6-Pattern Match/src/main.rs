/// 模式匹配
/// 用于为复杂的类型系统提供一个轻松的结构能力
fn main() {
    // match 和 if let
    // match 匹配
    {
        enum Direction {
            East,
            West,
            North,
            South,
        }
        let dire = Direction::South;
        match dire {
            Direction::East => println!("East"),
            Direction::North | Direction::South => {
                println!("South or North");
            }
            _ => println!("West"),
        };
    }
    // 通用语法
    // match VALUE {
    //     PATTERN => EXPRESSION,
    //     PATTERN => EXPRESSION,
    //     PATTERN => EXPRESSION,
    //     _ => EXPRESSION,
    // }
    // match 本身是一个表达式，可以进行赋值
    {
        enum Direction {
            East,
            West,
            North,
            South,
        }
        let dire = Direction::South;
        let result = match dire {
            Direction::East => "East",
            Direction::North | Direction::South => "South or North",
            _ => "West",
        };
        println!("{}", result);
    }
    // 模式绑定，从模式中取出绑定的值
    {
        enum IpAddr {
            V4(u8, u8, u8, u8),
            V6(String),
        }
        let ip = IpAddr::V4(127, 0, 0, 1);
        match ip {
            IpAddr::V4(a, b, c, d) => println!("{}.{}.{}.{}", a, b, c, d),
            IpAddr::V6(s) => println!("{}", s),
        }
    }
    // match 必须匹配所有情况
    // 存在不想处理的模式时，可以使用 _ 忽略

    // if let
    // if let 用于匹配只处理一个模式时的情况
    {
        enum IpAddr {
            V4(u8, u8, u8, u8),
            V6(String),
        }
        let ip = IpAddr::V4(127, 0, 0, 1);
        if let IpAddr::V4(a, b, c, d) = ip {
            println!("{}.{}.{}.{}", a, b, c, d);
        }
    }
    // 当你只要匹配一个条件，且忽略其他条件时就用 if let ，否则都用 match。


    // #############解构 Option#############
    // Option<T> 是一个枚举，Some(T) 和 None 是它的成员
    // 如何匹配Option呢
    {
        let some_number = Some(5);
        let some_string = Some("a string");
        let mut absent_number: Option<i32> = None;
        let a = absent_number.get_or_insert(1);
        // 匹配Option
        match absent_number {
            Some(i) => println!("{}", i),
            None => println!("None"),
        }
    }

    // #############模式适用场景#############
    // match
    {
        // match 往往用于匹配多个模式
        let x = 1;
        match x {
            1 => println!("one"),
            2 => println!("two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
    }
    // if let
    {
        // if let 往往用于匹配单个模式
        let x = Some(5);
        if let Some(5) = x {
            println!("x is five");
        }
    }
    // while let
    {
        // while let 用于循环匹配
        let mut stack = Vec::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        while let Some(top) = stack.pop() {
            println!("{}", top);
        }
    }

    // for 循环
    {
        // for 循环用于遍历迭代器的元素
        let v = vec!['a', 'b', 'c'];
        for (index, value) in v.iter().enumerate() {
            println!("{} is at index {}", value, index);
        }
    }
    // let 语句
    {
        // let 语句用于解构一个模式
        let (x, y, z) = (1, 2, 3);
        println!("{} {} {}", x, y, z);
    }
}
