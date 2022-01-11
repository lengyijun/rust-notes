# Why `needlees_collect` is so difficulty?

```
pub fn filter(v: impl IntoIterator<Item = i32>) -> Result<impl Iterator<Item = i32>, usize> {
    let mut zeros = 0;

    let res: Vec<_> = v
        .into_iter()
        .filter(|i| {
            if *i == 0 {
                zeros += 1
            };
            *i != 0
        })
        .collect();

    if zeros != 0 {
      Err(zeros)
    }else{
      Ok(res.into_iter())
    }
}
```

https://github.com/rust-lang/rust-clippy/issues/8055

