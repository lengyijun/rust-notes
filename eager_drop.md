

# eager drop 出问题的场景
https://rust-lang.zulipchat.com/#narrow/stream/122651-general/topic/Why.20is.20Drop.20run.20at.20end.20of.20lexical.20scope.3F
```
let mut v = vec![1, 2, 3];
let ptr = v.as_mut_ptr();
// Now `v` is no longer used, can it be dropped?
unsafe { *ptr = 0 }; // This would be unsound if `v` was already dropped
```

# eager drop 有用的场景
https://github.com/rust-lang/rust/issues/70797


