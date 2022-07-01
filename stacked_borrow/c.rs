// valid
fn main(){
    let x = &mut 0;
    let a = x as *mut i32;
    let b = x as *mut i32;
    let c = x as *mut i32;
    a; b; c; b; a;
}
