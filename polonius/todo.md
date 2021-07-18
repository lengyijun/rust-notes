- return a struct
https://github.com/rust-lang/rust/issues/70797

  related to eager drop

  If the mir polonius works on is equiped with eager drop, this can be fixed.


- HRKB

# initialization
- move error

- 规则有冗余
`path_assigned_at` is not needed, it can be replaced with  `path_maybe_initialized_on_exit`
` path_maybe_initialized_on_exit` 名字有误导，其实就是  `path_initialized_on_exit`
` path_maybe_uninitialized_on_exit` 名字有误导，其实就是  `path_uninitialized_on_exit`

- replace `universal_region` to `placeholder`
