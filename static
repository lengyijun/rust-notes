1. 作为引用的生命周期时，意味着该引用在程序的整个运行期间内都有效
2. 作为类型约束（trait bound 或泛型）时，含义是类型不能包含非 &'static 的引用。这包括两部分： &'static T 引用与 owned 类型。

```
// generate random 'static str refs at run-time
fn rand_str_generator() -> &'static str {
    let rand_string = rand::random::<u64>().to_string();
    Box::leak(rand_string.into_boxed_str())
}
```

https://mp.weixin.qq.com/s?src=11&timestamp=1657007565&ver=3901&signature=rAFiqeBzwG3WQ9yZ3ilrFwiDLIj39A7jP05rG0sHqx2ZQrBWXT2mOziDWcR3R4nv6j9gELO1o1s8mxFKcUnB-7-8ZEovT03kwYQeuViiceLEcVbDxPKmCYo8YBXgZWvL&new=1

