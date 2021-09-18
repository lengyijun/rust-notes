# 什么时候用`unwrap_or`, 什么时候用 `unwrap_or_else` ?

`unwrap_or` 接受一个值， `unwrap_or_else` 接受一个闭包（函数指针）
如果值是一个立即数（1，3,4），那么用`unwrap_or` 好
如果值是一个有一定计算量的，那么用闭包好

https://github.com/rust-lang/rust-clippy/pull/7639

我不赞成这个issue, 我觉得这个开销太大了 


