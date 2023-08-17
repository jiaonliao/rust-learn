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



}
