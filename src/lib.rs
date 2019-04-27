// ↓ これがあると、 Javascript から 'add_one' シンボルで呼べる
#[no_mangle]
pub fn add_one(x: i32) -> i32 {
   x + 1
}