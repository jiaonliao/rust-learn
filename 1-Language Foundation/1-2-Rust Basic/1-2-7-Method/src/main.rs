/// 方法
/// Rust中的方法类似于其他语言中的函数，但它们有一个额外的参数self。
fn main() {
    // #############定义方法#############
    {
        struct Circle {
            x: f64,
            y: f64,
            radius: f64,
        }
        // impl块用于为类型实现方法
        impl Circle {
            // 方法的第一个参数是self，表示方法的调用者
            // self是一个Circle类型的引用
            // self的所有权由方法定义者获取
            // self的借用由方法调用者获取
            fn area(&self) -> f64 {
                std::f64::consts::PI * (self.radius * self.radius)
            }
        }
        let circle = Circle {
            x: 0.0,
            y: 0.0,
            radius: 2.0,
        };
        // self、&self、&mut self
        // 在方法中，使用&self代替 circle: &Circle
        // 实际上，这是一种语法糖，我们使用self来指代结构体的类型
        // 也就是说，self代表了结构体的实例
        // self表示，将结构体的所有权转移给方法定义者
        // &self表示，将结构体的借用权转移给方法定义者
        // &mut self表示，将结构体的可变借用权转移给方法定义者
        println!("{}", circle.area());
    }

    // #############带有多个参数的方法#############
    {
        struct Circle {
            x: f64,
            y: f64,
            radius: f64,
        }
        impl Circle {
            fn area(&self) -> f64 {
                std::f64::consts::PI * (self.radius * self.radius)
            }
            // 带有多个参数的方法
            fn grow(&self, increment: f64) -> Circle {
                Circle {
                    x: self.x,
                    y: self.y,
                    radius: self.radius + increment,
                }
            }
        }
        let circle = Circle {
            x: 0.0,
            y: 0.0,
            radius: 2.0,
        };
        let new_circle = circle.grow(2.0).area();
        println!("{}", new_circle);
    }

    // #############关联函数#############
    {
        struct Circle {
            x: f64,
            y: f64,
            radius: f64,
        }
        impl Circle {
            fn area(&self) -> f64 {
                std::f64::consts::PI * (self.radius * self.radius)
            }
            // 关联函数
            // 关联函数不以self作为参数
            // 关联函数类似于其他语言中的静态方法
            fn new(x: f64, y: f64, radius: f64) -> Circle {
                Circle { x, y, radius }
            }
        }
        let circle = Circle::new(0.0, 0.0, 2.0);
        println!("{}", circle.area());
    }

    // #############枚举的方法#############
    {
        enum Color {
            Red,
            Green,
            Blue,
            RGB(u32, u32, u32),
            HSV(u32, u32, u32),
            HSL(u32, u32, u32),
            CMY(u32, u32, u32),
            CMYK(u32, u32, u32, u32),
        }
        impl Color {
            fn rgb(&self) -> Option<(u32, u32, u32)> {
                match self {
                    Color::RGB(r, g, b) => Some((*r, *g, *b)),
                    Color::HSV(h, s, v) => Some((*h, *s, *v)),
                    Color::HSL(h, s, l) => Some((*h, *s, *l)),
                    Color::CMY(c, m, y) => Some((*c, *m, *y)),
                    Color::CMYK(c, m, y, k) => Some((*c, *m, *y)),
                    _ => None,
                }
            }
        }
        let color = Color::RGB(122, 17, 40);
        println!("{:?}", color.rgb());
    }

    println!("Hello, world!");
}
