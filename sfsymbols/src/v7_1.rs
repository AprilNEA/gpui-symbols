//! Auto-generated SF Symbols enum.
//! Do not edit manually - regenerate with codegen.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SfSymbolV7_1 {
    /// `air.conditioner`
    AirConditioner,
    /// `air.conditioner.slash`
    AirConditionerSlash,
    /// `arrowtriangle.backward.inset.filled.trailingthird.rectangle`
    ArrowtriangleBackwardInsetFilledTrailingthirdRectangle,
    /// `arrowtriangle.down.2`
    ArrowtriangleDown2,
    /// `arrowtriangle.down.2.fill`
    ArrowtriangleDown2Fill,
    /// `arrowtriangle.forward.inset.filled.trailingthird.rectangle`
    ArrowtriangleForwardInsetFilledTrailingthirdRectangle,
    /// `arrowtriangle.up.2`
    ArrowtriangleUp2,
    /// `arrowtriangle.up.2.fill`
    ArrowtriangleUp2Fill,
    /// `button.horizontal.top`
    ButtonHorizontalTop,
    /// `button.horizontal.top.fill`
    ButtonHorizontalTopFill,
    /// `button.vertical.left`
    ButtonVerticalLeft,
    /// `button.vertical.left.fill`
    ButtonVerticalLeftFill,
    /// `button.vertical.right`
    ButtonVerticalRight,
    /// `button.vertical.right.fill`
    ButtonVerticalRightFill,
    /// `camera.viewfinder.badge.automatic`
    CameraViewfinderBadgeAutomatic,
    /// `digitalcrown`
    Digitalcrown,
    /// `digitalcrown.fill`
    DigitalcrownFill,
    /// `digitalcrown.horizontal`
    DigitalcrownHorizontal,
    /// `digitalcrown.horizontal.fill`
    DigitalcrownHorizontalFill,
    /// `head.profile.vision.pro.remove`
    HeadProfileVisionProRemove,
    /// `inset.filled.rectangle.and.person.filled.slash`
    InsetFilledRectangleAndPersonFilledSlash,
    /// `inset.filled.rectangle.and.person.filled.slash.rtl`
    InsetFilledRectangleAndPersonFilledSlashRtl,
    /// `radicand.squareroot`
    RadicandSquareroot,
    /// `radicand.squareroot.ar`
    RadicandSquarerootAr,
    /// `rectangle.badge.sparkles`
    RectangleBadgeSparkles,
    /// `rectangle.badge.sparkles.fill`
    RectangleBadgeSparklesFill,
    /// `slider.horizontal.below.sun.min`
    SliderHorizontalBelowSunMin,
    /// `star.rectangle`
    StarRectangle,
    /// `star.rectangle.fill`
    StarRectangleFill,
}

impl SfSymbolV7_1 {
    /// Returns the SF Symbol name string.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::AirConditioner => "air.conditioner",
            Self::AirConditionerSlash => "air.conditioner.slash",
            Self::ArrowtriangleBackwardInsetFilledTrailingthirdRectangle => "arrowtriangle.backward.inset.filled.trailingthird.rectangle",
            Self::ArrowtriangleDown2 => "arrowtriangle.down.2",
            Self::ArrowtriangleDown2Fill => "arrowtriangle.down.2.fill",
            Self::ArrowtriangleForwardInsetFilledTrailingthirdRectangle => "arrowtriangle.forward.inset.filled.trailingthird.rectangle",
            Self::ArrowtriangleUp2 => "arrowtriangle.up.2",
            Self::ArrowtriangleUp2Fill => "arrowtriangle.up.2.fill",
            Self::ButtonHorizontalTop => "button.horizontal.top",
            Self::ButtonHorizontalTopFill => "button.horizontal.top.fill",
            Self::ButtonVerticalLeft => "button.vertical.left",
            Self::ButtonVerticalLeftFill => "button.vertical.left.fill",
            Self::ButtonVerticalRight => "button.vertical.right",
            Self::ButtonVerticalRightFill => "button.vertical.right.fill",
            Self::CameraViewfinderBadgeAutomatic => "camera.viewfinder.badge.automatic",
            Self::Digitalcrown => "digitalcrown",
            Self::DigitalcrownFill => "digitalcrown.fill",
            Self::DigitalcrownHorizontal => "digitalcrown.horizontal",
            Self::DigitalcrownHorizontalFill => "digitalcrown.horizontal.fill",
            Self::HeadProfileVisionProRemove => "head.profile.vision.pro.remove",
            Self::InsetFilledRectangleAndPersonFilledSlash => "inset.filled.rectangle.and.person.filled.slash",
            Self::InsetFilledRectangleAndPersonFilledSlashRtl => "inset.filled.rectangle.and.person.filled.slash.rtl",
            Self::RadicandSquareroot => "radicand.squareroot",
            Self::RadicandSquarerootAr => "radicand.squareroot.ar",
            Self::RectangleBadgeSparkles => "rectangle.badge.sparkles",
            Self::RectangleBadgeSparklesFill => "rectangle.badge.sparkles.fill",
            Self::SliderHorizontalBelowSunMin => "slider.horizontal.below.sun.min",
            Self::StarRectangle => "star.rectangle",
            Self::StarRectangleFill => "star.rectangle.fill",
        }
    }
}

impl AsRef<str> for SfSymbolV7_1 {
    #[inline]
    fn as_ref(&self) -> &str {
        self.name()
    }
}

impl std::fmt::Display for SfSymbolV7_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
