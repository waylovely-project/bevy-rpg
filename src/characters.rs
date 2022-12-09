use bevy::prelude::*;

pub trait Character {
    fn name(&self) -> Option<Text>;
    /// The dialog style of the character
    ///
    /// For example: when the character Amia says "I wish we were friends from middle school". The text that says "I wish we were friends from middle school" can be written in a gold color and a handwriting font.
    ///
    /// Another example is maybe when you have an antagonistic character in a very tense scene. You could set their color to red and maybe to a scary looking font you found on the Internet.
    fn dialog_style(&self) -> Option<TextStyle>;
}

///
///
///
pub enum PossibleCharacter {
    Single(SingleCharacter),
    Multi(MultipleCharacters),
}

#[derive(Default)]
pub struct MultipleCharacters {
    pub chars: Vec<PossibleCharacter>,
    pub name: Option<Text>,
}

impl Character for MultipleCharacters {
    ///
    ///
    /// it depends on how many, it trie
    fn name(&self) -> Option<Text> {
        todo!()
    }

    fn dialog_style(&self) -> Option<TextStyle> {
        todo!()
    }
}

impl MultipleCharacters {
    pub fn default_name(&self) -> String {
        String::new()
    }
}
impl Default for PossibleCharacter {
    fn default() -> Self {
        Self::Multi(MultipleCharacters {
            chars: vec![],
            ..default()
        })
    }
}

pub struct SingleCharacter {}

impl Character for SingleCharacter {
    fn name(&self) -> Option<Text> {
        todo!()
    }

    fn dialog_style(&self) -> Option<TextStyle> {
        todo!()
    }
}

impl Character for PossibleCharacter {
    fn name(&self) -> Option<Text> {
        match self {
            PossibleCharacter::Single(char) => char.name(),
            PossibleCharacter::Multi(chars) => chars.name(),
        }
    }

    fn dialog_style(&self) -> Option<TextStyle> {
        todo!()
    }
}

