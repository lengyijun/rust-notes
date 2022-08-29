# Why we need polonius.next ?

```
1 fn main() {
2     let mut x: (&u32,) = (&22,);
3     let y = x.0;  // let y = &* x.0;
4     let mut z = 44;
5     x.0 = &z;
6     z = 1;
7     y;
8 }
```

This example can't compile with `rustc main.rs -Zpolonius` .

## Why?
`var_live_on_entry Line6 x`
=> `origin_live_at Line6 origin_x`
=> `subset Line6 origin_x origin_y`
=> `origin_contains_loan Line6 loan_z origin_y`

## Note
This counterexample has many variants.

```
1 fn main() {
2     let mut x = vec![&22];
3     let mut z = 44;
4     let y = x[0];
5     x[0] = &z;
6     z += 1;
7     y;
8 }
```

虽然原因各不相同，有些不是var live,而是有个origin还活着。
但是无论如何，都导致了类似的错误


According to 
https://rust-lang.zulipchat.com/#narrow/stream/186049-t-compiler.2Fwg-polonius/topic/2022-08/near/295735542

next一开始是想取消反向传播的liveness,
但是现在 next 依然有liveness。

线性程序不需要liveness,
但是环状程序需要liveness

可以对不同的程序用不同的规则
有环状一定要分析liveness
没有环状就不用分析liveness 了

