macro_rules! getAddress {
    ($a:expr) => {{
        let y = format!("{:p}", $a);
        let address = y.trim_start_matches("0x");
        let z: u64 = u64::from_str_radix(&address, 16).unwrap();
        z
    }};
}

fn main() {
    let b = Box::new([0; 10]);
    let address = getAddress!(b);
    println!("{}", address);
    println!("{:p}", b);

    let ptr = Box::into_raw(b);
    let address = getAddress!(ptr);
    println!("{}", address);
    println!("{:p}", ptr);
}

简单的说 Box的地址和里面的地址是一样的
