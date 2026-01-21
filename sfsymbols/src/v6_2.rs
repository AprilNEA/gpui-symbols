//! Auto-generated SF Symbols enum.
//! Do not edit manually - regenerate with codegen.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SfSymbolV6_2 {
    /// `airpods.gen4`
    AirpodsGen4,
    /// `airpods.gen4.chargingcase.wireless`
    AirpodsGen4ChargingcaseWireless,
    /// `airpods.gen4.chargingcase.wireless.fill`
    AirpodsGen4ChargingcaseWirelessFill,
    /// `airpods.gen4.left`
    AirpodsGen4Left,
    /// `airpods.gen4.right`
    AirpodsGen4Right,
    /// `apple.writing.tools`
    AppleWritingTools,
    /// `exclamationmark.triangle.text.page`
    ExclamationmarkTriangleTextPage,
    /// `exclamationmark.triangle.text.page.fill`
    ExclamationmarkTriangleTextPageFill,
    /// `exclamationmark.triangle.text.page.fill.rtl`
    ExclamationmarkTriangleTextPageFillRtl,
    /// `exclamationmark.triangle.text.page.rtl`
    ExclamationmarkTriangleTextPageRtl,
    /// `figure.seated.side.left.air.distribution.upper.and.middle.and.lower`
    FigureSeatedSideLeftAirDistributionUpperAndMiddleAndLower,
    /// `figure.seated.side.right.air.distribution.upper.and.middle.and.lower`
    FigureSeatedSideRightAirDistributionUpperAndMiddleAndLower,
    /// `headphones.dots`
    HeadphonesDots,
    /// `info.triangle`
    InfoTriangle,
    /// `info.triangle.fill`
    InfoTriangleFill,
    /// `inset.filled.rectangle.and.person.filled.circle`
    InsetFilledRectangleAndPersonFilledCircle,
    /// `inset.filled.rectangle.and.person.filled.circle.fill`
    InsetFilledRectangleAndPersonFilledCircleFill,
    /// `questionmark.message.ar`
    QuestionmarkMessageAr,
    /// `questionmark.message.fill.ar`
    QuestionmarkMessageFillAr,
    /// `receipt`
    Receipt,
    /// `receipt.fill`
    ReceiptFill,
    /// `wand.and.outline`
    WandAndOutline,
    /// `wand.and.outline.inverse`
    WandAndOutlineInverse,
}

impl SfSymbolV6_2 {
    /// Returns the SF Symbol name string.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::AirpodsGen4 => "airpods.gen4",
            Self::AirpodsGen4ChargingcaseWireless => "airpods.gen4.chargingcase.wireless",
            Self::AirpodsGen4ChargingcaseWirelessFill => "airpods.gen4.chargingcase.wireless.fill",
            Self::AirpodsGen4Left => "airpods.gen4.left",
            Self::AirpodsGen4Right => "airpods.gen4.right",
            Self::AppleWritingTools => "apple.writing.tools",
            Self::ExclamationmarkTriangleTextPage => "exclamationmark.triangle.text.page",
            Self::ExclamationmarkTriangleTextPageFill => "exclamationmark.triangle.text.page.fill",
            Self::ExclamationmarkTriangleTextPageFillRtl => "exclamationmark.triangle.text.page.fill.rtl",
            Self::ExclamationmarkTriangleTextPageRtl => "exclamationmark.triangle.text.page.rtl",
            Self::FigureSeatedSideLeftAirDistributionUpperAndMiddleAndLower => "figure.seated.side.left.air.distribution.upper.and.middle.and.lower",
            Self::FigureSeatedSideRightAirDistributionUpperAndMiddleAndLower => "figure.seated.side.right.air.distribution.upper.and.middle.and.lower",
            Self::HeadphonesDots => "headphones.dots",
            Self::InfoTriangle => "info.triangle",
            Self::InfoTriangleFill => "info.triangle.fill",
            Self::InsetFilledRectangleAndPersonFilledCircle => "inset.filled.rectangle.and.person.filled.circle",
            Self::InsetFilledRectangleAndPersonFilledCircleFill => "inset.filled.rectangle.and.person.filled.circle.fill",
            Self::QuestionmarkMessageAr => "questionmark.message.ar",
            Self::QuestionmarkMessageFillAr => "questionmark.message.fill.ar",
            Self::Receipt => "receipt",
            Self::ReceiptFill => "receipt.fill",
            Self::WandAndOutline => "wand.and.outline",
            Self::WandAndOutlineInverse => "wand.and.outline.inverse",
        }
    }
}

impl AsRef<str> for SfSymbolV6_2 {
    #[inline]
    fn as_ref(&self) -> &str {
        self.name()
    }
}

impl std::fmt::Display for SfSymbolV6_2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
