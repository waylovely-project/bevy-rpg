use bevy::prelude::*;

use crate::characters::{Character, PossibleCharacter};

pub struct TextDialog {
    pub char: PossibleCharacter,
    pub text: Text,
}

pub struct PromptDialog {
    pub text: Option<TextDialog>,
    pub options: Vec<Text>,
}
pub enum Dialog {
    Text(TextDialog),
    Prompt(PromptDialog),
}

pub struct DialogIncomingEvent(Dialog);
impl Dialog {
    pub fn start<A: Iterator<Item = Dialog>>(
        dialogs: A,
        mut dialog_event: EventWriter<DialogIncomingEvent>,
    ) {
        for dialog in dialogs {
            dialog_event.send(DialogIncomingEvent(dialog))
        }
    }
}

impl From<(PossibleCharacter, Text)> for TextDialog {
    fn from((char, text): (PossibleCharacter, Text)) -> Self {
        Self { char, text }
    }
}

impl From<(PossibleCharacter, Text)> for Dialog {
    fn from(input: (PossibleCharacter, Text)) -> Self {
        Self::Text(input.into())
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
