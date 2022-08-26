```
fn main() {
    let mut a = vec![];
    std::mem::drop(a);
    a = vec![10];
}
```

我不明白为什么不报错
我觉得这个应该要报错

被move 掉的变量，还可以起死回生

