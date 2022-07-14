use std::cell::RefCell;

thread_local!{
    static BAR: RefCell<f32> = RefCell::new(1.0);
}

fn main() {
}

#[test]
fn test_p1() {
   BAR.with(|f| {
       assert_eq!(*f.borrow(), 1.0);
   })
}