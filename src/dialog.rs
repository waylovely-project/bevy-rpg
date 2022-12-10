use bevy::prelude::*;

use crate::characters::{Character, MultipleCharacters, PossibleCharacter, SingleCharacter};

#[derive(Clone)]
pub struct TextDialog {
    pub char: PossibleCharacter,
    pub text: Text,
}

#[derive(Clone)]
pub struct PromptDialog {
    pub text: Option<TextDialog>,
    pub options: Vec<Text>,
}

#[derive(Clone)]
pub enum Dialog {
    Text(TextDialog),
    Prompt(PromptDialog),
}

pub struct DialogIncomingEvent(Dialog);
impl Dialog {
    pub fn start(dialogs: Dialogs, mut dialog_event: EventWriter<DialogIncomingEvent>) {
        for dialog in dialogs.0 {
            dialog_event.send(DialogIncomingEvent(dialog.clone()))
        }
    }
}

pub struct TextWrapper(Text);
pub use TextWrapper as raw;
impl<S> From<S> for TextWrapper
where
    String: From<S>,
{
    fn from(str: S) -> Self {
        Self(Text::from_section(str, Default::default()))
    }
}
impl<C: Into<PossibleCharacter>, T: Into<TextWrapper>> From<(C, T)> for TextDialog {
    fn from((char, text): (C, T)) -> Self {
        Self {
            char: char.into(),
            text: text.into().0,
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

pub struct Dialogs(Vec<Dialog>);

impl<A: IntoIterator<Item = B>, B: Into<Dialog>> From<A> for Dialogs {
    fn from(dialogs: A) -> Self {
        Dialogs(dialogs.into_iter().map(|dialog| dialog.into()).collect())
    }
}
pub enum Prompt {
    Choose(Text),
    Write(Text),
}

pub enum BoxTitle {
    Untitled,
    Titled(Text),
}
pub enum UseDialog {
    Previous,
    None,
    New(Dialog),
}
impl From<(UseDialog, Prompt)> for Dialog {
    fn from(_: (UseDialog, Prompt)) -> Self {
        todo!()
    }
}
