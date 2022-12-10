use bevy::prelude::*;
pub mod prelude {
    pub use super::MultipleCharacters as Multi;
    pub use super::PossibleCharacter as PC;
    pub use super::SingleCharacter as Single;
}
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
#[derive(Clone)]
pub enum PossibleCharacter {
    Single(SingleCharacter),
    Multi(MultipleCharacters),
}

#[derive(Default, Clone)]
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

impl<A: Into<PossibleCharacter>> From<&A> for PossibleCharacter {
    fn from(char: &A) -> Self {
        char.clone().into()
    }
}
