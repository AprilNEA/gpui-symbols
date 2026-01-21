//! Auto-generated SF Symbols enum.
//! Do not edit manually - regenerate with codegen.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SfSymbolV1_1 {
    /// `arrow.uturn.left.circle.badge.ellipsis`
    ArrowUturnLeftCircleBadgeEllipsis,
    /// `aspectratio`
    Aspectratio,
    /// `aspectratio.fill`
    AspectratioFill,
    /// `car`
    Car,
    /// `circle.grid.2x2`
    CircleGrid2X2,
    /// `circle.grid.2x2.fill`
    CircleGrid2X2Fill,
    /// `flashlight.off.fill`
    FlashlightOffFill,
    /// `flashlight.on.fill`
    FlashlightOnFill,
    /// `flip.horizontal`
    FlipHorizontal,
    /// `flip.horizontal.fill`
    FlipHorizontalFill,
    /// `mappin.circle`
    MappinCircle,
    /// `mappin.circle.fill`
    MappinCircleFill,
    /// `paperclip.circle`
    PaperclipCircle,
    /// `paperclip.circle.fill`
    PaperclipCircleFill,
    /// `pin.circle`
    PinCircle,
    /// `pin.circle.fill`
    PinCircleFill,
    /// `scissors.badge.ellipsis`
    ScissorsBadgeEllipsis,
    /// `studentdesk`
    Studentdesk,
}

impl SfSymbolV1_1 {
    /// Returns the SF Symbol name string.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::ArrowUturnLeftCircleBadgeEllipsis => "arrow.uturn.left.circle.badge.ellipsis",
            Self::Aspectratio => "aspectratio",
            Self::AspectratioFill => "aspectratio.fill",
            Self::Car => "car",
            Self::CircleGrid2X2 => "circle.grid.2x2",
            Self::CircleGrid2X2Fill => "circle.grid.2x2.fill",
            Self::FlashlightOffFill => "flashlight.off.fill",
            Self::FlashlightOnFill => "flashlight.on.fill",
            Self::FlipHorizontal => "flip.horizontal",
            Self::FlipHorizontalFill => "flip.horizontal.fill",
            Self::MappinCircle => "mappin.circle",
            Self::MappinCircleFill => "mappin.circle.fill",
            Self::PaperclipCircle => "paperclip.circle",
            Self::PaperclipCircleFill => "paperclip.circle.fill",
            Self::PinCircle => "pin.circle",
            Self::PinCircleFill => "pin.circle.fill",
            Self::ScissorsBadgeEllipsis => "scissors.badge.ellipsis",
            Self::Studentdesk => "studentdesk",
        }
    }
}

impl AsRef<str> for SfSymbolV1_1 {
    #[inline]
    fn as_ref(&self) -> &str {
        self.name()
    }
}

impl std::fmt::Display for SfSymbolV1_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
