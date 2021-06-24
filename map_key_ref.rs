use std::collections::HashSet;

#[derive(Hash,Debug)]
struct S(usize);

impl PartialEq for &S {
    fn eq(&self, other: &Self) -> bool {
        // self.0== other.0
        *self as *const S as usize == *other as *const S as usize
    }
}
impl Eq for &S{}

fn main() {
    let mut h=HashSet::<&S>::new();

    let a=S(0);
    let b=S(0);
    h.insert(&a);
    h.insert(&b);

    dbg!(h);

}

// other link:
// https://stackoverflow.com/questions/33847537/how-do-i-make-a-pointer-hashable
