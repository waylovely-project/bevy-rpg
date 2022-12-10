use std::{cell::RefCell, rc::Rc};

use bevy::prelude::*;

use crate::characters::{Character, MultipleCharacters, PossibleCharacter, SingleCharacter};

#[derive(Clone)]
pub struct TextDialog {
    pub char: PossibleCharacter,
    pub text: Text,
}

#[derive(Clone)]
pub enum UseDialogStatus {
    Resolved(TextDialog),
    Unresolved(UseDialog),
}

#[derive(Clone)]
pub struct ChooseDialog {
    pub dialog: Option<UseDialogStatus>,
    pub answers: Vec<Text>,
    pub question: Option<Text>,
}

#[derive(Clone)]
pub enum Dialog {
    Text(TextDialog),
    Choose(ChooseDialog),
}

#[derive(Clone, Eq, PartialEq)]
pub struct WriteDialog {}
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
    ///
    /// This converts
    ///
    /// Alongside just making Into<Dialog> into Dialog, this implementation also sets the UseDialogStatus of unresolved dialogs to the previous dialog if they want to.
    fn from(dialogs: A) -> Self {
        let dialogs: Vec<RefCell<Dialog>> = dialogs
            .into_iter()
            .map(|dialog| RefCell::new(dialog.into()))
            .collect();
        let mut iter = dialogs.iter();
        for (pos, dialog) in iter.clone().enumerate() {
            let mut dialog = dialog.borrow_mut();
            use Dialog::*;
            use UseDialogStatus::*;
            match &mut *dialog {
                Choose(choose) => match choose.dialog {
                    Some(Unresolved(UseDialog::Previous)) => {
                        choose.dialog = match iter.nth(pos - 1) {
                            Some(dialog) => match &*dialog.borrow() {
                                Text(text) => Some(Resolved(text.clone())),
                                _ => None,
                            },
                            None => None,
                        }
                    }
                    _ => {}
                },
                _ => {}
            };
        }
        let mut dialogs_end = vec![];
        for dialog in dialogs {
            dialogs_end.push(dialog.into_inner())
        }
        Dialogs(dialogs_end)
    }
}
pub enum Input<A: Into<TextWrapper>, B: IntoIterator<Item = A>> {
    Choose(B, UseDialog),
    Write(Text),
}

pub enum BoxTitle {
    Untitled,
    Titled(Text),
}
#[derive(Clone)]
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
pub struct ChooseDialogSettings<A: Into<TextWrapper>> {
    pub question: Option<A>,
    pub use_dialog: UseDialog,
}

impl<A: Into<TextWrapper>, B: IntoIterator<Item = A>> From<(B, ChooseDialogSettings<A>)>
    for ChooseDialog
{
    fn from((answers, settings): (B, ChooseDialogSettings<A>)) -> Self {
        Self {
            answers: answers.into_iter().map(|answer| answer.into().0).collect(),
            question: settings.question.map(|text| text.into().0),
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

impl<A: Into<TextWrapper>, B: IntoIterator<Item = A>> From<(B, ChooseDialogSettings<A>)>
    for Dialog
{
    fn from(input: (B, ChooseDialogSettings<A>)) -> Self {
        Dialog::Choose(input.into())
    }
}
