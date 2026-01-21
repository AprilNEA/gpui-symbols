//! Auto-generated SF Symbols enum.
//! Do not edit manually - regenerate with codegen.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SfSymbolV2_2 {
    /// `airpodsmax`
    Airpodsmax,
    /// `applewatch.side.right`
    ApplewatchSideRight,
    /// `character.bubble`
    CharacterBubble,
    /// `character.bubble.ar`
    CharacterBubbleAr,
    /// `character.bubble.fill`
    CharacterBubbleFill,
    /// `character.bubble.fill.ar`
    CharacterBubbleFillAr,
    /// `character.bubble.fill.he`
    CharacterBubbleFillHe,
    /// `character.bubble.he`
    CharacterBubbleHe,
    /// `character.cursor.ibeam`
    CharacterCursorIbeam,
    /// `character.cursor.ibeam.ar`
    CharacterCursorIbeamAr,
    /// `character.cursor.ibeam.he`
    CharacterCursorIbeamHe,
    /// `character.cursor.ibeam.hi`
    CharacterCursorIbeamHi,
    /// `character.cursor.ibeam.ja`
    CharacterCursorIbeamJa,
    /// `character.cursor.ibeam.ko`
    CharacterCursorIbeamKo,
    /// `character.cursor.ibeam.th`
    CharacterCursorIbeamTh,
    /// `character.cursor.ibeam.zh`
    CharacterCursorIbeamZh,
    /// `character.textbox`
    CharacterTextbox,
    /// `character.textbox.ar`
    CharacterTextboxAr,
    /// `character.textbox.he`
    CharacterTextboxHe,
    /// `character.textbox.hi`
    CharacterTextboxHi,
    /// `character.textbox.ja`
    CharacterTextboxJa,
    /// `character.textbox.ko`
    CharacterTextboxKo,
    /// `character.textbox.th`
    CharacterTextboxTh,
    /// `character.textbox.zh`
    CharacterTextboxZh,
    /// `hifispeaker.and.homepodmini`
    HifispeakerAndHomepodmini,
    /// `hifispeaker.and.homepodmini.fill`
    HifispeakerAndHomepodminiFill,
    /// `homepod.and.homepodmini`
    HomepodAndHomepodmini,
    /// `homepod.and.homepodmini.fill`
    HomepodAndHomepodminiFill,
    /// `homepodmini`
    Homepodmini,
    /// `homepodmini.2`
    Homepodmini2,
    /// `homepodmini.2.fill`
    Homepodmini2Fill,
    /// `homepodmini.fill`
    HomepodminiFill,
    /// `rectangle.topthird.inset.fill`
    RectangleTopthirdInsetFill,
}

impl SfSymbolV2_2 {
    /// Returns the SF Symbol name string.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Airpodsmax => "airpodsmax",
            Self::ApplewatchSideRight => "applewatch.side.right",
            Self::CharacterBubble => "character.bubble",
            Self::CharacterBubbleAr => "character.bubble.ar",
            Self::CharacterBubbleFill => "character.bubble.fill",
            Self::CharacterBubbleFillAr => "character.bubble.fill.ar",
            Self::CharacterBubbleFillHe => "character.bubble.fill.he",
            Self::CharacterBubbleHe => "character.bubble.he",
            Self::CharacterCursorIbeam => "character.cursor.ibeam",
            Self::CharacterCursorIbeamAr => "character.cursor.ibeam.ar",
            Self::CharacterCursorIbeamHe => "character.cursor.ibeam.he",
            Self::CharacterCursorIbeamHi => "character.cursor.ibeam.hi",
            Self::CharacterCursorIbeamJa => "character.cursor.ibeam.ja",
            Self::CharacterCursorIbeamKo => "character.cursor.ibeam.ko",
            Self::CharacterCursorIbeamTh => "character.cursor.ibeam.th",
            Self::CharacterCursorIbeamZh => "character.cursor.ibeam.zh",
            Self::CharacterTextbox => "character.textbox",
            Self::CharacterTextboxAr => "character.textbox.ar",
            Self::CharacterTextboxHe => "character.textbox.he",
            Self::CharacterTextboxHi => "character.textbox.hi",
            Self::CharacterTextboxJa => "character.textbox.ja",
            Self::CharacterTextboxKo => "character.textbox.ko",
            Self::CharacterTextboxTh => "character.textbox.th",
            Self::CharacterTextboxZh => "character.textbox.zh",
            Self::HifispeakerAndHomepodmini => "hifispeaker.and.homepodmini",
            Self::HifispeakerAndHomepodminiFill => "hifispeaker.and.homepodmini.fill",
            Self::HomepodAndHomepodmini => "homepod.and.homepodmini",
            Self::HomepodAndHomepodminiFill => "homepod.and.homepodmini.fill",
            Self::Homepodmini => "homepodmini",
            Self::Homepodmini2 => "homepodmini.2",
            Self::Homepodmini2Fill => "homepodmini.2.fill",
            Self::HomepodminiFill => "homepodmini.fill",
            Self::RectangleTopthirdInsetFill => "rectangle.topthird.inset.fill",
        }
    }
}

impl AsRef<str> for SfSymbolV2_2 {
    #[inline]
    fn as_ref(&self) -> &str {
        self.name()
    }
}

impl std::fmt::Display for SfSymbolV2_2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
