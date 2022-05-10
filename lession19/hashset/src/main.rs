// 散列集 HashSet

// HashSet底层是用HashMap实现的，是对HashMap<T, ()>的封装

// HashSet保证元素都是唯一的，不存在重复的元素

// 如果插入的值已经存在于HashSet中，那么新值将会替换旧值。

// 集合的基本操作：（下面的调用全部都返回一个迭代器）
// - union(并集)：获得两个集合中的所有元素（不含重复值）
// - difference(差集)：获取属于第一个集合而不属于第二集合的所有元素
// - intersection(交集)：获取同时属于两个集合的所有元素。
// - symmetric_difference(对称差)：获取所有只属于其中一个集合，而不同时属于两个集合的所有元素

use std::collections::HashSet;
fn main() {
    let mut a: HashSet<i32> = vec![1i32, 2, 3].into_iter().collect();
    let mut b: HashSet<i32> = vec![2i32, 3, 4].into_iter().collect();

    assert!(a.insert(4));
    assert!(a.contains(&4));

    // 如果值已经存在，那么HashSet::insert() 返回false
    // assert!(b.insert(4), "Value 4 is already in set B!");

    b.insert(5);

    println!("A: {:?}", a);
    println!("B: {:?}", b);

    // 乱序打印
    println!("{:?}", a.union(&b).collect::<Vec<&i32>>());

    println!("{:?}", a.difference(&b).collect::<Vec<&i32>>());

    println!("{:?}", a.intersection(&b).collect::<Vec<&i32>>());

    println!("{:?}", a.symmetric_difference(&b).collect::<Vec<&i32>>());
}
