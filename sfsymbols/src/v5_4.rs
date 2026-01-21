//! Auto-generated SF Symbols enum.
//! Do not edit manually - regenerate with codegen.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SfSymbolV5_4 {
    /// `beats.pill`
    BeatsPill,
    /// `beats.pill.fill`
    BeatsPillFill,
    /// `beats.solobuds`
    BeatsSolobuds,
    /// `beats.solobuds.chargingcase`
    BeatsSolobudsChargingcase,
    /// `beats.solobuds.chargingcase.fill`
    BeatsSolobudsChargingcaseFill,
    /// `beats.solobuds.left`
    BeatsSolobudsLeft,
    /// `beats.solobuds.right`
    BeatsSolobudsRight,
}

impl SfSymbolV5_4 {
    /// Returns the SF Symbol name string.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::BeatsPill => "beats.pill",
            Self::BeatsPillFill => "beats.pill.fill",
            Self::BeatsSolobuds => "beats.solobuds",
            Self::BeatsSolobudsChargingcase => "beats.solobuds.chargingcase",
            Self::BeatsSolobudsChargingcaseFill => "beats.solobuds.chargingcase.fill",
            Self::BeatsSolobudsLeft => "beats.solobuds.left",
            Self::BeatsSolobudsRight => "beats.solobuds.right",
        }
    }
}

impl AsRef<str> for SfSymbolV5_4 {
    #[inline]
    fn as_ref(&self) -> &str {
        self.name()
    }
}

impl std::fmt::Display for SfSymbolV5_4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
