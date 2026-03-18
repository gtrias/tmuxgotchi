mod frames;

use crate::session::SessionStatus;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CreatureType {
    Blob,
    Snail,
    Cat,
    Robot,
    Ghost,
    Bird,
    Octopus,
    Cactus,
    Mushroom,
    Alien,
    Frog,
    Pumpkin,
}

impl CreatureType {
    pub const ALL: [CreatureType; 12] = [
        CreatureType::Blob,
        CreatureType::Snail,
        CreatureType::Cat,
        CreatureType::Robot,
        CreatureType::Ghost,
        CreatureType::Bird,
        CreatureType::Octopus,
        CreatureType::Cactus,
        CreatureType::Mushroom,
        CreatureType::Alien,
        CreatureType::Frog,
        CreatureType::Pumpkin,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            CreatureType::Blob => "Blob",
            CreatureType::Snail => "Snail",
            CreatureType::Cat => "Cat",
            CreatureType::Robot => "Robot",
            CreatureType::Ghost => "Ghost",
            CreatureType::Bird => "Bird",
            CreatureType::Octopus => "Octopus",
            CreatureType::Cactus => "Cactus",
            CreatureType::Mushroom => "Mushroom",
            CreatureType::Alien => "Alien",
            CreatureType::Frog => "Frog",
            CreatureType::Pumpkin => "Pumpkin",
        }
    }
}

/// Assign a creature type based on session_id hash
pub fn creature_for_session(session_id: &str) -> CreatureType {
    let hash: usize = session_id.bytes().map(|b| b as usize).sum();
    CreatureType::ALL[hash % 12]
}

/// Get the sprite frames for a creature in a given state
pub fn get_frames(creature: CreatureType, status: SessionStatus) -> &'static [&'static [&'static str]] {
    frames::get_frames(creature, status)
}
