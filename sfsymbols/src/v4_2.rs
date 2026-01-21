//! Auto-generated SF Symbols enum.
//! Do not edit manually - regenerate with codegen.

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum SfSymbolV4_2 {
    /// `axle.2`
    Axle2,
    /// `axle.2.front.and.rear.engaged`
    Axle2FrontAndRearEngaged,
    /// `axle.2.front.engaged`
    Axle2FrontEngaged,
    /// `axle.2.rear.engaged`
    Axle2RearEngaged,
    /// `beats.powerbeats3.left`
    BeatsPowerbeats3Left,
    /// `beats.powerbeats3.right`
    BeatsPowerbeats3Right,
    /// `beats.powerbeats.left`
    BeatsPowerbeatsLeft,
    /// `beats.powerbeats.right`
    BeatsPowerbeatsRight,
    /// `10.lane.ar`
    N10LaneAr,
    /// `10.lane.hi`
    N10LaneHi,
    /// `11.lane.ar`
    N11LaneAr,
    /// `11.lane.hi`
    N11LaneHi,
    /// `12.lane.ar`
    N12LaneAr,
    /// `12.lane.hi`
    N12LaneHi,
    /// `1.lane.ar`
    N1LaneAr,
    /// `1.lane.hi`
    N1LaneHi,
    /// `2.lane.ar`
    N2LaneAr,
    /// `2.lane.hi`
    N2LaneHi,
    /// `3.lane.ar`
    N3LaneAr,
    /// `3.lane.hi`
    N3LaneHi,
    /// `4.lane.ar`
    N4LaneAr,
    /// `4.lane.hi`
    N4LaneHi,
    /// `5.lane.ar`
    N5LaneAr,
    /// `5.lane.hi`
    N5LaneHi,
    /// `6.lane.ar`
    N6LaneAr,
    /// `6.lane.hi`
    N6LaneHi,
    /// `7.lane.ar`
    N7LaneAr,
    /// `7.lane.hi`
    N7LaneHi,
    /// `8.lane.ar`
    N8LaneAr,
    /// `8.lane.hi`
    N8LaneHi,
    /// `9.lane.ar`
    N9LaneAr,
    /// `9.lane.hi`
    N9LaneHi,
}

impl SfSymbolV4_2 {
    /// Returns the SF Symbol name string.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Axle2 => "axle.2",
            Self::Axle2FrontAndRearEngaged => "axle.2.front.and.rear.engaged",
            Self::Axle2FrontEngaged => "axle.2.front.engaged",
            Self::Axle2RearEngaged => "axle.2.rear.engaged",
            Self::BeatsPowerbeats3Left => "beats.powerbeats3.left",
            Self::BeatsPowerbeats3Right => "beats.powerbeats3.right",
            Self::BeatsPowerbeatsLeft => "beats.powerbeats.left",
            Self::BeatsPowerbeatsRight => "beats.powerbeats.right",
            Self::N10LaneAr => "10.lane.ar",
            Self::N10LaneHi => "10.lane.hi",
            Self::N11LaneAr => "11.lane.ar",
            Self::N11LaneHi => "11.lane.hi",
            Self::N12LaneAr => "12.lane.ar",
            Self::N12LaneHi => "12.lane.hi",
            Self::N1LaneAr => "1.lane.ar",
            Self::N1LaneHi => "1.lane.hi",
            Self::N2LaneAr => "2.lane.ar",
            Self::N2LaneHi => "2.lane.hi",
            Self::N3LaneAr => "3.lane.ar",
            Self::N3LaneHi => "3.lane.hi",
            Self::N4LaneAr => "4.lane.ar",
            Self::N4LaneHi => "4.lane.hi",
            Self::N5LaneAr => "5.lane.ar",
            Self::N5LaneHi => "5.lane.hi",
            Self::N6LaneAr => "6.lane.ar",
            Self::N6LaneHi => "6.lane.hi",
            Self::N7LaneAr => "7.lane.ar",
            Self::N7LaneHi => "7.lane.hi",
            Self::N8LaneAr => "8.lane.ar",
            Self::N8LaneHi => "8.lane.hi",
            Self::N9LaneAr => "9.lane.ar",
            Self::N9LaneHi => "9.lane.hi",
        }
    }
}

impl AsRef<str> for SfSymbolV4_2 {
    #[inline]
    fn as_ref(&self) -> &str {
        self.name()
    }
}

impl std::fmt::Display for SfSymbolV4_2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}
