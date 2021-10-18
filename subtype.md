```
fn main() {
  let c1 = || {};
  let c2 = || {};
  let v1 = [c1, c2]; //ok

  // fail
  let mut v2 = vec![c1];
  v2.push(c2);
}
```

v1 不会扩充，所以类型可以推导
v2 会扩充，类型还不确定, 会有subtype的问题

