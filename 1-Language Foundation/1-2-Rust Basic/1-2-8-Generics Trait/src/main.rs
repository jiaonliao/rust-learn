/// 泛型与特征
/// 泛型是一种将类型参数化以便于多种类型使用相同代码的方式。
/// 特征是一种将方法签名组合起来的方法，以定义一个未知类型的行为。
fn main() {

    // #############泛型详解#############
    {
        fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
            let mut largest = list[0];
            for &item in list.iter() {
                if item > largest {
                    largest = item;
                }
            }
            largest
        }
        let number_list = vec![34, 50, 25, 100, 65];
        let result = largest(&number_list);
        println!("The largest number is {}", result);
        // largest函数定义了一个泛型T
    }
    println!("Hello, world!");
}
