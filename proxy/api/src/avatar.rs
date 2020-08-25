#![allow(
    clippy::as_conversions,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::float_arithmetic,
    clippy::integer_arithmetic
)]

//! Avatar generation.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Emoji whitelist for all usages.
///
/// Note that these are `str` and not `char` because an emoji can span multiple unicode scalars.
#[allow(clippy::non_ascii_literal)]
#[rustfmt::skip]
const EMOJIS: &[&str] = &[
    "👊", "✌️", "🤘", "👌", "👋", "👀", "🧠", "🧶", "🧵", "👠", "🥾", "🧤", "🧣", "🎩", "🧢",
    "🎓", "⛑", "👑", "👜", "💼", "🎒", "🧳", "👓", "🕶", "🥽", "🌂", "🛺", "🪂", "🪐", "🤿",
    "🪀", "🪁", "🪕", "🪔", "🪓", "🪑", "🪒", "🐣", "🐥", "🦆", "🦢", "🦉", "🦚", "🦜", "🦇",
    "🐺", "🐗", "🐴", "🦄", "🐝", "🐛", "🦋", "🐌", "🐚", "🐞", "🐜", "🦗", "🕷", "🦂", "🦟",
    "🦠", "🐢", "🐍", "🦎", "🦖", "🦕", "🐙", "🦑", "🦐", "🦀", "🐡", "🐠", "🐟", "🐬", "🐳",
    "🐋", "🦈", "🐊", "🐅", "🐆", "🦓", "🦍", "🐘", "🦏", "🐪", "🐫", "🦙", "🦒", "🐃", "🐂",
    "🐄", "🐎", "🐖", "🐏", "🐑", "🐐", "🦌", "🐕", "🐩", "🐈", "🐓", "🦃", "🕊", "🐇", "🐁",
    "🐀", "🐿", "🦔", "🦧", "🦮", "🐕‍🦺", "🦥", "🦦", "🦨", "🦩", "☃️", "🐉", "🐲", "🌵", "🎄",
    "🌲", "🌳", "🌴", "🌿", "☘️", "🍀", "🎍", "🎋", "🍃", "🍂", "🍁", "🍄", "🌾", "💐", "🌷",
    "🌹", "🥀", "🌺", "🌸", "🌼", "🌻", "🌞", "🌝", "🌎", "🌍", "🌏", "💫", "⭐️", "🌟", "✨",
    "⚡️", "☄️", "💥", "🔥", "🌈", "☀️", "🌤", "⛅️", "☁️", "🌦", "🌩", "🌨", "❄️", "💨", "💧", "💦",
    "☔️", "☂️", "🌊", "🍏", "🍎", "🍐", "🍊", "🍋", "🍌", "🍉", "🍇", "🍓", "🍈", "🍒", "🍑",
    "🍍", "🥭", "🥥", "🥝", "🍅", "🍆", "🥑", "🥦", "🥒", "🥬", "🌶", "🌽", "🥕", "🥔", "🍠",
    "🥐", "🍞", "🥖", "🥨", "🥯", "🧀", "🥚", "🍳", "🥞", "🥓", "🥩", "🍗", "🍖", "🌭", "🍔",
    "🍟", "🍕", "🥪", "🥙", "🌮", "🌯", "🥗", "🥘", "🥫", "🍝", "🍜", "🍲", "🍛", "🍣", "🍱",
    "🥟", "🍤", "🍙", "🍚", "🍘", "🍥", "🥮", "🥠", "🍢", "🍡", "🍧", "🍨", "🍦", "🥧", "🍰",
    "🎂", "🍮", "🍭", "🍬", "🍫", "🍿", "🧂", "🍩", "🍪", "🌰", "🥜", "🍯", "🥛", "🍼", "☕️",
    "🍵", "🥤", "🍶", "🍺", "🍻", "🥂", "🍷", "🥃", "🍸", "🍹", "🍾", "🥄", "🍴", "🍽", "🥣",
    "🥡", "🥢", "🧄", "🧅", "🧇", "🧆", "🧈", "🦪", "🧃", "🧉", "🧊", "⚽️", "🏀", "🏈", "⚾️",
    "🥎", "🏐", "🏉", "🎾", "🥏", "🎱", "🏓", "🏸", "🥅", "🏒", "🏑", "🥍", "🏏", "⛳️", "🏹",
    "🎣", "🥊", "🥋", "🎽", "⛸", "🥌", "🛷", "🛹", "🎿", "🎪", "🎤", "🎧", "🎹", "🥁", "🎷",
    "🎺", "🎸", "🎻", "🎲", "🎯", "🎳", "🎮", "🎰", "🗺", "🗿", "🚗", "🚕", "🚙", "🚌", "🚎",
    "🏎", "🚓", "🚑", "🚒", "🚐", "🚚", "🚛", "🚜", "🛴", "🚲", "🛵", "🏍", "🚨", "🚔", "🚍",
    "🚘", "🚖", "🚡", "🚠", "🚟", "🚃", "🚋", "🚞", "🚝", "🚄", "🚅", "🚈", "🚂", "🚆", "🚇",
    "🚊", "🚉", "✈️", "🛫", "🛬", "🛩", "💺", "🛰", "🚀", "🛸", "🚁", "🛶", "⛵️", "🚤", "🛥",
    "🛳", "⛴", "🚢", "⚓️", "⛽️", "🚧", "🗼", "🏰", "🏯", "🏟", "🎡", "🎢", "🎠", "⛲️", "⛱",
    "🏖", "🏝", "🏜", "🌋", "⛰", "🏔", "🗻", "🏕", "⛺️", "🏠", "🏡", "🏘", "🏚", "🏗", "🏭",
    "🏢", "⌚️", "🖲", "🕹", "🗜", "💽", "💾", "💿", "📀", "📼", "📷", "📸", "📹", "🎥", "📽",
    "📞", "☎️", "📟", "📠", "📺", "📻", "🎙", "🎚", "🎛", "⏱", "⏲", "⏰", "🕰", "⌛️", "⏳",
    "📡", "🔋", "🔌", "💡", "🔦", "🕯", "💎", "⚖️", "🔧", "🔨", "⚒", "🛠", "⛏", "🔩", "⚙️",
    "⛓", "🛡", "🧭", "🧱", "🔮", "🧿", "🧸", "💈", "⚗️", "🔭", "🧰", "🧲", "🧪", "🧫", "🧬",
    "🧯", "🔬", "🧴", "🧵", "🧶", "🧷", "🧹", "🧺", "🧻", "🧼", "🧽", "🛎", "🔑", "🗝", "🚪",
    "🛋", "🛏", "🛌", "🖼", "🛍", "🧳", "🛒", "🎁", "🎈", "🎏", "🎀", "🎊", "🎉", "🧨", "🎎",
    "🏮", "🎐", "🧧", "✉️", "📨", "📦", "🏷",  "📫", "📮", "📯", "📇", "🗃", "🗳", "🗄", "📋",
    "📁", "🗂", "🗞", "📰", "📓", "📔", "📒", "📕", "📗", "📘", "📙", "📚", "📖", "🔖", "🔗",
    "📎", "🖇", "📐", "📏", "📌", "📍", "✂️", "🖊", "✒️", "🖍", "📝", "✏️", "❤️", "🧡", "💛",
    "💚", "💙", "💜", "🖤", "🔊", "🔔", "📣", "📢", "💠",
];

/// Emoji whitelist for users only.
#[allow(clippy::non_ascii_literal)]
const EMOJIS_USER: &[&str] = &[
    "😀", "😁", "😂", "🤣", "😃", "😄", "😅", "😆", "😉", "😊", "😋", "😎", "🙂", "🤗", "🤩", "🤔",
    "🤨", "😐", "😑", "😶", "🙄", "😏", "😴", "😌", "😒", "🙃", "😲", "🤯", "😬", "🥵", "🥶", "😳",
    "🤪", "🤠", "🤡", "🥳", "🥴", "🥺", "🧐", "🤓", "😈", "👿", "👹", "👺", "💀", "👻", "👽", "🤖",
    "😺", "😸", " ", "😼", "😽", "🐶", "🐱", "🐭", "🐹", "🐰", "🦊", "🦝", "🐻", "🐼", "🦘", "🦡",
    "🐨", "🐯", "🦁", "🐮", "🐷", "🐽", "🐸", "🐵", "🙈", "🙉", "🙊", "🐒", "🐔", "🐧", "🐦", "🐤",
];

/// An emoji.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
pub struct Emoji(&'static str);

impl fmt::Display for Emoji {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Avatar usage.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Usage {
    /// A generic avatar.
    Any,
    /// An [`crate::identity::Identity`] avatar.
    Identity,
}

/// An avatar.
#[derive(Clone, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Avatar {
    /// The emoji component.
    pub emoji: String,
    /// The background color component.
    pub background: Color,
}

impl Avatar {
    /// Generate an avatar from an input string.
    #[must_use]
    pub fn from(input: &str, usage: Usage) -> Self {
        Self {
            emoji: generate_emoji(input, usage).to_string(),
            background: compress_color(generate_color(input)),
        }
    }
}

/// A 32-bit RGBA color.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Color {
    /// The red channel.
    pub r: u8,
    /// The green channel.
    pub g: u8,
    /// The blue channel.
    pub b: u8,

    /// The alpha is here to facilitate working with `u32` values.
    /// We don't use it as part of the output.
    #[serde(skip)]
    a: u8,
}

impl Color {
    /// Create a new color from individual channels.
    #[must_use]
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 0x0 }
    }

    /// Compute the lightness of a color.
    #[must_use]
    pub fn lightness(self) -> f32 {
        let r = f32::from(self.r);
        let g = f32::from(self.g);
        let b = f32::from(self.b);
        let n = f32::from(u8::max_value());

        // This isn't perceptual lightness, but whatever.
        (r / n + g / n + b / n) / 3.
    }

    /// Ligthen a color by an amount between `-1.0` and `1.0`.
    fn lighten(self, amount: f32) -> Self {
        // Constrain range to -1 .. 1.
        let amount = f32::max(amount, -1.0);
        let amount = f32::min(amount, 1.0);

        let x = (amount.abs() * (255_f32)) as u8;

        if amount >= 0. {
            let r = self.r.saturating_add(x);
            let g = self.g.saturating_add(x);
            let b = self.b.saturating_add(x);

            Self::new(r, g, b)
        } else {
            let r = self.r.saturating_sub(x);
            let g = self.g.saturating_sub(x);
            let b = self.b.saturating_sub(x);

            Self::new(r, g, b)
        }
    }
}

impl From<u32> for Color {
    fn from(rgba: u32) -> Self {
        unsafe { std::mem::transmute(rgba) }
    }
}

/// Generate an emoji from an input.
fn generate_emoji(input: &str, usage: Usage) -> Emoji {
    let ix = hash(input);

    match usage {
        Usage::Identity => {
            let ix = ix as usize % (EMOJIS.len() + EMOJIS_USER.len());

            if let Some(s) = EMOJIS.get(ix) {
                Emoji(s)
            } else {
                Emoji(
                    EMOJIS_USER
                        .get(ix - EMOJIS.len())
                        .expect("index of out of range"),
                )
            }
        },
        Usage::Any => Emoji(
            EMOJIS
                .get(ix as usize % EMOJIS.len())
                .expect("index of out of range"),
        ),
    }
}

/// Generate a color from an input.
fn generate_color(input: &str) -> Color {
    // Drop the last 40 bits. We drop 32 bits because our hash output is 64 bits,
    // and we drop 8 bits because we don't use the alpha channel.
    let h = (hash(input) >> (32 + 8)) as u32;

    Color::from(h)
}

/// Compress the range of a color towards the middle.
fn compress_color(c: Color) -> Color {
    let l = c.lightness();

    if l < 0.5 {
        c.lighten(0.125 * (1. - l))
    } else {
        c.lighten(0.125 * -l)
    }
}

/// Fowler–Noll–Vo hash function. We use this simple hash function
/// to make it easy to port the avatar generation algorithm to other
/// platforms.
fn hash(input: &str) -> u64 {
    let bytes = input.bytes();
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325; // FNV offset basis.

    for byte in bytes {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x100_0000_01b3);
    }

    hash
}

#[allow(clippy::float_cmp, clippy::non_ascii_literal)]
#[cfg(test)]
mod test {
    use super::{generate_color, generate_emoji, hash, Avatar, Color, Emoji, Usage};

    #[test]
    fn test_avatar() {
        assert_eq!(
            Avatar::from("cloudhead", Usage::Identity),
            Avatar {
                emoji: "🌻".to_string(),
                background: Color::new(24, 105, 216)
            }
        );
    }

    #[test]
    fn test_avatar_hash() {
        assert_eq!(hash("chongo was here!\n\0"), 0xc33b_ce57_bef6_3eaf);
    }

    #[test]
    fn test_avatar_emoji() {
        assert_eq!(generate_emoji("cloudhead", Usage::Identity), Emoji("🌻"));
        assert_eq!(generate_emoji("radicle", Usage::Any), Emoji("☕\u{fe0f}"));
    }

    #[test]
    fn test_avatar_color() {
        assert_eq!(generate_color("cloudhead"), Color::new(40, 121, 232));
        assert_eq!(generate_color("radicle"), Color::new(255, 49, 16));
    }

    #[test]
    fn test_avatar_lightness() {
        assert_eq!(Color::new(0, 0, 0).lightness(), 0.0);
        assert_eq!(Color::new(0xff, 0xff, 0xff).lightness(), 1.0);
        assert_eq!(Color::new(127, 127, 127).lightness(), 127. / 255.);
    }

    #[test]
    fn test_avatar_lighten() {
        assert_eq!(
            Color::new(0, 0, 0).lighten(1.),
            Color::new(0xff, 0xff, 0xff)
        );
        assert_eq!(
            Color::new(0xff, 0xff, 0xff).lighten(-1.),
            Color::new(0, 0, 0)
        );
        assert_eq!(
            Color::new(0xff, 0xff, 0xff).lighten(1.),
            Color::new(0xff, 0xff, 0xff)
        );
        assert_eq!(Color::new(0, 0, 0).lighten(-1.), Color::new(0, 0, 0));
        assert_eq!(Color::new(0, 0, 0).lighten(0.5), Color::new(127, 127, 127));
    }
}
