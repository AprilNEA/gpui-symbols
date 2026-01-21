//! Auto-generated SF Symbols enum.
//! Do not edit manually - regenerate with codegen.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SfSymbolV3_3 {
    /// `camera.macro`
    CameraMacro,
    /// `camera.macro.circle`
    CameraMacroCircle,
    /// `camera.macro.circle.fill`
    CameraMacroCircleFill,
    /// `dots.and.line.vertical.and.cursorarrow.rectangle`
    DotsAndLineVerticalAndCursorarrowRectangle,
    /// `key.viewfinder`
    KeyViewfinder,
    /// `person.badge.key`
    PersonBadgeKey,
    /// `person.badge.key.fill`
    PersonBadgeKeyFill,
}

impl SfSymbolV3_3 {
    /// Returns the SF Symbol name string.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::CameraMacro => "camera.macro",
            Self::CameraMacroCircle => "camera.macro.circle",
            Self::CameraMacroCircleFill => "camera.macro.circle.fill",
            Self::DotsAndLineVerticalAndCursorarrowRectangle => "dots.and.line.vertical.and.cursorarrow.rectangle",
            Self::KeyViewfinder => "key.viewfinder",
            Self::PersonBadgeKey => "person.badge.key",
            Self::PersonBadgeKeyFill => "person.badge.key.fill",
        }
    }
}

impl AsRef<str> for SfSymbolV3_3 {
    #[inline]
    fn as_ref(&self) -> &str {
        self.name()
    }
}

impl std::fmt::Display for SfSymbolV3_3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
