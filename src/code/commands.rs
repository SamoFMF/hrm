#[derive(Debug, PartialEq)]
pub enum Value {
    Value(i32),
    Index(i32),
}

#[derive(Debug, PartialEq)]
pub enum Command {
    Inbox,
    Outbox,
    CopyFrom(Value),
    CopyTo(Value),
    Add(Value),
    Sub(Value),
    BumpUp(Value),
    BumpDown(Value),
    Jump(String),
    JumpZero(String),
    JumpNegative(String),
    End,
}
