//! AdCOM (Advertising Common Object Model) Enumerations
//!
//! This module contains standardized enumeration lists from the AdCOM v1.0 specification.
//! These enumerations are used across multiple advertising protocols including OpenRTB 2.x/3.x.
//!
//! AdCOM enumerations provide consistent value definitions for:
//! - Auction mechanics (auction types, no-bid reasons)
//! - Media specifications (video protocols, API frameworks)
//! - Device characteristics (device types, connection types)
//! - Content classification (contexts, quality ratings)
//! - Creative attributes (expandable, skippable, etc.)
//!
//! Reference: <https://github.com/InteractiveAdvertisingBureau/AdCOM>

use serde_repr::{Deserialize_repr, Serialize_repr};

/// Auction type, where 1 = First Price, 2 = Second Price Plus, 3 = the value passed in
/// bidfloor is the agreed upon deal price.
///
/// Additional auction types can be defined by the exchange.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AuctionType {
    /// First price auction
    FirstPrice = 1,

    /// Second price plus auction (default)
    SecondPricePlus = 2,

    /// Fixed price specified in bidfloor attribute
    FixedPrice = 3,
}

impl Default for AuctionType {
    fn default() -> Self {
        AuctionType::SecondPricePlus
    }
}

/// The position of the ad as a relative measure of visibility or prominence.
///
/// This OpenRTB list has values derived from the Inventory Quality Guidelines (IQG).
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AdPosition {
    /// Unknown position
    Unknown = 0,

    /// Above the fold
    AboveTheFold = 1,

    /// May or may not be initially visible (deprecated by OpenRTB)
    #[deprecated(note = "Use Unknown or other appropriate value")]
    MayNotBeVisible = 2,

    /// Below the fold
    BelowTheFold = 3,

    /// Header
    Header = 4,

    /// Footer
    Footer = 5,

    /// Sidebar
    Sidebar = 6,

    /// Full screen
    FullScreen = 7,
}

impl Default for AdPosition {
    fn default() -> Self {
        AdPosition::Unknown
    }
}

/// API frameworks supported by the publisher.
///
/// Note that MRAID-1, MRAID-2, and MRAID-3 are numbered 3, 5, and 6 since it was
/// determined that their predecessors, values 1 and 2, were duplicates as the
/// VPAID 1.0 and VPAID 2.0 specifications are inherently HTML5 compliant.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ApiFramework {
    /// VPAID 1.0
    Vpaid1 = 1,

    /// VPAID 2.0
    Vpaid2 = 2,

    /// MRAID-1
    Mraid1 = 3,

    /// ORMMA
    Ormma = 4,

    /// MRAID-2
    Mraid2 = 5,

    /// MRAID-3
    Mraid3 = 6,

    /// OMID-1
    Omid1 = 7,

    /// SIMID-1
    Simid1 = 8,

    /// SIMID-1.1
    Simid1_1 = 9,
}

/// Video or audio protocols supported.
///
/// OpenRTB version 2.5 list. VAST versions are numbered in a sub-range to distinguish
/// from other protocol values. DAAST is included for audio ads. OpenRTB 2.6 adds support
/// for VAST 4.2 and 4.3.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Protocol {
    /// VAST 1.0
    Vast1 = 1,

    /// VAST 2.0
    Vast2 = 2,

    /// VAST 3.0
    Vast3 = 3,

    /// VAST 1.0 Wrapper
    Vast1Wrapper = 4,

    /// VAST 2.0 Wrapper
    Vast2Wrapper = 5,

    /// VAST 3.0 Wrapper
    Vast3Wrapper = 6,

    /// VAST 4.0
    Vast4 = 7,

    /// VAST 4.0 Wrapper
    Vast4Wrapper = 8,

    /// DAAST 1.0
    Daast1 = 9,

    /// DAAST 1.0 Wrapper
    Daast1Wrapper = 10,

    /// VAST 4.1
    Vast4_1 = 11,

    /// VAST 4.1 Wrapper
    Vast4_1Wrapper = 12,

    /// VAST 4.2
    Vast4_2 = 13,

    /// VAST 4.2 Wrapper
    Vast4_2Wrapper = 14,
}

/// Video linearity: "in-stream" or "linear" video refers to pre-roll, mid-roll, and
/// post-roll video ads where the user must watch the ad before viewing the content.
/// Nonlinear refers to video ads that overlay content and may not necessarily interrupt
/// streaming content.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum VideoLinearity {
    /// Linear / In-stream
    Linear = 1,

    /// Non-linear / Overlay
    NonLinear = 2,
}

/// Playback methods available for video inventory.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PlaybackMethod {
    /// Initiates on page load with sound on
    AutoPlaySoundOn = 1,

    /// Initiates on page load with sound off by default
    AutoPlaySoundOff = 2,

    /// Initiates on click with sound on
    ClickToPlay = 3,

    /// Initiates on mouse-over with sound on
    MouseOver = 4,

    /// Initiates on entering viewport with sound on
    EnterViewportSoundOn = 5,

    /// Initiates on entering viewport with sound off by default
    EnterViewportSoundOff = 6,
}

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

/// Type of device from which the impression originates.
///
/// OpenRTB version 2.2 of the specification added distinct values for Mobile and Tablet.
/// It is recommended that any bidder with differentiation in their campaign-creative
/// management systems between these 2 device types properly determine and use these types.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DeviceType {
    /// Mobile/Tablet - General (deprecated, use specific types)
    #[deprecated(note = "Use Mobile or Tablet")]
    MobileTablet = 1,

    /// Personal Computer
    PersonalComputer = 2,

    /// Connected TV
    ConnectedTv = 3,

    /// Phone
    Phone = 4,

    /// Tablet
    Tablet = 5,

    /// Connected Device
    ConnectedDevice = 6,

    /// Set Top Box
    SetTopBox = 7,

    /// Out of Home (OOH) Device
    OutOfHome = 8,
}

/// Type of connection.
///
/// The various options for the type of device connectivity.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ConnectionType {
    /// Unknown
    Unknown = 0,

    /// Ethernet
    Ethernet = 1,

    /// WIFI
    Wifi = 2,

    /// Cellular Network - Unknown Generation
    CellularUnknown = 3,

    /// Cellular Network - 2G
    Cellular2G = 4,

    /// Cellular Network - 3G
    Cellular3G = 5,

    /// Cellular Network - 4G
    Cellular4G = 6,

    /// Cellular Network - 5G
    Cellular5G = 7,
}

/// Type of content being displayed.
///
/// The nature of the content on the site, app, or other property.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ContentContext {
    /// Video (i.e., video file or stream such as Internet TV broadcasts)
    Video = 1,

    /// Game (i.e., an interactive software game)
    Game = 2,

    /// Music (i.e., audio file or stream such as Internet radio broadcasts)
    Music = 3,

    /// Application (i.e., an interactive software application)
    Application = 4,

    /// Text (i.e., primarily textual document such as a web page, eBook, or news article)
    Text = 5,

    /// Other (i.e., none of the other categories applies)
    Other = 6,

    /// Unknown
    Unknown = 7,
}

/// Quality of content.
///
/// This enum is deprecated in favor of prodq below. See content object.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ContentQuality {
    /// Unknown
    Unknown = 0,

    /// Professionally Produced
    Professional = 1,

    /// Prosumer
    Prosumer = 2,

    /// User Generated (UGC)
    UserGenerated = 3,
}

/// Production quality.
///
/// The production quality of the content.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ProductionQuality {
    /// Unknown
    Unknown = 0,

    /// Professionally Produced
    Professional = 1,

    /// Prosumer
    Prosumer = 2,

    /// User Generated (UGC)
    UserGenerated = 3,
}

/// Options for the video content and ad play mode.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PlaybackCessationMode {
    /// On video completion or when user exits
    OnCompletion = 1,

    /// On page exit
    OnExit = 2,

    /// On float
    OnFloat = 3,
}

/// No-Bid Reason Codes.
///
/// The following table lists the options for a bidder to signal the exchange as to why
/// it did not bid on the impression.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum NoBidReason {
    /// Unknown Error
    UnknownError = 0,

    /// Technical Error
    TechnicalError = 1,

    /// Invalid Request
    InvalidRequest = 2,

    /// Known Web Spider
    KnownSpider = 3,

    /// Suspected Non-Human Traffic
    SuspectedNonHuman = 4,

    /// Cloud, Data Center, or Proxy IP
    CloudDatacenterProxy = 5,

    /// Unsupported Device
    UnsupportedDevice = 6,

    /// Blocked Publisher or Site
    BlockedPublisher = 7,

    /// Unmatched User
    UnmatchedUser = 8,

    /// Daily Reader Cap Met
    DailyCapMet = 9,

    /// Daily Domain Cap Met
    DailyDomainCapMet = 10,
}

/// Loss reason codes.
///
/// The following table lists the possible codes for bid response loss reasons.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum LossReason {
    /// Bid Won
    BidWon = 0,

    /// Internal Error
    InternalError = 1,

    /// Impression Opportunity Expired
    Expired = 2,

    /// Invalid Bid Response
    InvalidBidResponse = 3,

    /// Invalid Deal ID
    InvalidDealId = 4,

    /// Invalid Auction ID
    InvalidAuctionId = 5,

    /// Invalid Advertiser Domain
    InvalidAdvertiserDomain = 6,

    /// Missing Markup
    MissingMarkup = 7,

    /// Missing Creative ID
    MissingCreativeId = 8,

    /// Missing Price
    MissingPrice = 9,

    /// Missing Minimum Creative Approval Data
    MissingCreativeApproval = 10,

    /// Bid was Below Auction Floor
    BelowFloor = 100,

    /// Bid was Below Deal Floor
    BelowDealFloor = 101,

    /// Lost to Higher Bid
    LostToHigherBid = 102,

    /// Lost to a Bid for a PMP Deal
    LostToPmp = 103,

    /// Buyer Seat Blocked
    SeatBlocked = 104,

    /// Creative Filtered - General
    CreativeFiltered = 200,

    /// Creative Filtered - Pending Processing
    CreativePending = 201,

    /// Creative Filtered - Disapproved
    CreativeDisapproved = 202,

    /// Creative Filtered - Size Not Allowed
    CreativeSizeNotAllowed = 203,

    /// Creative Filtered - Not Secure
    CreativeNotSecure = 204,

    /// Creative Filtered - Language Exclusions
    CreativeLanguageExcluded = 205,

    /// Creative Filtered - Category Exclusions
    CreativeCategoryExcluded = 206,

    /// Creative Filtered - Creative Attribute Exclusions
    CreativeAttributeExcluded = 207,

    /// Creative Filtered - Ad Type Exclusions
    CreativeAdTypeExcluded = 208,

    /// Creative Filtered - Animation Too Long
    CreativeAnimationTooLong = 209,

    /// Creative Filtered - Not Allowed in PMP Deal
    CreativeNotAllowedInPmp = 210,
}

/// Banner ad types.
///
/// The type of banner creative to be served using an AdUnit.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum BannerAdType {
    /// XHTML Text Ad (usually mobile)
    XhtmlTextAd = 1,

    /// XHTML Banner Ad (usually mobile)
    XhtmlBannerAd = 2,

    /// JavaScript Ad; must be valid XHTML (i.e., script tags included)
    JavaScriptAd = 3,

    /// iFrame
    IFrame = 4,
}

/// Video placement types.
///
/// These values are derived from the IAB's Digital Video Guidelines for programmatic
/// video advertising.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum VideoPlacementType {
    /// In-Stream: Played before, during or after the streaming video content that the
    /// consumer has requested (Pre-roll, Mid-roll, Post-roll).
    InStream = 1,

    /// In-Banner: Exists within a web banner that leverages the banner space to deliver
    /// a video experience as opposed to another static or rich media format.
    InBanner = 2,

    /// In-Article: Loads and plays dynamically between paragraphs of editorial content;
    /// existing as a standalone branded message.
    InArticle = 3,

    /// In-Feed: Found in content, social, or product feeds.
    InFeed = 4,

    /// Interstitial/Slider/Floating: Covers the entire or a portion of screen area, but
    /// is always on screen while displayed (i.e. cannot be scrolled out of view).
    Interstitial = 5,
}

/// Location type for geolocation.
///
/// Describes the source of location data.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum LocationType {
    /// GPS/Location Services
    GpsLocation = 1,

    /// IP Address
    IpAddress = 2,

    /// User Provided (e.g., registration data)
    UserProvided = 3,
}

/// Location service provider.
///
/// Source of the location service being used.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum LocationService {
    /// IP2Location
    Ip2Location = 1,

    /// Neustar (Quova)
    Neustar = 2,

    /// MaxMind
    MaxMind = 3,

    /// NetAcuity (Digital Element)
    NetAcuity = 4,
}

/// Feed types for audio content.
///
/// Type of audio feed.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum FeedType {
    /// Music Service
    MusicService = 1,

    /// FM/AM Broadcast
    Broadcast = 2,

    /// Podcast
    Podcast = 3,
}

/// Volume normalization modes.
///
/// Volume normalization modes for audio content.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum VolumeNormalizationMode {
    /// None
    None = 0,

    /// Ad Volume Average Normalized to Content
    AverageVolume = 1,

    /// Ad Volume Peak Normalized to Content
    PeakVolume = 2,

    /// Ad Loudness Normalized to Content
    Loudness = 3,

    /// Custom Volume Normalization
    Custom = 4,
}

/// Content delivery methods.
///
/// The various options for content delivery.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ContentDeliveryMethod {
    /// Streaming
    Streaming = 1,

    /// Progressive
    Progressive = 2,

    /// Download
    Download = 3,
}

/// IQG Media Ratings.
///
/// The content rating from the IAB Quality Assurance Guidelines (IQG) Taxonomy.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum QagMediaRating {
    /// All Audiences
    AllAudiences = 1,

    /// Everyone Over 12
    Over12 = 2,

    /// Mature Audiences (17+)
    Mature = 3,
}

/// Expandable direction.
///
/// Direction in which an expandable ad may expand.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ExpandableDirection {
    /// Left
    Left = 1,

    /// Right
    Right = 2,

    /// Up
    Up = 3,

    /// Down
    Down = 4,

    /// Full Screen
    FullScreen = 5,
}

/// DOOH Venue type taxonomy.
///
/// Taxonomy defining venue types for Digital Out-Of-Home advertising.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DoohVenueTaxonomy {
    /// AdCOM 1.0
    AdCom1 = 1,

    /// DPAA 2016
    Dpaa2016 = 2,

    /// DMI 2017
    Dmi2017 = 3,
}

/// Agent type.
///
/// Type of user agent, distinguishing between human users and automated agents.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AgentType {
    /// Human user
    Human = 1,

    /// Robot, crawler, or spider
    Robot = 2,
}

/// Audit status codes.
///
/// Status codes for creative audits and approval.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AuditStatusCode {
    /// Approved
    Approved = 1,

    /// Approved with changes
    ApprovedWithChanges = 2,

    /// Rejected
    Rejected = 3,

    /// Rejected for impressions (creative not served)
    RejectedForImpressions = 4,
}

/// Event tracking methods.
///
/// Methods for tracking ad events.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum EventTrackingMethod {
    /// Image-pixel tracking (1x1 pixel)
    ImagePixel = 1,

    /// JavaScript tracking
    JavaScript = 2,
}

/// Event types.
///
/// Types of ad-related events that can be tracked.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum EventType {
    /// Impression (ad rendered)
    Impression = 1,

    /// Viewable impression (meets viewability standard)
    ViewableImpression = 2,

    /// Click
    Click = 3,

    /// Ad expanded
    Expand = 4,

    /// Ad collapsed
    Collapse = 5,

    /// Creative loaded
    CreativeLoaded = 6,
}

/// Category taxonomy.
///
/// Taxonomy used for content categorization.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CategoryTaxonomy {
    /// IAB Content Category Taxonomy 1.0
    IabContentCategory1_0 = 1,

    /// IAB Content Category Taxonomy 2.0
    IabContentCategory2_0 = 2,

    /// IAB Ad Product Taxonomy 1.0
    IabAdProduct1_0 = 3,

    /// Publisher-specific proprietary taxonomy
    PublisherSpecific = 4,

    /// IAB Content Category Taxonomy 2.1
    IabContentCategory2_1 = 5,

    /// IAB Content Category Taxonomy 2.2
    IabContentCategory2_2 = 6,

    /// IAB Content Category Taxonomy 3.0
    IabContentCategory3_0 = 7,
}

/// Creative subtype for display ads.
///
/// More granular categorization of display creative types.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CreativeSubtypeDisplay {
    /// HTML banner
    HtmlBanner = 1,

    /// VAST tag for video
    Vast = 2,

    /// VPAID for interactive video
    Vpaid = 3,

    /// JavaScript tag
    JavaScript = 4,

    /// iFrame
    IFrame = 5,
}

/// Creative subtype for audio/video ads.
///
/// Categorization of audio and video creative formats.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CreativeSubtypeAudioVideo {
    /// VAST (Video Ad Serving Template)
    Vast = 1,

    /// DAAST (Digital Audio Ad Serving Template)
    Daast = 2,

    /// VPAID (Video Player-Ad Interface Definition)
    Vpaid = 3,

    /// Proprietary format
    Proprietary = 4,
}

/// Display context type.
///
/// Context in which a display ad appears.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DisplayContextType {
    /// Content-centric context (e.g., newsfeed, article)
    ContentCentric = 1,

    /// Social-centric context (e.g., social network feed)
    SocialCentric = 2,

    /// Product context (e.g., product details, reviews)
    ProductContext = 3,
}

/// Native data asset types.
///
/// Types of data assets in native ads.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum NativeDataAssetType {
    /// Sponsored by message
    Sponsored = 1,

    /// Descriptive text
    Description = 2,

    /// Rating (e.g., 5 stars)
    Rating = 3,

    /// Number of likes
    Likes = 4,

    /// Number of downloads
    Downloads = 5,

    /// Product price
    Price = 6,

    /// Sale price (discounted)
    SalePrice = 7,

    /// Phone number
    Phone = 8,

    /// Address
    Address = 9,

    /// Additional descriptive text
    Description2 = 10,

    /// Display URL
    DisplayUrl = 11,

    /// Call to action text
    CallToAction = 12,
}

/// Native image asset types.
///
/// Types of image assets in native ads.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum NativeImageAssetType {
    /// Icon image (typically small, square)
    Icon = 1,

    /// Logo image
    Logo = 2,

    /// Large image (main creative image)
    Main = 3,
}

/// Operating systems.
///
/// Operating system of the device.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum OperatingSystem {
    /// Apple iOS
    IOS = 1,

    /// Google Android
    Android = 2,

    /// Microsoft Windows
    Windows = 3,

    /// Apple macOS
    MacOS = 4,

    /// Linux
    Linux = 5,

    /// Other/Unknown
    Other = 6,
}

/// Click type.
///
/// Types of ad click behavior.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ClickType {
    /// Non-clickable
    NonClickable = 0,

    /// Clickable
    Clickable = 1,

    /// Clickable with embedded browser
    EmbeddedBrowser = 2,
}

/// Companion type.
///
/// Types of companion ads that can accompany video/audio.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CompanionType {
    /// Static resource
    Static = 0,

    /// HTML resource
    Html = 1,

    /// iFrame resource
    IFrame = 2,
}

/// Display placement type.
///
/// General type or context of the display placement.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DisplayPlacementType {
    /// In-feed placement (e.g., newsfeed, content stream)
    InFeed = 1,

    /// Sidebar placement
    Sidebar = 2,

    /// Interstitial/Overlay placement
    Interstitial = 3,

    /// Floating placement
    Floating = 4,
}

/// Placement position.
///
/// Ad position on screen (may duplicate AdPosition for legacy reasons).
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PlacementPosition {
    /// Unknown
    Unknown = 0,

    /// Above the fold
    AboveTheFold = 1,

    /// Below the fold
    BelowTheFold = 3,

    /// Header
    Header = 4,

    /// Footer
    Footer = 5,

    /// Sidebar
    Sidebar = 6,

    /// Full screen
    FullScreen = 7,
}

/// Pod deduplication.
///
/// Deduplication method for ad pods.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum PodDeduplication {
    /// Unknown/not specified
    Unknown = 0,

    /// No deduplication
    None = 1,

    /// Deduplicate by creative ID
    ByCreativeId = 2,

    /// Deduplicate by advertiser domain
    ByAdvertiserDomain = 3,
}

/// Pod sequence.
///
/// Position of ad within a pod.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(i8)]
pub enum PodSequence {
    /// Unknown
    Unknown = 0,

    /// First ad in pod
    First = 1,

    /// Last ad in pod
    Last = 2,

    /// Middle ad in pod
    Middle = 3,

    /// Only ad in pod
    Only = 4,
}

/// Size unit.
///
/// Units of measurement for sizes.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum SizeUnit {
    /// Device Independent Pixels (DIPS)
    Dips = 1,

    /// Physical pixels
    Pixels = 2,
}

/// Video placement subtype.
///
/// More specific video placement types.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum VideoPlacementSubtype {
    /// In-stream placement (pre/mid/post-roll)
    InStream = 1,

    /// In-banner video
    InBanner = 2,

    /// In-article video
    InArticle = 3,

    /// In-feed video
    InFeed = 4,

    /// Interstitial/floating video
    Interstitial = 5,
}

/// Auto-refresh trigger.
///
/// Trigger that causes a placement to auto-refresh.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum AutoRefreshTrigger {
    /// User-initiated refresh
    UserInitiated = 1,

    /// Time-based expiration
    TimeExpiration = 2,

    /// Scroll-based refresh
    Scroll = 3,
}

/// User agent source.
///
/// Source of the user agent string.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum UserAgentSource {
    /// Unknown source
    Unknown = 0,

    /// User-agent HTTP header
    HttpHeader = 1,

    /// Client hints
    ClientHints = 2,

    /// Server-side detection
    ServerSide = 3,
}

/// Device interface orientation.
///
/// The orientation of the device when the ad is shown.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DeviceOrientation {
    /// Portrait orientation
    Portrait = 0,

    /// Landscape orientation
    Landscape = 1,
}

impl Default for DeviceOrientation {
    fn default() -> Self {
        DeviceOrientation::Portrait
    }
}

/// Local market identifier types.
///
/// Designates the local market/DMA provider (Nielsen, Kantar, etc.).
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum LocalMarketIdentifierType {
    /// Nielsen DMA
    Nielsen = 1,

    /// Kantar
    Kantar = 2,
}

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

/// Slot position within an ad pod.
///
/// Indicates the position of the individual ad slot within an ad pod for video/audio.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(i8)]
pub enum SlotPosition {
    /// Last ad in the pod
    Last = -1,

    /// Any other position (middle of pod)
    Any = 0,

    /// First ad in the pod
    First = 1,

    /// First or last position in the pod
    FirstOrLast = 2,
}

/// ID matching methods for user identification.
///
/// Indicates the method used to match a user ID across different contexts.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum IDMatchMethod {
    /// No matching - ID came directly from 3rd-party cookie or device IFA
    NoMatching = 0,

    /// First-party observation without user authentication
    FirstParty = 1,

    /// Probabilistic matching based on non-authenticated features
    Probabilistic = 2,

    /// Deterministic matching with user authentication
    Deterministic = 3,
}

/// DOOH multiplier measurement source types.
///
/// Identifies the entity providing quantity measurement for impression multipliers
/// in Digital Out-of-Home advertising.
#[derive(Serialize_repr, Deserialize_repr, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum DOOHMultiplierMeasurementSource {
    /// Unknown source
    Unknown = 0,

    /// Measurement vendor provided
    MeasurementVendor = 1,

    /// Publisher provided
    Publisher = 2,

    /// Exchange provided
    Exchange = 3,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auction_type_default() {
        assert_eq!(AuctionType::default(), AuctionType::SecondPricePlus);
    }

    #[test]
    fn test_auction_type_serialization() {
        let at = AuctionType::FirstPrice;
        let json = serde_json::to_string(&at).unwrap();
        assert_eq!(json, "1");

        let at2: AuctionType = serde_json::from_str(&json).unwrap();
        assert_eq!(at, at2);
    }

    #[test]
    fn test_auction_type_deserialization() {
        let json = "2";
        let at: AuctionType = serde_json::from_str(json).unwrap();
        assert_eq!(at, AuctionType::SecondPricePlus);
    }

    #[test]
    fn test_auction_type_invalid() {
        let json = "99";
        let result: Result<AuctionType, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_ad_position_default() {
        assert_eq!(AdPosition::default(), AdPosition::Unknown);
    }

    #[test]
    fn test_ad_position_roundtrip() {
        let positions = vec![
            AdPosition::Unknown,
            AdPosition::AboveTheFold,
            AdPosition::BelowTheFold,
            AdPosition::Header,
            AdPosition::Footer,
            AdPosition::Sidebar,
            AdPosition::FullScreen,
        ];

        for pos in positions {
            let json = serde_json::to_string(&pos).unwrap();
            let pos2: AdPosition = serde_json::from_str(&json).unwrap();
            assert_eq!(pos, pos2);
        }
    }

    #[test]
    fn test_api_framework_serialization() {
        assert_eq!(serde_json::to_string(&ApiFramework::Vpaid1).unwrap(), "1");
        assert_eq!(serde_json::to_string(&ApiFramework::Mraid1).unwrap(), "3");
        assert_eq!(serde_json::to_string(&ApiFramework::Omid1).unwrap(), "7");
    }

    #[test]
    fn test_protocol_serialization() {
        assert_eq!(serde_json::to_string(&Protocol::Vast1).unwrap(), "1");
        assert_eq!(serde_json::to_string(&Protocol::Vast4_2).unwrap(), "13");
    }

    #[test]
    fn test_video_linearity() {
        let linear = VideoLinearity::Linear;
        let json = serde_json::to_string(&linear).unwrap();
        assert_eq!(json, "1");

        let nonlinear = VideoLinearity::NonLinear;
        let json = serde_json::to_string(&nonlinear).unwrap();
        assert_eq!(json, "2");
    }

    #[test]
    fn test_playback_method() {
        let pm = PlaybackMethod::AutoPlaySoundOn;
        let json = serde_json::to_string(&pm).unwrap();
        assert_eq!(json, "1");

        let pm2: PlaybackMethod = serde_json::from_str(&json).unwrap();
        assert_eq!(pm, pm2);
    }

    #[test]
    fn test_creative_attribute() {
        let ca = CreativeAttribute::AudioAuto;
        let json = serde_json::to_string(&ca).unwrap();
        assert_eq!(json, "1");

        let ca2 = CreativeAttribute::Skippable;
        let json2 = serde_json::to_string(&ca2).unwrap();
        assert_eq!(json2, "16");
    }

    #[test]
    fn test_device_type() {
        let dt = DeviceType::Phone;
        let json = serde_json::to_string(&dt).unwrap();
        assert_eq!(json, "4");

        let dt2: DeviceType = serde_json::from_str(&json).unwrap();
        assert_eq!(dt, dt2);
    }

    #[test]
    fn test_connection_type() {
        let ct = ConnectionType::Wifi;
        assert_eq!(serde_json::to_string(&ct).unwrap(), "2");

        let ct2 = ConnectionType::Cellular5G;
        assert_eq!(serde_json::to_string(&ct2).unwrap(), "7");
    }

    #[test]
    fn test_content_context() {
        let cc = ContentContext::Video;
        let json = serde_json::to_string(&cc).unwrap();
        assert_eq!(json, "1");

        let cc2: ContentContext = serde_json::from_str(&json).unwrap();
        assert_eq!(cc, cc2);
    }

    #[test]
    fn test_no_bid_reason() {
        let nbr = NoBidReason::UnknownError;
        assert_eq!(serde_json::to_string(&nbr).unwrap(), "0");

        let nbr2 = NoBidReason::BlockedPublisher;
        assert_eq!(serde_json::to_string(&nbr2).unwrap(), "7");
    }

    #[test]
    fn test_loss_reason() {
        let lr = LossReason::BidWon;
        assert_eq!(serde_json::to_string(&lr).unwrap(), "0");

        let lr2 = LossReason::BelowFloor;
        assert_eq!(serde_json::to_string(&lr2).unwrap(), "100");

        let lr3 = LossReason::CreativeFiltered;
        assert_eq!(serde_json::to_string(&lr3).unwrap(), "200");
    }

    #[test]
    fn test_banner_ad_type() {
        let bat = BannerAdType::JavaScriptAd;
        let json = serde_json::to_string(&bat).unwrap();
        assert_eq!(json, "3");
    }

    #[test]
    fn test_video_placement_type() {
        let vpt = VideoPlacementType::InStream;
        assert_eq!(serde_json::to_string(&vpt).unwrap(), "1");

        let vpt2 = VideoPlacementType::InFeed;
        assert_eq!(serde_json::to_string(&vpt2).unwrap(), "4");
    }

    #[test]
    fn test_location_type() {
        let lt = LocationType::GpsLocation;
        let json = serde_json::to_string(&lt).unwrap();
        assert_eq!(json, "1");

        let lt2: LocationType = serde_json::from_str(&json).unwrap();
        assert_eq!(lt, lt2);
    }

    #[test]
    fn test_feed_type() {
        let ft = FeedType::Podcast;
        let json = serde_json::to_string(&ft).unwrap();
        assert_eq!(json, "3");
    }

    #[test]
    fn test_qag_media_rating() {
        let qmr = QagMediaRating::AllAudiences;
        assert_eq!(serde_json::to_string(&qmr).unwrap(), "1");

        let qmr2 = QagMediaRating::Mature;
        assert_eq!(serde_json::to_string(&qmr2).unwrap(), "3");
    }

    #[test]
    fn test_enum_hash() {
        use std::collections::HashSet;

        let mut set = HashSet::new();
        set.insert(AuctionType::FirstPrice);
        set.insert(AuctionType::SecondPricePlus);
        set.insert(AuctionType::FirstPrice); // Duplicate

        assert_eq!(set.len(), 2);
        assert!(set.contains(&AuctionType::FirstPrice));
        assert!(set.contains(&AuctionType::SecondPricePlus));
    }

    #[test]
    fn test_enum_equality() {
        assert_eq!(DeviceType::Phone, DeviceType::Phone);
        assert_ne!(DeviceType::Phone, DeviceType::Tablet);
    }

    #[test]
    fn test_device_orientation() {
        let portrait = DeviceOrientation::Portrait;
        let json = serde_json::to_string(&portrait).unwrap();
        assert_eq!(json, "0");

        let landscape = DeviceOrientation::Landscape;
        let json2 = serde_json::to_string(&landscape).unwrap();
        assert_eq!(json2, "1");

        // Test deserialization
        let portrait2: DeviceOrientation = serde_json::from_str(&json).unwrap();
        assert_eq!(portrait, portrait2);

        // Test default
        assert_eq!(DeviceOrientation::default(), DeviceOrientation::Portrait);
    }

    #[test]
    fn test_local_market_identifier_type() {
        let nielsen = LocalMarketIdentifierType::Nielsen;
        let json = serde_json::to_string(&nielsen).unwrap();
        assert_eq!(json, "1");

        let kantar = LocalMarketIdentifierType::Kantar;
        let json2 = serde_json::to_string(&kantar).unwrap();
        assert_eq!(json2, "2");

        // Test roundtrip
        let nielsen2: LocalMarketIdentifierType = serde_json::from_str(&json).unwrap();
        assert_eq!(nielsen, nielsen2);
    }

    #[test]
    fn test_start_delay() {
        // Test pre-roll
        let pre_roll = StartDelay::PRE_ROLL;
        assert_eq!(pre_roll.0, 0);
        assert!(pre_roll.is_pre_roll());
        assert!(!pre_roll.is_mid_roll());
        assert!(!pre_roll.is_post_roll());

        // Test generic mid-roll
        let generic_mid = StartDelay::GENERIC_MID_ROLL;
        assert_eq!(generic_mid.0, -1);
        assert!(!generic_mid.is_pre_roll());

        // Test generic post-roll
        let generic_post = StartDelay::GENERIC_POST_ROLL;
        assert_eq!(generic_post.0, -2);
        assert!(generic_post.is_post_roll());

        // Test specific mid-roll
        let mid_roll = StartDelay::mid_roll(30);
        assert_eq!(mid_roll.0, 30);
        assert!(mid_roll.is_mid_roll());
        assert!(!mid_roll.is_pre_roll());

        // Test serialization
        let json = serde_json::to_string(&pre_roll).unwrap();
        assert_eq!(json, "0");

        let json2 = serde_json::to_string(&mid_roll).unwrap();
        assert_eq!(json2, "30");

        // Test deserialization
        let pre_roll2: StartDelay = serde_json::from_str(&json).unwrap();
        assert_eq!(pre_roll, pre_roll2);
    }

    #[test]
    fn test_slot_position() {
        let first = SlotPosition::First;
        let json = serde_json::to_string(&first).unwrap();
        assert_eq!(json, "1");

        let last = SlotPosition::Last;
        let json2 = serde_json::to_string(&last).unwrap();
        assert_eq!(json2, "-1");

        let any = SlotPosition::Any;
        let json3 = serde_json::to_string(&any).unwrap();
        assert_eq!(json3, "0");

        let first_or_last = SlotPosition::FirstOrLast;
        let json4 = serde_json::to_string(&first_or_last).unwrap();
        assert_eq!(json4, "2");

        // Test roundtrip
        let first2: SlotPosition = serde_json::from_str(&json).unwrap();
        assert_eq!(first, first2);
    }

    #[test]
    fn test_id_match_method() {
        let no_match = IDMatchMethod::NoMatching;
        let json = serde_json::to_string(&no_match).unwrap();
        assert_eq!(json, "0");

        let first_party = IDMatchMethod::FirstParty;
        let json2 = serde_json::to_string(&first_party).unwrap();
        assert_eq!(json2, "1");

        let probabilistic = IDMatchMethod::Probabilistic;
        let json3 = serde_json::to_string(&probabilistic).unwrap();
        assert_eq!(json3, "2");

        let deterministic = IDMatchMethod::Deterministic;
        let json4 = serde_json::to_string(&deterministic).unwrap();
        assert_eq!(json4, "3");

        // Test roundtrip
        let no_match2: IDMatchMethod = serde_json::from_str(&json).unwrap();
        assert_eq!(no_match, no_match2);
    }

    #[test]
    fn test_dooh_multiplier_measurement_source() {
        let unknown = DOOHMultiplierMeasurementSource::Unknown;
        let json = serde_json::to_string(&unknown).unwrap();
        assert_eq!(json, "0");

        let vendor = DOOHMultiplierMeasurementSource::MeasurementVendor;
        let json2 = serde_json::to_string(&vendor).unwrap();
        assert_eq!(json2, "1");

        let publisher = DOOHMultiplierMeasurementSource::Publisher;
        let json3 = serde_json::to_string(&publisher).unwrap();
        assert_eq!(json3, "2");

        let exchange = DOOHMultiplierMeasurementSource::Exchange;
        let json4 = serde_json::to_string(&exchange).unwrap();
        assert_eq!(json4, "3");

        // Test roundtrip
        let vendor2: DOOHMultiplierMeasurementSource = serde_json::from_str(&json2).unwrap();
        assert_eq!(vendor, vendor2);
    }
}
