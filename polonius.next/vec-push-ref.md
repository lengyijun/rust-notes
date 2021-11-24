```
// -Zpolonius pass        // -Zpolonius fail
fn foo() {                fn bar() {             
  let mut v = vec![];       let mut v = vec![];
  let a = 12;               let a = 12;
  let mut b = 12;           let mut b = 12;
  let mut x = &a;           let mut x = (&a, 1);
  v.push(x);                v.push(x);
  x = &b;                   x.0 = &b;
  b += 1;                   b += 1;
  v;                        v;
}                         }
```

右边的代码不能编译通过，这是false negative。

原因是 subset 关系不恰当的传递。

