use serde_repr::{Deserialize_repr, Serialize_repr};

/// The various types of creative attributes.
///
/// Creative attributes that describe ad creatives in detail. They can be used to
/// indicate restrictions on what kinds of creatives can be displayed.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum CreativeAttribute {
    /// Audio ad (autoplay)
    AudioAuto = 1,

    /// Audio ad (user initiated)
    AudioUser = 2,

    /// Expandable (automatic)
    ExpandableAuto = 3,

    /// Expandable (user initiated - click)
    ExpandableClick = 4,

    /// Expandable (user initiated - rollover)
    ExpandableRollover = 5,

    /// In-banner video ad (autoplay)
    VideoBannerAuto = 6,

    /// In-banner video ad (user initiated)
    VideoBannerUser = 7,

    /// Pop (e.g., over, under, or upon exit)
    Pop = 8,

    /// Provocative or suggestive imagery
    Provocative = 9,

    /// Shaky, flashing, flickering, extreme animation, smileys
    Annoying = 10,

    /// Surveys
    Surveys = 11,

    /// Text only
    TextOnly = 12,

    /// User interactive (e.g., embedded games)
    UserInteractive = 13,

    /// Windows dialog or alert style
    Alert = 14,

    /// Has audio on/off button
    AudioOnOffButton = 15,

    /// Ad can be skipped (e.g., skip button)
    Skippable = 16,

    /// Adobe Flash
    AdobeFlash = 17,
}
