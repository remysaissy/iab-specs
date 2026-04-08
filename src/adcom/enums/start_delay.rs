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

#[cfg(test)]
mod tests {
    use super::*;

    /// AdCOM 1.0 Table: Start Delay — PRE_ROLL constant is 0
    #[test]
    fn test_constant_pre_roll() {
        assert_eq!(StartDelay::PRE_ROLL.0, 0);
    }

    /// AdCOM 1.0 Table: Start Delay — GENERIC_MID_ROLL constant is -1
    #[test]
    fn test_constant_generic_mid_roll() {
        assert_eq!(StartDelay::GENERIC_MID_ROLL.0, -1);
    }

    /// AdCOM 1.0 Table: Start Delay — GENERIC_POST_ROLL constant is -2
    #[test]
    fn test_constant_generic_post_roll() {
        assert_eq!(StartDelay::GENERIC_POST_ROLL.0, -2);
    }

    /// AdCOM 1.0 Table: Start Delay — mid_roll() creates a StartDelay with positive seconds
    #[test]
    fn test_mid_roll_constructor() {
        let delay = StartDelay::mid_roll(30);
        assert_eq!(delay.0, 30);
    }

    /// AdCOM 1.0 Table: Start Delay — is_pre_roll() returns true only for 0
    #[test]
    fn test_is_pre_roll() {
        assert!(StartDelay::PRE_ROLL.is_pre_roll());
        assert!(!StartDelay::GENERIC_MID_ROLL.is_pre_roll());
        assert!(!StartDelay::GENERIC_POST_ROLL.is_pre_roll());
        assert!(!StartDelay::mid_roll(30).is_pre_roll());
    }

    /// AdCOM 1.0 Table: Start Delay — is_mid_roll() returns true only for positive values
    #[test]
    fn test_is_mid_roll() {
        assert!(StartDelay::mid_roll(30).is_mid_roll());
        assert!(StartDelay::mid_roll(1).is_mid_roll());
        assert!(!StartDelay::PRE_ROLL.is_mid_roll());
        assert!(!StartDelay::GENERIC_MID_ROLL.is_mid_roll());
        assert!(!StartDelay::GENERIC_POST_ROLL.is_mid_roll());
    }

    /// AdCOM 1.0 Table: Start Delay — is_post_roll() returns true only for -2
    #[test]
    fn test_is_post_roll() {
        assert!(StartDelay::GENERIC_POST_ROLL.is_post_roll());
        assert!(!StartDelay::PRE_ROLL.is_post_roll());
        assert!(!StartDelay::GENERIC_MID_ROLL.is_post_roll());
        assert!(!StartDelay::mid_roll(30).is_post_roll());
    }

    /// AdCOM 1.0 Table: Start Delay — serde roundtrip for all key values
    #[test]
    fn test_serialization_roundtrip() {
        let values = [
            StartDelay::PRE_ROLL,
            StartDelay::GENERIC_MID_ROLL,
            StartDelay::GENERIC_POST_ROLL,
            StartDelay::mid_roll(30),
            StartDelay::mid_roll(120),
        ];

        for original in values {
            let json = serde_json::to_string(&original).unwrap();
            let deserialized: StartDelay = serde_json::from_str(&json).unwrap();
            assert_eq!(
                original, deserialized,
                "Serialization roundtrip failed for {:?}",
                original
            );
        }
    }
}
