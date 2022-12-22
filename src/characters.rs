use bevy::prelude::*;
pub mod prelude {
    pub use super::MultipleCharacters as Multi;
    pub use super::PossibleCharacter as PC;
    pub use super::SingleCharacter as Single;
}
pub trait Character
where
    Self: CharacterName + CharacterStyle,
{
    // The dialog style of the character
    //
    // For example: when the character Amia says "I wish we were friends from middle school". The text that says "I wish we were friends from middle school" can be written in a gold color and a handwriting font.
    //
    // Another example is maybe when you have an antagonistic character in a very tense scene. You could set their color to red and maybe to a scary looking font you found on the Internet.
}

///
///
///
#[derive(Clone, Debug)]
pub enum PossibleCharacter {
    Single(SingleCharacter),
    Multi(MultipleCharacters),
}

#[derive(Clone, Default, Debug)]
pub struct MultipleCharacters {
    pub chars: Vec<PossibleCharacter>,
    pub name: Option<Text>,
    pub text_style: TextStyle,
}

impl Character for MultipleCharacters {}

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

#[derive(Clone, Debug)]
pub struct SingleCharacter {
    pub name: Text,
    pub text_style: TextStyle,
}

impl<A> From<(A, TextStyle, TextStyle)> for SingleCharacter
where
    A: ToString,
{
    fn from((name, name_style, text_style): (A, TextStyle, TextStyle)) -> Self {
        Self {
            name: Text::from_section(name.to_string(), name_style),
            text_style,
        }
    }
}

impl<A> From<(A, TextStyle, TextStyle)> for MultipleCharacters
where
    A: ToString,
{
    fn from((name, name_style, text_style): (A, TextStyle, TextStyle)) -> Self {
        Self {
            name: Some(Text::from_section(name.to_string(), name_style)),
            chars: vec![],
            text_style,
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

impl From<&Self> for PossibleCharacter {
    fn from(char: &Self) -> Self {
        match char {
            Self::Single(single) => Self::Single(single.into()),
            Self::Multi(multi) => Self::Multi(multi.into()),
        }
    }
}
fn text_ptr_to_text(text: &Text) -> Text {
    Text::from_sections(text.sections.iter().map(|section| {
        TextSection::new(
            std::str::from_utf8(section.value.as_bytes())
                .unwrap()
                .to_string(),
            text_style(&section.style),
        )
    }))
}
impl From<&Self> for SingleCharacter {
    fn from(char: &Self) -> Self {
        Self {
            name: text_ptr_to_text(&char.name),
            text_style: text_style(&char.text_style),
        }
    }
}

impl From<&Self> for MultipleCharacters {
    fn from(chars: &Self) -> Self {
        Self {
            chars: chars.chars.iter().map(|char| char.into()).collect(),
            name: chars.name.as_ref().map(text_ptr_to_text),
            text_style: text_style(&chars.text_style),
        }
    }
}

pub(crate) fn text_style(style: &TextStyle) -> TextStyle {
    TextStyle {
        font: style.font.clone(),
        font_size: style.font_size,
        color: Color::rgb(style.color.r(), style.color.g(), style.color.b()),
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

impl CharacterName for MultipleCharacters {
    fn charname(&self) -> Option<Text> {
        match &self.name {
            Some(name) => Some(name.clone()),
            None => {
                let default = || Text::from_section("Noone", Default::default());
                if self.chars.is_empty() {
                    Some(default())
                } else {
                    let mut sections = vec![];
                    for char in &self.chars {
                        let name = char.charname().unwrap_or_else(default);

                        sections.extend(name.sections)
                    }
                    Some(Text::from_sections(sections))
                }
            }
        }
    }
}

impl CharacterName for SingleCharacter {
    fn charname(&self) -> Option<Text> {
        Some(self.name.clone())
    }
}

pub trait CharacterStyle {
    fn text_style(&self) -> &TextStyle;
}
impl CharacterStyle for MultipleCharacters {
    fn text_style(&self) -> &TextStyle {
        &self.text_style
    }
}

impl CharacterStyle for SingleCharacter {
    fn text_style(&self) -> &TextStyle {
        &self.text_style
    }
}

impl CharacterStyle for PossibleCharacter {
    fn text_style(&self) -> &TextStyle {
        match self {
            Self::Single(c) => c.text_style(),
            Self::Multi(c) => c.text_style(),
        }
    }
}
