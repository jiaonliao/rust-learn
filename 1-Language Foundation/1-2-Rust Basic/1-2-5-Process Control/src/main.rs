fn main() {
    // #############if#############
    {
        let number = 3;
        if number < 5 {
            println!("condition was true");
        } else {
            println!("condition was false");
        }
    }
    // 需要注意的是，if 是一个表达式，所以可以在 let 语句的右侧使用它。
    {
        let condition = true;
        let number = if condition { 5 } else { 6 };
        println!("The value of number is: {}", number);
    }
    // else if
    {
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

    // #############for#############
    {
        for i in 0..5 {
            println!("Hello, world! : {}", i);
        }

        let a: [i32; 5] = [10, 20, 30, 40, 50];
        for element in a.iter() {
            println!("the value is: {}", element);
        }
        for i in &a {
            println!("the value is: {}", i);
        }
        // 因为数组实现了 IntoIterator trait，所以可以直接使用 for 循环遍历数组。
    }

    // #############loop#############
    // loop是一个表达式，可以返回一个值
    // 返回值可以通过break关键字来指定
    // loop是一个无限循环，可以通过break关键字来退出循环
    {
        let mut counter = 0;
        let result = loop {
            counter += 1;
            if counter == 10 {
                break counter * 2;
            }
        };
        println!("The result is {}", result);
    }

    // #############while#############
    // 如果你需要一个条件来循环，当该条件为 true 时，继续循环，条件为 false，跳出循环，那么 while 就非常适用：
    {
        let mut number = 3;
        while number != 0 {
            println!("{}!", number);
            number -= 1;
        };
        println!("LIFTOFF!!!");
    }




    println!("Hello, world!");
}
