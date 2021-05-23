在 c++ 中， 

```c
char z='z';
int x=z-'a';
```



但是在Rust中，不能这么写。大概有两种写法

```rust
// 写法1
let z:u8=b'z';
let x:u8=z-b'a';
```

```rust
// 写法2
let a:i32='a' as i32 - 'a' as i32;
```

