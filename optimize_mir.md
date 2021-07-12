```
struct S;

impl Drop for S {
    fn drop(&mut self) {}
}

fn foo(os: Option<S>) -> usize {
    if let Some(s) = os {
        1
    } else {
        2
    }
}

fn main() {}
```

```
rustc -Zunpretty=mir-cfg main.rs > optimized.dot
dot -Tpng  optimized.dot -o optimized.png
rustc -Znll-facts -Zdump-mir="all" -Zdump-mir-graphviz  main.rs
dot -Tpng  mir_dump/main.foo.-------.nll.0.dot -o unoptimized.png
```

比较两个png,就能发现会有不同的move 契机，如果内部的S已经move出来了，就不会重复drop Option<S>

