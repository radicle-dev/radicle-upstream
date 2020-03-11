//! Org and user avatar generation.

use std::fmt;

/// Emoji whitelist.
///
/// Note that these are `str` and not `char` because an emoji can span multiple unicode scalars.
const EMOJIS: &[&str] = &[
    "⌚️", "📱", "📲", "💻", "⌨️", "💽", "💾", "💿", "📀", "📼", "📷", "📸", "📹", "🎥", "🎞", "📞",
    "☎️", "📟", "📠", "📺", "📻", "⏰", "🕰", "⌛️", "⏳", "📡", "🔋", "🔌", "💡", "🔦", "💸", "💵",
    "💴", "💶", "💷", "💰", "💳", "🧾", "💎", "⚖️", "🔧", "🔨", "", "🔩", "⚙️", "⛓", "🔫", "💣", "🔪",
    "🗡", "⚔️", "🚬", "⚰️", "⚱️", "🏺", "🧭", "🧱", "🔮", "🧿", "🧸", "📿", "💈", "⚗️", "🔭", "🧰", "🧲", "🧪",
    "🧫", "🧬", "🧯", "🔬", "🕳", "💊", "💉", "🚽", "🚰", "🚿", "🛁", "🛀", "🛀🏻", "🛀🏼", "🛀🏽",
    "🛀🏾", "🛀🏿", "🧴", "🧵", "🧶", "🧷", "🧹", "🧺", "🧻", "🧼", "🧽", "🛎", "🔑", "🗝", "🚪", "🛍", "🧳",
    "🛒", "🎁", "🎈", "🎏", "🎀", "🎊", "🎉", "🧨", "🎎", "🏮", "🎐", "🧧", "✉️", "📩", "📨", "📧",
    "💌", "📥", "📤", "📦", "📪", "📫", "📬", "📭", "📮", "📯", "📜", "📃", "📄", "📑", "📊", "📈",
    "📉", "📆", "📅", "📇", "📋", "📁", "📂", "🗂", "🗞", "📰", "📓", "📔", "📒", "📕", "📗", "📘",
    "📙", "📚", "📖", "🔖", "🔗", "📎", "📐", "📏", "📌", "📍", "✂️", "🖊", "🖋", "✒️", "🖌", "📝", "✏️",
    "🔍", "🔎", "🔏", "🔐",
];

/// An emoji.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Emoji(&'static str);

impl fmt::Display for Emoji {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// An avatar.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Avatar {
    /// The emoji component.
    pub emoji: Emoji,
    /// The background color component.
    pub background: Color,
}

impl Avatar {
    /// Generate an avatar from an input string.
    pub fn from(input: &str) -> Self {
        Self {
            emoji: generate_emoji(input),
            background: compress_color(generate_color(input)),
        }
    }
}

/// A 32-bit RGBA color.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Color {
    /// The red channel.
    pub r: u8,
    /// The green channel.
    pub g: u8,
    /// The blue channel.
    pub b: u8,

    // The alpha is here to facilitate working with `u32` values.
    // We don't use it as part of the output.
    a: u8,
}

impl Color {
    /// Create a new color from individual channels.
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b, a: 0x0 }
    }

    /// Compute the lightness of a color.
    pub fn lightness(&self) -> f32 {
        let r = self.r as f32;
        let g = self.g as f32;
        let b = self.b as f32;
        let n = 0xff as f32;

        // This isn't perceptual lightness, but whatever.
        (r / n + g / n + b / n) / 3.
    }

    /// Ligthen a color by an amount between `-1.0` and `1.0`.
    fn lighten(self, amount: f32) -> Self {
        // Constrain range to -1 .. 1.
        let amount = f32::max(amount, -1.0);
        let amount = f32::min(amount, 1.0);

        let x = (amount.abs() * (0xff as f32)) as u8;

        if amount >= 0. {
            let r = self.r.checked_add(x).unwrap_or(0xff);
            let g = self.g.checked_add(x).unwrap_or(0xff);
            let b = self.b.checked_add(x).unwrap_or(0xff);

            Self::new(r, g, b)
        } else {
            let r = self.r.checked_sub(x).unwrap_or(0x0);
            let g = self.g.checked_sub(x).unwrap_or(0x0);
            let b = self.b.checked_sub(x).unwrap_or(0x0);

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
fn generate_emoji(input: &str) -> Emoji {
    let ix = hash(input);

    Emoji(EMOJIS[ix as usize % EMOJIS.len()])
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
    let mut hash: u64 = 0xcbf29ce484222325; // FNV offset basis.

    for byte in bytes {
        hash = hash ^ (byte as u64);
        hash = hash.wrapping_mul(0x100000001b3);
    }

    hash
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_avatar() {
        assert_eq!(
            Avatar::from("cloudhead"),
            Avatar {
                emoji: Emoji("🧱"),
                background: Color::new(24, 105, 216)
            }
        );
    }

    #[test]
    fn test_avatar_hash() {
        assert_eq!(hash("chongo was here!\n\0"), 0xc33bce57bef63eaf);
    }

    #[test]
    fn test_avatar_emoji() {
        assert_eq!(generate_emoji("cloudhead"), Emoji("🧱"));
        assert_eq!(generate_emoji("radicle"), Emoji("🛀🏿"));
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
