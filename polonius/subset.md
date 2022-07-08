# 为什么 subset 关系要水平传递

v Vec<&u8>  'v

```
temp : &'temp0 Vec<&'temp1 u8>
temp = &'L_v mut v   // subset 'L_v 'temp0
                     // subset 'temp1 'v
                     // subset 'v     'temp1

Vec::push(temp, &'x 0)  // subset  'x 'temp1

目标： 'x 传到 'v 里面
```

