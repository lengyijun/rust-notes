rust官方教程中关于hrtb有3个link

https://doc.rust-lang.org/nomicon/hrtb.html

https://rust-lang.github.io/rfcs/0387-higher-ranked-trait-bounds.html

https://doc.rust-lang.org/reference/trait-bounds.html

但是我都没看懂。我是在以下例子才看懂 hrtb的

```
// can compile
struct Closure {
    data: (u8, u16),
    func: dyn Fn(&(u8, u16)) -> &u8,
}
```


func这个函数显然需要lifetime参数，但是如果这么写：
```
// can compile
// but with wrong meaning
struct Closure<'a> {
    data: (u8, u16),
    func: dyn Fn(&'a(u8, u16)) -> &'a u8,
}
```

就意味着 Closure和func里的输入lifetime，返回lifetime都有了联系。 这显然是不对的

func里的lifetime应该和Closure的lifetime脱钩。

所以只能搞一个 for<'b> 给 func 了
