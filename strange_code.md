```
use std::mem::MaybeUninit;
use std::ptr;

#[repr(C, align(8))]
struct AlignedUninitBuffer {
    //buf: [MaybeUninit<u8>]
    buf: [u8],
}

impl AlignedUninitBuffer {
    fn from_slice(s: &[u8]) -> &Self {
        let align = 8;

        /* The pointer must be aligned to 8 */
        let offset = s.as_ptr().align_offset(align);
        /* Also, length must be aligned to 8 */
        let newlen = s.len() - offset;

        /* Why this line is needed? */
        //let newlen = newlen - (newlen % align);

        println!("len: {}, offset: {}, newlen: {}", s.len(), offset, newlen);

        if let Some(s) = s.get(offset..offset+newlen) {
            return unsafe {
                let ptr: *const [u8] = s;
                let ptr: *const Self = ptr as _;
                &*ptr
            };
        }

        todo!();
    }
}

fn main() {
    let buffer: Vec<u8> = (0u8..24u8).collect();

    for i in 0..8 {
        let s = &buffer[i..];
        let aub = AlignedUninitBuffer::from_slice(s);
        println!("{:?}", &aub.buf);
    }
}
```
