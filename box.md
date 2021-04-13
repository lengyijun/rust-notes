
Box要求比较高，传参数必须有个初始值


用Box分配一个长数组的方式：

```rust
// 写法1（更好的写法）
// coresponding mir:
// let _1: std::boxed::Box<[i32; 5242880]>; // in scope 0 at src/main.rs:4:9: 4:10
// let mut _2: std::boxed::Box<[i32; _]>; // in scope 0 at src/main.rs:4:11: 4:24
// _2 = Box([i32; _]);              // scope 0 at src/main.rs:4:11: 4:24
// (*_2) = [const 0_i32; 5<<20];    // scope 0 at src/main.rs:4:15: 4:24
// _1 = move _2;                    // scope 0 at src/main.rs:4:11: 4:24
let a=box [0;5<<20];
```



```rust
// 写法2（不好的写法）
// coresponding mir:
// let mut _4: [i32; _];                // in scope 0 at src/main.rs:5:20: 5:29
// let _3: std::boxed::Box<[i32; 5242880]>; // in scope 1 at src/main.rs:5:9: 5:10
// _4 = [const 0_i32; 5<<20];       // scope 1 at src/main.rs:5:20: 5:29
// _3 = Box::<[i32; 5242880]>::new(move _4) -> [return: bb1, unwind: bb4]; // scope 1 at src/main.rs:5:11: 5:30
let b=Box::new([0;5<<20]);
```

在写法2中，_4 是一个分配在stack上的变量，所以会stack overflow

在写法1中，所有的变量都在heap上，所以没有问题
