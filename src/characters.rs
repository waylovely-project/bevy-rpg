use bevy::prelude::*;
pub mod prelude {
    pub use super::MultipleCharacters as Multi;
    pub use super::PossibleCharacter as PC;
    pub use super::SingleCharacter as Single;
}
pub trait Character
where
    Self: CharacterName,
{
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
#[derive(Clone)]
pub enum PossibleCharacter {
    Single(SingleCharacter),
    Multi(MultipleCharacters),
}

#[derive(Clone, Default)]
pub struct MultipleCharacters {
    pub chars: Vec<PossibleCharacter>,
    pub name: Option<Text>,
}

impl Character for MultipleCharacters {
    fn dialog_style(&self) -> Option<TextStyle> {
        todo!()
    }
}

impl MultipleCharacters {
    pub fn default_charname(&self) -> String {
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

#[derive(Clone)]
pub struct SingleCharacter {
    pub name: Text,
}

impl<A> From<A> for SingleCharacter
where
    A: ToString,
{
    fn from(name: A) -> Self {
        Self {
            name: Text::from_section(name.to_string(), Default::default()),
        }
    }
}

impl<A> From<A> for MultipleCharacters
where
    A: ToString,
{
    fn from(name: A) -> Self {
        Self {
            name: Some(Text::from_section(name.to_string(), Default::default())),
            chars: vec![],
        }
    }
}

impl From<SingleCharacter> for PossibleCharacter {
    fn from(char: SingleCharacter) -> Self {
        Self::Single(char)
    }
}

impl From<MultipleCharacters> for PossibleCharacter {
    fn from(chars: MultipleCharacters) -> Self {
        Self::Multi(chars)
    }
}

impl<A: Into<PossibleCharacter>> From<&A> for PossibleCharacter {
    fn from(char: &A) -> Self {
        char.clone().into()
    }
}

pub trait CharacterName {
    fn charname(&self) -> Option<Text>;
}

impl CharacterName for PossibleCharacter {
    fn charname(&self) -> Option<Text> {
        match self {
            PossibleCharacter::Single(char) => char.charname(),
            PossibleCharacter::Multi(chars) => chars.charname(),
        }
    }
}

    fn dialog_style(&self) -> Option<TextStyle> {
        todo!()
    }
}

impl CharacterName for SingleCharacter {
    fn charname(&self) -> Option<Text> {
        Some(self.name.clone())
    }
}
