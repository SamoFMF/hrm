#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CommandValue {
    Value(usize),
    Index(usize),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    Inbox,
    Outbox,
    CopyFrom(CommandValue),
    CopyTo(CommandValue),
    Add(CommandValue),
    Sub(CommandValue),
    BumpUp(CommandValue),
    BumpDown(CommandValue),
    Jump(String),
    JumpZero(String),
    JumpNegative(String),
    End,
}
