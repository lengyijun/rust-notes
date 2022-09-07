#![feature(unboxed_closures)]
#![feature(fn_traits)]

struct Foo;

impl FnOnce<()> for Foo{
    type Output = Foo;

    extern "rust-call"  fn call_once(self, _args: ()) -> Self::Output {
        Foo
    }
}

// https://www.youtube.com/watch?v=tNVDjMxCz-c&ab_channel=Rust
fn main(){
    let f : Foo = Foo () ()() ()()();
}

