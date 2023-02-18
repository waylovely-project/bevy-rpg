use std::any::Any;

use bevy::{prelude::*, utils::HashMap};

use crate::{
    characters::{
        text_style, CharacterName, HasTextStyle, MultipleCharacters, PossibleCharacter,
        SingleCharacter,
    },
    ui::DialogIter,
};

use crate::dialog::wrapper::DialogText;

#[derive(Debug, Clone)]
pub struct TextDialog {
    pub char: PossibleCharacter,
    pub text: Text,
}

#[derive(Debug, Clone)]
pub enum UseDialogStatus {
    Resolved(TextDialog),
    Unresolved(UseDialog),
}

#[derive(Debug, Clone)]
pub struct ChooseDialog {
    pub prev_dialog: Option<UseDialogStatus>,
    pub answers: HashMap<String, Text>,
    pub question: Option<Text>,
}

#[derive(Debug, Clone)]
pub enum Dialog {
    Text(TextDialog),
    Choose(ChooseDialog),
}

#[derive(Clone, Eq, PartialEq)]
pub struct WriteDialog {}
pub struct DialogIncomingEvent(pub Dialog);
impl Dialog {}

impl<C: Into<PossibleCharacter>, T> From<(C, T)> for TextDialog
where
    (PossibleCharacter, T): Into<DialogText>,
{
    fn from((char, text): (C, T)) -> Self {
        let char = char.into();
        let text: DialogText = (char.clone(), text).into();
        let text: Text = (*text).clone();
        Self { char, text }
    }
}

impl<A: ToString, C: Into<PossibleCharacter>> From<(C, A)> for Dialog
where
    TextDialog: From<(C, A)>,
{
    fn from(input: (C, A)) -> Self {
        Self::Text(input.into())
    }
}

#[derive(Debug, Clone)]
pub struct Dialogs {
    pub dialogs: Vec<Dialog>,
    pub defaults: StyleDefaults,
}

#[derive(Debug, Clone)]
pub struct StyleDefaults {
    pub text: TextStyle,
}

impl Dialogs {
    ///
    /// This converts
    ///
    ///
    pub fn add<A: IntoIterator<Item = B>, B: Into<Dialog>>(&mut self, dialogs: A) {
        self.dialogs.extend(
            dialogs
                .into_iter()
                .map(|dialog| dialog.into())
                .collect::<Vec<Dialog>>(),
        )
    }

    pub fn new(defaults: StyleDefaults) -> Self {
        Self {
            dialogs: vec![],
            defaults,
        }
    }

    pub fn single<B>(&self, name: B) -> SingleCharacter
    where
        (B, TextStyle, TextStyle): Into<SingleCharacter>,
    {
        (name, self.defaults.text.clone(), self.defaults.text.clone()).into()
    }

    pub fn multi<B>(&self, name: B) -> MultipleCharacters
    where
        (B, TextStyle, TextStyle): Into<MultipleCharacters>,
    {
        (name, self.defaults.text.clone(), self.defaults.text.clone()).into()
    }

    pub fn start(&mut self, mut commands: Commands) {
        commands.insert_resource(DialogIter {
            dialogs: self.dialogs.clone(),

            ..default()
        })
    }
}
pub enum Input<A, B: IntoIterator<Item = A>> {
    Choose(B, UseDialog),
    Write(Text),
}

pub enum BoxTitle {
    Untitled,
    Titled(Text),
}
#[derive(Debug, Clone)]
pub enum UseDialog {
    Previous,
    None,
    New(TextDialog),
}

impl Default for UseDialog {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Default)]
pub struct ChooseDialogSettings {
    pub question: Option<Text>,
    pub use_dialog: UseDialog,
}
impl<B: IntoIterator<Item = T>, T> From<(B, ChooseDialogSettings, TextStyle, ())> for Dialog
where
    T: ToString,
{
    fn from(value: (B, ChooseDialogSettings, TextStyle, ())) -> Self {
        Dialog::Choose(value.into())
    }
}
impl<B: IntoIterator<Item = T>, T> From<(B, ChooseDialogSettings, TextStyle, ())> for ChooseDialog
where
    T: ToString,
{
    fn from(value: (B, ChooseDialogSettings, TextStyle, ())) -> Self {
        Self::from((
            value
                .0
                .into_iter()
                .map(|text| (text.to_string(), text.to_string())),
            value.1,
            value.2,
        ))
    }
}
impl<B: IntoIterator<Item = (T, Id)>, T, Id> From<(B, ChooseDialogSettings, TextStyle)>
    for ChooseDialog
where
    Id: ToString,
    T: ToString,
{
    fn from((answers, settings, style): (B, ChooseDialogSettings, TextStyle)) -> Self {
        Self::from((
            answers
                .into_iter()
                .map(|(answer, id)| (Text::from_section(answer.to_string(), style.clone()), id)),
            settings,
        ))
    }
}

impl<B: IntoIterator<Item = (T, Id)>, T, Id> From<(B, ChooseDialogSettings, TextStyle)> for Dialog
where
    Id: ToString,
    T: ToString,
{
    fn from(value: (B, ChooseDialogSettings, TextStyle)) -> Self {
        Dialog::Choose(value.into())
    }
}
impl<B: IntoIterator<Item = (Text, Id)>, Id> From<(B, ChooseDialogSettings)> for ChooseDialog
where
    Id: ToString,
{
    fn from((answers, settings): (B, ChooseDialogSettings)) -> Self {
        Self {
            answers: answers
                .into_iter()
                .map(|(answer, id)| (id.to_string(), answer))
                .collect(),
            question: settings.question.map(|text| text),
            prev_dialog: {
                use UseDialog::*;
                match settings.use_dialog {
                    Previous => Some(UseDialogStatus::Unresolved(settings.use_dialog)),
                    None => Option::None,
                    New(dialog) => Some(UseDialogStatus::Resolved(dialog)),
                }
            },
        }
    }
}

impl<B: IntoIterator<Item = (Text, Id)>, Id> From<(B, ChooseDialogSettings)> for Dialog
where
    Id: ToString,
{
    fn from(input: (B, ChooseDialogSettings)) -> Self {
        Dialog::Choose(input.into())
    }
}

impl CharacterName for TextDialog {
    fn charname(&self) -> Option<Text> {
        self.char.charname()
    }
}

pub mod wrapper {

    #[derive(Deref, DerefMut)]
    pub struct DialogText(Text);

    pub(crate) enum TextSource {
        DefaultTextStyle,
        /// Text
        DirectlyFromText,
    }

    use bevy::{
        prelude::{Deref, DerefMut},
        text::{Text, TextStyle},
    };

    use crate::characters::HasTextStyle;

    impl From<Text> for DialogText {
        fn from(text: Text) -> Self {
            Self(text)
            //   source: TextSource::DirectlyFromText,
        }
    }
    impl Into<Text> for DialogText {
        fn into(self) -> Text {
            self.0
        }
    }
    impl<C: HasTextStyle, S: ToString> From<(C, S)> for DialogText {
        fn from((char, text): (C, S)) -> Self {
            Self(Text::from_section(
                text.to_string(),
                char.text_style().clone(),
            ))
            //   source: TextSource::DefaultTextStyle,
        }
    }
}
