pub const ALL_COMMANDS: [&str; 11] = [
    "INBOX", "OUTBOX", "COPYFROM", "COPYTO",
    "ADD", "SUB", "BUMPUP", "BUMPDN",
    "JUMP", "JUMPZ", "JUMPN"
];

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

impl Command {
    pub fn get_type(&self) -> String {
        match self {
            Command::Inbox => String::from("INBOX"),
            Command::Outbox => String::from("OUTBOX"),
            Command::CopyFrom(_) => String::from("COPYFROM"),
            Command::CopyTo(_) => String::from("COPYTO"),
            Command::Add(_) => String::from("ADD"),
            Command::Sub(_) => String::from("SUB"),
            Command::BumpUp(_) => String::from("BUMUP"),
            Command::BumpDown(_) => String::from("BUMPDN"),
            Command::Jump(_) => String::from("JUMP"),
            Command::JumpZero(_) => String::from("JUMPZ"),
            Command::JumpNegative(_) => String::from("JUMPN"),
            Command::End => panic!("Command End is not available via syntax."),
        }
    }
}
