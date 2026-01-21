//! Auto-generated SF Symbols enum.
//! Do not edit manually - regenerate with codegen.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SfSymbolV3_2 {
    /// `airpod.gen3.left`
    AirpodGen3Left,
    /// `airpod.gen3.right`
    AirpodGen3Right,
    /// `airpods.gen3`
    AirpodsGen3,
    /// `airpods.gen3.chargingcase.wireless`
    AirpodsGen3ChargingcaseWireless,
    /// `airpods.gen3.chargingcase.wireless.fill`
    AirpodsGen3ChargingcaseWirelessFill,
    /// `beats.fit.pro`
    BeatsFitPro,
    /// `beats.fit.pro.chargingcase`
    BeatsFitProChargingcase,
    /// `beats.fit.pro.chargingcase.fill`
    BeatsFitProChargingcaseFill,
    /// `beats.fit.pro.left`
    BeatsFitProLeft,
    /// `beats.fit.pro.right`
    BeatsFitProRight,
    /// `rectangle.leadinghalf.filled`
    RectangleLeadinghalfFilled,
    /// `rectangle.trailinghalf.filled`
    RectangleTrailinghalfFilled,
    /// `square.3.layers.3d.down.left.slash`
    Square3Layers3DDownLeftSlash,
    /// `square.3.layers.3d.down.right.slash`
    Square3Layers3DDownRightSlash,
    /// `square.3.stack.3d.slash`
    Square3Stack3DSlash,
}

impl SfSymbolV3_2 {
    /// Returns the SF Symbol name string.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::AirpodGen3Left => "airpod.gen3.left",
            Self::AirpodGen3Right => "airpod.gen3.right",
            Self::AirpodsGen3 => "airpods.gen3",
            Self::AirpodsGen3ChargingcaseWireless => "airpods.gen3.chargingcase.wireless",
            Self::AirpodsGen3ChargingcaseWirelessFill => "airpods.gen3.chargingcase.wireless.fill",
            Self::BeatsFitPro => "beats.fit.pro",
            Self::BeatsFitProChargingcase => "beats.fit.pro.chargingcase",
            Self::BeatsFitProChargingcaseFill => "beats.fit.pro.chargingcase.fill",
            Self::BeatsFitProLeft => "beats.fit.pro.left",
            Self::BeatsFitProRight => "beats.fit.pro.right",
            Self::RectangleLeadinghalfFilled => "rectangle.leadinghalf.filled",
            Self::RectangleTrailinghalfFilled => "rectangle.trailinghalf.filled",
            Self::Square3Layers3DDownLeftSlash => "square.3.layers.3d.down.left.slash",
            Self::Square3Layers3DDownRightSlash => "square.3.layers.3d.down.right.slash",
            Self::Square3Stack3DSlash => "square.3.stack.3d.slash",
        }
    }
}

impl AsRef<str> for SfSymbolV3_2 {
    #[inline]
    fn as_ref(&self) -> &str {
        self.name()
    }
}

impl std::fmt::Display for SfSymbolV3_2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
