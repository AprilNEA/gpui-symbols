//! Auto-generated SF Symbols enum.
//! Do not edit manually - regenerate with codegen.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SfSymbolV6_4 {
    /// `apple.intelligence.badge.xmark`
    AppleIntelligenceBadgeXmark,
    /// `beats.powerbeats.pro.2`
    BeatsPowerbeatsPro2,
    /// `beats.powerbeats.pro.2.chargingcase`
    BeatsPowerbeatsPro2Chargingcase,
    /// `beats.powerbeats.pro.2.chargingcase.fill`
    BeatsPowerbeatsPro2ChargingcaseFill,
    /// `beats.powerbeats.pro.2.left`
    BeatsPowerbeatsPro2Left,
    /// `beats.powerbeats.pro.2.right`
    BeatsPowerbeatsPro2Right,
}

impl SfSymbolV6_4 {
    /// Returns the SF Symbol name string.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::AppleIntelligenceBadgeXmark => "apple.intelligence.badge.xmark",
            Self::BeatsPowerbeatsPro2 => "beats.powerbeats.pro.2",
            Self::BeatsPowerbeatsPro2Chargingcase => "beats.powerbeats.pro.2.chargingcase",
            Self::BeatsPowerbeatsPro2ChargingcaseFill => "beats.powerbeats.pro.2.chargingcase.fill",
            Self::BeatsPowerbeatsPro2Left => "beats.powerbeats.pro.2.left",
            Self::BeatsPowerbeatsPro2Right => "beats.powerbeats.pro.2.right",
        }
    }
}

impl AsRef<str> for SfSymbolV6_4 {
    #[inline]
    fn as_ref(&self) -> &str {
        self.name()
    }
}

impl std::fmt::Display for SfSymbolV6_4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
