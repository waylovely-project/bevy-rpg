use bevy::prelude::*;

use crate::{
    characters::{
        text_style, CharacterName, CharacterStyle, MultipleCharacters, PossibleCharacter,
        SingleCharacter,
    },
    ui::DialogIter,
};

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
    pub dialog: Option<UseDialogStatus>,
    pub answers: Vec<Text>,
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

pub struct TextWrapper(Text);
pub use TextWrapper as raw;
impl<S> From<(S, TextStyle)> for TextWrapper
where
    String: From<S>,
{
    fn from((str, style): (S, TextStyle)) -> Self {
        Self(Text::from_section(str, style))
    }
}
impl<C: Into<PossibleCharacter>, T> From<(C, T)> for TextDialog
where
    String: From<T>,
{
    fn from((char, text): (C, T)) -> Self {
        let char = char.into();
        Self {
            text: TextWrapper::from((text, text_style(char.text_style()))).0,
            char,
        }
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
            current: 0,
            current_char_step: 0,
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
pub struct ChooseDialogSettings<A>
where
    (A, TextStyle): Into<TextWrapper>,
{
    pub question: Option<A>,
    pub use_dialog: UseDialog,
}

impl<A, B: IntoIterator<Item = A>, C> From<(B, ChooseDialogSettings<C>, TextStyle)> for ChooseDialog
where
    (A, TextStyle): Into<TextWrapper>,
    String: From<A> + From<C>,
{
    fn from((answers, settings, text_style): (B, ChooseDialogSettings<C>, TextStyle)) -> Self {
        Self {
            answers: answers
                .into_iter()
                .map(|answer| TextWrapper::from((answer, text_style.clone())).0)
                .collect(),
            question: settings
                .question
                .map(|text| TextWrapper::from((text, text_style.clone())).0),
            dialog: {
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

impl<A, B: IntoIterator<Item = A>, C> From<(B, ChooseDialogSettings<C>, TextStyle)> for Dialog
where
    (A, TextStyle): Into<TextWrapper>,
    (C, TextStyle): Into<TextWrapper>,
    String: From<A> + From<C>,
{
    fn from(input: (B, ChooseDialogSettings<C>, TextStyle)) -> Self {
        Dialog::Choose(input.into())
    }
}

impl CharacterName for TextDialog {
    fn charname(&self) -> Option<Text> {
        self.char.charname()
    }
}
