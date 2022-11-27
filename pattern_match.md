```
let ([x] | [x, _] ) = [1, 2].as_slice() else {
   unreachable!()
};
dbg!(x);
```

```
let (x | x | x) = (1, 2);
```

```
fn main() {
    let x = |_| Some(1);
    let (|x| x) = match x(..) {
        _ | Some(2) => |_| Some(3),
        _ | _ => unreachable!(),
    };
    assert!(matches!(x(..), |_| Some(4)));
}
```

```
let (|x| x) = (|x: u8| x);
```

```
let (x | x) = (1 | 1);
```

