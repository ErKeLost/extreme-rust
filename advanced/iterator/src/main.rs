fn main() {
    // 用途来看 迭代器跟 for循环比较相似
    // 都是去遍历一个集合
    // 最主要的差别就是 是否通过索引来访问集合

    let arr = [1, 2, 3, 4, 5];
    for v in arr {
        println!("{}", v);
    }

    let arr1 = [1, 2, 3];
    for v in arr1.into_iter() {
        println!("{}", v);
    }

    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("{}", val);
    }

    // 迭代器有那个迭代器类型

    let mut arr_iter = arr.into_iter();
    assert_eq!(arr_iter.next(), Some(1));
    assert_eq!(arr_iter.next(), Some(2));
    assert_eq!(arr_iter.next(), Some(3));
    assert_eq!(arr_iter.next(), Some(4));

    // next 返回的是 Option 类型

    // 模拟实现for循环

    let values = vec![1, 2, 3, 4, 5];

    {
        let result = match IntoIterator::into_iter(values) {
            mut iter => loop {
                match iter.next() {
                    Some(v) => println!("{}", v),
                    None => break,
                }
            },
        };
        result
    }

    // into_iter 会夺走所有权
    // iter 是借用
    // iter_mut 是可变借用

    let values = vec![1, 2, 3];

    for v in values.into_iter() {
        println!("{}", v)
    }

    // 下面的代码将报错，因为 values 的所有权在上面 `for` 循环中已经被转移走
    // println!("{:?}",values);

    let values = vec![1, 2, 3];
    let _values_iter = values.iter();

    // 不会报错，因为 values_iter 只是借用了 values 中的元素
    // println!("{:?}", values);

    let mut values = vec![1, 2, 3];
    // 对 values 中的元素进行可变借用
    let mut values_iter_mut = values.iter_mut();

    // 取出第一个元素，并修改为0
    if let Some(v) = values_iter_mut.next() {
        *v = 0;
    }

    // 输出[0, 2, 3]
    // println!("{:?}", values);

    // .iter() 方法实现的迭代器 调用 next 方法 返回的类型是 Some(&T)

    // .iter_mut() 方法实现的迭代器 调用next 方法

    // 返回的类型是 Some(&mut T) 因此在if let Some(v) = ....

    // v 的类型是 &mut i32 最终我们可以通过 *v = 0 来修改值

    // IntoIterator trait 是如果某个类型 实现了该特征 可以通过

    // into_iter iter 方法 变成一个迭代器

    // Iterator 就是迭代器的特征

    // 消费者与适配器

    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);

    // v1_iter 是借用了 v1，因此 v1 可以照常使用
    println!("{:?}", v1);

    // 以下代码会报错，因为 `sum` 拿到了迭代器 `v1_iter` 的所有权
    // println!("{:?}",v1_iter);

    let v1: Vec<i32> = vec![1, 2, 3];

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

    assert_eq!(v2, vec![2, 3, 4]);

    // 迭代者适配器就是返回一个新的迭代器
    // collect 一个消费者适配器给 迭代着适配器进行收尾
}
