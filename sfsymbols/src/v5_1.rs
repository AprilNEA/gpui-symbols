//! Auto-generated SF Symbols enum.
//! Do not edit manually - regenerate with codegen.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SfSymbolV5_1 {
    /// `chevron.compact.backward`
    ChevronCompactBackward,
    /// `chevron.compact.forward`
    ChevronCompactForward,
    /// `person.crop.square.badge.camera`
    PersonCropSquareBadgeCamera,
    /// `person.crop.square.badge.camera.fill`
    PersonCropSquareBadgeCameraFill,
    /// `person.crop.square.badge.video`
    PersonCropSquareBadgeVideo,
    /// `person.crop.square.badge.video.fill`
    PersonCropSquareBadgeVideoFill,
}

impl SfSymbolV5_1 {
    /// Returns the SF Symbol name string.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::ChevronCompactBackward => "chevron.compact.backward",
            Self::ChevronCompactForward => "chevron.compact.forward",
            Self::PersonCropSquareBadgeCamera => "person.crop.square.badge.camera",
            Self::PersonCropSquareBadgeCameraFill => "person.crop.square.badge.camera.fill",
            Self::PersonCropSquareBadgeVideo => "person.crop.square.badge.video",
            Self::PersonCropSquareBadgeVideoFill => "person.crop.square.badge.video.fill",
        }
    }
}

impl AsRef<str> for SfSymbolV5_1 {
    #[inline]
    fn as_ref(&self) -> &str {
        self.name()
    }
}

impl std::fmt::Display for SfSymbolV5_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
