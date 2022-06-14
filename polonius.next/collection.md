```
// https://github.com/rust-lang/rust/issues/47680
fn get_default(opt: &mut Option<u8>) -> &mut u8 {
    match opt.as_mut() {
        Some(value) => value,
        None => opt.insert(0),
    }
}
```

