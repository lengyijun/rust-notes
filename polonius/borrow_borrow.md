```
a : u8
b : &u8
c : &&u8

a = 0
b = &a    // bw0
c = &b    // bw1
```

a : u8
b : & u8
  (bw0)
     
c : &        &     u8
   (bw1)   (bw0)

类型里，每有一个&，就有一个origin
mir 的 expr 里，每有一个&，就发生一次loan


