```
/// miri will give error
fn main() {
    let s: String = "fafsd".to_string();
    std::mem::forget(s);
}
```

