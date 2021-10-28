# Why we need polonius.next ?

```
1 fn main() {
2     let mut x: (&u32,) = (&22,);
3     let mut y: &u32 = &33;
4     let mut z = 44;
5     y = x.0;
6     x.0 = &z;
7     z += 1;
8     y;
9 }
```

This example can't compile with `rustc main.rs -Zpolonius` .

## Why?
`var_live_on_entry Line6 x`
=> `origin_live_at Line6 origin_x`
=> `subset Line6 origin_x origin_y`
=> `origin_contains_loan Line6 loan_z origin_y`

## Note
This counterexample has many variants.
`x=vec![&22];`

虽然原因各不相同，有些不是var live,而是有个origin还活着。
但是无论如何，都导致了类似的错误

