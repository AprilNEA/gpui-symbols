//! Auto-generated SF Symbols enum.
//! Do not edit manually - regenerate with codegen.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SfSymbolV3_1 {
    /// `bolt.ring.closed`
    BoltRingClosed,
    /// `platter.filled.bottom.and.arrow.down.iphone`
    PlatterFilledBottomAndArrowDownIphone,
    /// `platter.filled.bottom.iphone`
    PlatterFilledBottomIphone,
    /// `platter.filled.top.and.arrow.up.iphone`
    PlatterFilledTopAndArrowUpIphone,
    /// `platter.filled.top.iphone`
    PlatterFilledTopIphone,
    /// `square.3.layers.3d.down.backward`
    Square3Layers3DDownBackward,
    /// `square.3.layers.3d.down.forward`
    Square3Layers3DDownForward,
    /// `square.3.layers.3d.down.left`
    Square3Layers3DDownLeft,
    /// `square.3.layers.3d.down.right`
    Square3Layers3DDownRight,
    /// `text.justify.leading`
    TextJustifyLeading,
    /// `text.justify.left`
    TextJustifyLeft,
    /// `text.justify.right`
    TextJustifyRight,
    /// `text.justify.trailing`
    TextJustifyTrailing,
}

impl SfSymbolV3_1 {
    /// Returns the SF Symbol name string.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::BoltRingClosed => "bolt.ring.closed",
            Self::PlatterFilledBottomAndArrowDownIphone => "platter.filled.bottom.and.arrow.down.iphone",
            Self::PlatterFilledBottomIphone => "platter.filled.bottom.iphone",
            Self::PlatterFilledTopAndArrowUpIphone => "platter.filled.top.and.arrow.up.iphone",
            Self::PlatterFilledTopIphone => "platter.filled.top.iphone",
            Self::Square3Layers3DDownBackward => "square.3.layers.3d.down.backward",
            Self::Square3Layers3DDownForward => "square.3.layers.3d.down.forward",
            Self::Square3Layers3DDownLeft => "square.3.layers.3d.down.left",
            Self::Square3Layers3DDownRight => "square.3.layers.3d.down.right",
            Self::TextJustifyLeading => "text.justify.leading",
            Self::TextJustifyLeft => "text.justify.left",
            Self::TextJustifyRight => "text.justify.right",
            Self::TextJustifyTrailing => "text.justify.trailing",
        }
    }
}

impl AsRef<str> for SfSymbolV3_1 {
    #[inline]
    fn as_ref(&self) -> &str {
        self.name()
    }
}

impl std::fmt::Display for SfSymbolV3_1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
