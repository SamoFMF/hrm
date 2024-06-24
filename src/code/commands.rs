#[derive(Debug, PartialEq)]
pub enum Value {
    Value(u32),
    Index(u32),
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
