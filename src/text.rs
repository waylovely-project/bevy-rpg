use bevy::prelude::*;
use std::collections::{BTreeMap, HashMap};

pub struct DrainedText {
    pub text: Text,
    pub len: usize,
}

impl DrainedText {
    pub fn i_just_want_the_length(text: &Text) -> usize {
        let mut char_vec = vec![];
        for (nth, section) in text.sections.iter().enumerate() {
            for char in section.value.chars() {
                char_vec.push((char, nth, section.style.clone()));
            }
        }
        let len = char_vec.len();
        len
    }
    ///
    pub fn drain_from(text: &Text, max_len: usize) -> Self {
        type Type = Vec<(char, usize, TextStyle)>;
        let mut char_vec: Type = vec![];
        for (nth, section) in text.sections.iter().enumerate() {
            for char in section.value.chars() {
                char_vec.push((char, nth, section.style.clone()));
            }
        }
        let len = char_vec.len();
        if len < max_len {
            warn!("The max length is higher than the lenght of the text itself.");
            return Self {
                text: text.clone(),
                len: len,
            };
        }
        let char_vec: Type = char_vec.drain(..max_len).collect();
        let mut map: BTreeMap<usize, TextSection> = BTreeMap::new();
        for (char, nth, style) in char_vec {
            if map.contains_key(&nth) {
                let section = map.get_mut(&nth).unwrap();

                section.value.push(char);
            } else {
                assert!(map.insert(nth, TextSection::new(char, style)).is_none());
            }
        }

        let mut vec = vec![];

        for (_, map) in map {
            vec.push(map);
        }

        Self {
            text: Text::from_sections(vec),
            len,
        }
    }
}

#[cfg(test)]
mod test {

    use bevy::text::{Text, TextSection};

    use crate::text::DrainedText;

    #[test]
    fn test_index() {
        let text = Text::from_sections([
            TextSection {
                value: "Hiiiiii".to_string(),
                ..Default::default()
            },
            TextSection {
                value: "Fianaaaaa hiiiii".to_string(),
                ..Default::default()
            },
        ]);
        {
            let DrainedText { text, len } = DrainedText::drain_from(&text, 10);
            assert_eq!(text.sections[0].value, "Hiiiiii".to_string(),);
            assert_eq!(text.sections[1].value, "Fia".to_string());
            assert_eq!(len, 7 + 16);
            assert!(len > 10);
        }
        {
            let DrainedText { text, len } = DrainedText::drain_from(&text, 1);
            assert_eq!(text.sections[0].value, "H".to_string(),);
            assert!(len > 1);
        }
        {
            let DrainedText { text, len } = DrainedText::drain_from(&text, 100);

            assert_eq!(text.sections[0].value, "Hiiiiii".to_string());
            assert_eq!(text.sections[1].value, "Fianaaaaa hiiiii".to_string());
            assert!(len < 100);
        }
    }

    #[test]
    pub fn dialog() {
        let dialogs = [
            "Hiii haii haiii!",
            "Hii Yuki!",
            "How was your day?",
            "It was awesome! How about yours?",
            "Me too!",
            "Yahuuu!!",
        ];

        for dialog in dialogs {
            let dialog_text = Text::from_section(dialog, Default::default());
            for max_len in 1..dialogs.len() {
                let DrainedText { text, len } = DrainedText::drain_from(&dialog_text, max_len);

                assert_eq!(text.sections[0].value, dialog[0..max_len]);
                assert_eq!(len, dialog.len());
            }
        }
    }
}
