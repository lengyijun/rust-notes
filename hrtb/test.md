```
trait T{}

// 1
// impl<'a> T for fn(&'a u8) {}

//2
impl T for for<'a> fn(&'a u8) {}

//3
impl T for fn(& u8) {}

fn main(){}
```

Question:
1 2 3 单独都是ok 的
但是组合在一起是错误的

1 2 => warning
1 3 => warning
2 3 => error


2 和 3 本质上是一样的
3 是 2 的缩写

