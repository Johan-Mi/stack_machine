use derive_more::Display;

#[derive(Clone, Display)]
pub enum Value {
    Int(i32),
}
