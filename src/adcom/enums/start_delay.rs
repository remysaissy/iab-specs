/// Start delay modes for video/audio ad placement.
///
/// Indicates the start delay in seconds for pre-roll, mid-roll, or post-roll ad placements.
/// Positive values represent the exact start time in seconds (mid-roll).
/// Note: This uses i8 to support negative values for generic positions.
#[derive(serde::Serialize, serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct StartDelay(pub i32);

impl StartDelay {
    /// Pre-roll (start delay = 0 seconds)
    pub const PRE_ROLL: StartDelay = StartDelay(0);

    /// Generic mid-roll (position unknown)
    pub const GENERIC_MID_ROLL: StartDelay = StartDelay(-1);

    /// Generic post-roll (position unknown)
    pub const GENERIC_POST_ROLL: StartDelay = StartDelay(-2);

    /// Create a mid-roll with specific start time in seconds (> 0)
    pub const fn mid_roll(seconds: i32) -> Self {
        StartDelay(seconds)
    }

    /// Check if this is pre-roll
    pub const fn is_pre_roll(&self) -> bool {
        self.0 == 0
    }

    /// Check if this is mid-roll (positive value)
    pub const fn is_mid_roll(&self) -> bool {
        self.0 > 0
    }

    /// Check if this is post-roll
    pub const fn is_post_roll(&self) -> bool {
        self.0 == -2
    }
}
