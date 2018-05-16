use spawn::Spawns;
use timer::Timing;

#[derive(Deserialize)]
pub struct Command {
    timing: Timing,
}

impl Spawns for Command {}
