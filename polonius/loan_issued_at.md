
```
a : &mutu8
b : &mut u8
a = &mut* b   // loan_issued_at
```

```
a : &u8
b : &mut u8
a = &* b   // loan_issued_at
```

```
a : &u8
b : &u8
a = &* b   // no loan_issued_at
```

