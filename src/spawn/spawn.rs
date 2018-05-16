use spawn::command::Command;

#[derive(Deserialize)]
pub enum Spawn {
    Command(Command),
}
