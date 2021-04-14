```
let mut local = 0;
let x = & mut local ;
let y = & mut *x; // Reborrow x to y.
* x = 1; // Use x again .
* y = 2; // Error ! y used after x got used

// x and y point to the same address
// it's a little strange
```

```
let a=1;
let b=&a;
let c=*&a;
// c and a are at different address
// a copy to c
```


