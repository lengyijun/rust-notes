// example 1
#[derive(Debug)]
struct S;

fn main() {
    let s=S;
    let mut v:Vec<&S>=vec![];
    v.push(&s);
    v.pop();
    drop(s);
    // although v has remove the last
    // but still throw error
    dbg!(v);
}

// example 2
// https://github.com/rust-lang/rust/issues/70797


// example 3
// https://rust-lang.zulipchat.com/#narrow/stream/186049-t-compiler.2Fwg-polonius/topic/purely.20forward.20propagation
fn random_bool() -> bool {
    unimplemented!()
}

fn main() {
    let mut x: (&u32,) = (&0,);
    let mut y: (&u32,) = (&1,);
    let mut z = 2;

    if random_bool() {
        y.0 = x.0; // creates `'x: 'y` subset relation
    }

    if random_bool() {
        x.0 = &z; // creates {L0} in 'x constraint
        // this point, we have `'x: 'y` and `{L0} in `'x`, so we also have `{L0} in 'y`
        drop(x.0);
    }

    z += 1; // polonius flags an (unnecessary) error

    drop(y.0);
}
