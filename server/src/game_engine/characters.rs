use crate::{game_engine, shared_types};

pub fn get_character_data(
  character: &shared_types::Character,
) -> &'static game_engine::types::CharacterData {
  match character {
    shared_types::Character::BillyTheKid => &BILLYTHEKID_CHARACTER_DATA,
  }
}

static BILLYTHEKID_CHARACTER_DATA: game_engine::types::CharacterData =
  game_engine::types::CharacterData {
    hp: 5,
    triggers: &[game_engine::types::EventTrigger::Damage],
    effect_optional: true,
    effect: "wat is this type, idk ",
  };
