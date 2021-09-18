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

