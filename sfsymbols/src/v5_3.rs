//! Auto-generated SF Symbols enum.
//! Do not edit manually - regenerate with codegen.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SfSymbolV5_3 {
    /// `apple.meditate`
    AppleMeditate,
    /// `apple.meditate.square.stack`
    AppleMeditateSquareStack,
    /// `apple.meditate.square.stack.fill`
    AppleMeditateSquareStackFill,
    /// `apple.terminal.circle`
    AppleTerminalCircle,
    /// `apple.terminal.circle.fill`
    AppleTerminalCircleFill,
    /// `arrow.down.app.dashed`
    ArrowDownAppDashed,
    /// `arrow.down.app.dashed.trianglebadge.exclamationmark`
    ArrowDownAppDashedTrianglebadgeExclamationmark,
    /// `audio.jack.mono`
    AudioJackMono,
    /// `audio.jack.stereo`
    AudioJackStereo,
    /// `ipad.badge.exclamationmark`
    IpadBadgeExclamationmark,
    /// `ipad.gen1.badge.exclamationmark`
    IpadGen1BadgeExclamationmark,
    /// `ipad.gen1.landscape.badge.exclamationmark`
    IpadGen1LandscapeBadgeExclamationmark,
    /// `ipad.gen2.badge.exclamationmark`
    IpadGen2BadgeExclamationmark,
    /// `ipad.gen2.landscape.badge.exclamationmark`
    IpadGen2LandscapeBadgeExclamationmark,
    /// `ipad.landscape.badge.exclamationmark`
    IpadLandscapeBadgeExclamationmark,
    /// `iphone.badge.exclamationmark`
    IphoneBadgeExclamationmark,
    /// `iphone.gen1.badge.exclamationmark`
    IphoneGen1BadgeExclamationmark,
    /// `iphone.gen2.badge.exclamationmark`
    IphoneGen2BadgeExclamationmark,
    /// `iphone.gen3.badge.exclamationmark`
    IphoneGen3BadgeExclamationmark,
    /// `medal.star`
    MedalStar,
    /// `medal.star.fill`
    MedalStarFill,
    /// `plus.circle.dashed`
    PlusCircleDashed,
    /// `translate`
    Translate,
}

impl SfSymbolV5_3 {
    /// Returns the SF Symbol name string.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::AppleMeditate => "apple.meditate",
            Self::AppleMeditateSquareStack => "apple.meditate.square.stack",
            Self::AppleMeditateSquareStackFill => "apple.meditate.square.stack.fill",
            Self::AppleTerminalCircle => "apple.terminal.circle",
            Self::AppleTerminalCircleFill => "apple.terminal.circle.fill",
            Self::ArrowDownAppDashed => "arrow.down.app.dashed",
            Self::ArrowDownAppDashedTrianglebadgeExclamationmark => "arrow.down.app.dashed.trianglebadge.exclamationmark",
            Self::AudioJackMono => "audio.jack.mono",
            Self::AudioJackStereo => "audio.jack.stereo",
            Self::IpadBadgeExclamationmark => "ipad.badge.exclamationmark",
            Self::IpadGen1BadgeExclamationmark => "ipad.gen1.badge.exclamationmark",
            Self::IpadGen1LandscapeBadgeExclamationmark => "ipad.gen1.landscape.badge.exclamationmark",
            Self::IpadGen2BadgeExclamationmark => "ipad.gen2.badge.exclamationmark",
            Self::IpadGen2LandscapeBadgeExclamationmark => "ipad.gen2.landscape.badge.exclamationmark",
            Self::IpadLandscapeBadgeExclamationmark => "ipad.landscape.badge.exclamationmark",
            Self::IphoneBadgeExclamationmark => "iphone.badge.exclamationmark",
            Self::IphoneGen1BadgeExclamationmark => "iphone.gen1.badge.exclamationmark",
            Self::IphoneGen2BadgeExclamationmark => "iphone.gen2.badge.exclamationmark",
            Self::IphoneGen3BadgeExclamationmark => "iphone.gen3.badge.exclamationmark",
            Self::MedalStar => "medal.star",
            Self::MedalStarFill => "medal.star.fill",
            Self::PlusCircleDashed => "plus.circle.dashed",
            Self::Translate => "translate",
        }
    }
}

impl AsRef<str> for SfSymbolV5_3 {
    #[inline]
    fn as_ref(&self) -> &str {
        self.name()
    }
}

impl std::fmt::Display for SfSymbolV5_3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
