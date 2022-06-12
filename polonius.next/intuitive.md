# Why we need polonius.next ?

```
1 fn main() {
2     let mut x: (&u32,) = (&22,);
3     let mut z = 44;
4     let y = x.0;
5     x.0 = &z;
6     z += 1;
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

