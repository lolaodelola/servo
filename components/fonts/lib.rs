/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

#![deny(unsafe_code)]

mod font;
mod font_context;
mod font_store;
mod font_template;
mod glyph;
#[allow(unsafe_code)]
pub mod platform;
mod shaper;
mod system_font_service;

use std::sync::Arc;

pub use font::*;
pub use font_context::*;
pub use font_store::*;
pub use font_template::*;
pub use glyph::*;
use ipc_channel::ipc::IpcSharedMemory;
pub use platform::LocalFontIdentifier;
pub use shaper::*;
pub use system_font_service::*;
use unicode_properties::{EmojiStatus, UnicodeEmoji, emoji};

/// A data structure to store data for fonts. Data is stored internally in an
/// [`IpcSharedMemory`] handle, so that it can be send without serialization
/// across IPC channels.
#[derive(Clone)]
pub struct FontData(pub(crate) Arc<IpcSharedMemory>);

impl FontData {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self(Arc::new(IpcSharedMemory::from_bytes(bytes)))
    }

    pub(crate) fn as_ipc_shared_memory(&self) -> Arc<IpcSharedMemory> {
        self.0.clone()
    }
}

impl AsRef<[u8]> for FontData {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

/// Whether or not font fallback selection prefers the emoji or text representation
/// of a character. If `None` then either presentation is acceptable.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EmojiPresentationPreference {
    None,
    Text,
    Emoji,
}

#[derive(Clone, Copy, Debug)]
pub struct FallbackFontSelectionOptions {
    pub character: char,
    pub presentation_preference: EmojiPresentationPreference,
}

impl Default for FallbackFontSelectionOptions {
    fn default() -> Self {
        Self {
            character: ' ',
            presentation_preference: EmojiPresentationPreference::None,
        }
    }
}

impl FallbackFontSelectionOptions {
    pub fn new(character: char, next_character: Option<char>) -> Self {
        let presentation_preference = match next_character {
            Some(next_character) if emoji::is_emoji_presentation_selector(next_character) => {
                EmojiPresentationPreference::Emoji
            },
            Some(next_character) if emoji::is_text_presentation_selector(next_character) => {
                EmojiPresentationPreference::Text
            },
            // We don't want to select emoji prsentation for any possible character that might be an emoji, because
            // that includes characters such as '0' that are also used outside of emoji clusters. Instead, only
            // select the emoji font for characters that explicitly have an emoji presentation (in the absence
            // of the emoji presentation selectors above).
            _ if matches!(
                character.emoji_status(),
                EmojiStatus::EmojiPresentation |
                    EmojiStatus::EmojiPresentationAndModifierBase |
                    EmojiStatus::EmojiPresentationAndEmojiComponent |
                    EmojiStatus::EmojiPresentationAndModifierAndEmojiComponent
            ) =>
            {
                EmojiPresentationPreference::Emoji
            },
            _ if character.is_emoji_char() => EmojiPresentationPreference::Text,
            _ => EmojiPresentationPreference::None,
        };
        Self {
            character,
            presentation_preference,
        }
    }
}

pub(crate) fn float_to_fixed(before: usize, f: f64) -> i32 {
    ((1i32 << before) as f64 * f) as i32
}

pub(crate) fn fixed_to_float(before: usize, f: i32) -> f64 {
    f as f64 * 1.0f64 / ((1i32 << before) as f64)
}
