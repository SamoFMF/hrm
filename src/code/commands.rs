pub mod add;
pub mod bump_down;
pub mod bump_up;
pub mod command;
pub mod copy_from;
pub mod copy_to;
pub mod inbox;
pub mod jump;
pub mod jump_negative;
pub mod jump_zero;
pub mod outbox;
pub mod sub;

pub const ALL_COMMANDS: [&str; 11] = [
    "INBOX", "OUTBOX", "COPYFROM", "COPYTO", "ADD", "SUB", "BUMPUP", "BUMPDN", "JUMP", "JUMPZ",
    "JUMPN",
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
            Command::BumpUp(_) => String::from("BUMPUP"),
            Command::BumpDown(_) => String::from("BUMPDN"),
            Command::Jump(_) => String::from("JUMP"),
            Command::JumpZero(_) => String::from("JUMPZ"),
            Command::JumpNegative(_) => String::from("JUMPN"),
            Command::End => panic!("Command End is not available via syntax."),
        }
    }
}

#[macro_export]
macro_rules! commands {
    () => {
        vec![
            |cmd, val| {
                $crate::code::commands::inbox::Inbox::create(cmd, val).map(|cmd| {
                    Box::new(cmd) as Box<dyn $crate::code::commands::command::CommandNew>
                })
            },
            |cmd, val| {
                $crate::code::commands::outbox::Outbox::create(cmd, val).map(|cmd| {
                    Box::new(cmd) as Box<dyn $crate::code::commands::command::CommandNew>
                })
            },
            |cmd, val| {
                $crate::code::commands::copy_from::CopyFrom::create(cmd, val).map(|cmd| {
                    Box::new(cmd) as Box<dyn $crate::code::commands::command::CommandNew>
                })
            },
            |cmd, val| {
                $crate::code::commands::copy_to::CopyTo::create(cmd, val).map(|cmd| {
                    Box::new(cmd) as Box<dyn $crate::code::commands::command::CommandNew>
                })
            },
            |cmd, val| {
                $crate::code::commands::add::Add::create(cmd, val).map(|cmd| {
                    Box::new(cmd) as Box<dyn $crate::code::commands::command::CommandNew>
                })
            },
            |cmd, val| {
                $crate::code::commands::sub::Sub::create(cmd, val).map(|cmd| {
                    Box::new(cmd) as Box<dyn $crate::code::commands::command::CommandNew>
                })
            },
            |cmd, val| {
                $crate::code::commands::bump_up::BumpUp::create(cmd, val).map(|cmd| {
                    Box::new(cmd) as Box<dyn $crate::code::commands::command::CommandNew>
                })
            },
            |cmd, val| {
                $crate::code::commands::bump_down::BumpDown::create(cmd, val).map(|cmd| {
                    Box::new(cmd) as Box<dyn $crate::code::commands::command::CommandNew>
                })
            },
            |cmd, val| {
                $crate::code::commands::jump::Jump::create(cmd, val).map(|cmd| {
                    Box::new(cmd) as Box<dyn $crate::code::commands::command::CommandNew>
                })
            },
            |cmd, val| {
                $crate::code::commands::jump_zero::JumpZero::create(cmd, val).map(|cmd| {
                    Box::new(cmd) as Box<dyn $crate::code::commands::command::CommandNew>
                })
            },
            |cmd, val| {
                $crate::code::commands::jump_negative::JumpNegative::create(cmd, val).map(|cmd| {
                    Box::new(cmd) as Box<dyn $crate::code::commands::command::CommandNew>
                })
            },
        ];
    };
}
