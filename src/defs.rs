//! This module contains the transcript of types from Definitions section of
//! the Letterboxd API:
//!
//! http://letterboxd-api.dev.cactuslab.com/#definitions.
//!
//! Note that, in the API it is not always specified if a field is optional.
//! Therefore, most of the types below have to be adjusted with optional
//! values. Further, only the types that are in the API implementation are
//! public.

// TODO: remove
#![allow(dead_code)]

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
enum AbstractActivity {
    /// Common fields:
    /// member The member associated with the activity.
    /// when_created The timestamp of the activity, in ISO 8601 format with UTC
    /// timezone, i.e. YYYY-MM-DDThh:mm:ssZ "1997-08-29T07:14:00Z"
    DiaryEntryActivity {
        member: MemberSummary,
        when_created: String,
        /// The log entry associated with this activity.
        diary_entry: LogEntry,
    },
    FilmLikeActivity {
        member: MemberSummary,
        when_created: String,
        /// The film associated with the activity. Includes a
        /// MemberFilmRelationship for the member who added the activity.
        film: FilmSummary,
    },
    FilmRatingActivity {
        member: MemberSummary,
        when_created: String,
        /// The film associated with the activity. Includes a
        /// MemberFilmRelationship for the member who added the activity.
        film: FilmSummary,
        /// The member’s rating for the film. Allowable values are between 0.5
        /// and 5.0, with increments of 0.5.
        rating: f32,
    },
    FilmWatchActivity {
        member: MemberSummary,
        when_created: String,
        /// The film associated with the activity. Includes a
        /// MemberFilmRelationship for the member who added the activity.
        film: FilmSummary,
    },
    FollowActivity {
        member: MemberSummary,
        when_created: String,
        /// A summary of the member that was followed.
        followed: MemberSummary,
    },
    InvitationAcceptedActivity {
        member: MemberSummary,
        when_created: String,
        invitor: MemberSummary,
    },
    ListActivity {
        member: MemberSummary,
        when_created: String,
        /// The list associated with the activity.
        list: ListSummary,
        /// The list that was cloned, if applicable.
        cloned_from: Option<ListSummary>,
    },
    ListCommentActivity {
        member: MemberSummary,
        when_created: String,
        /// The list associated with the activity.
        list: ListSummary,
        /// The comment associated with the activity.
        comment: ListComment,
    },
    ListLikeActivity {
        member: MemberSummary,
        when_created: String,
        /// The list associated with the activity.
        list: ListSummary,
    },
    RegistrationActivity {
        member: MemberSummary,
        when_created: String,
    },
    ReviewActivity {
        member: MemberSummary,
        when_created: String,
        /// The log entry associated with this activity.
        review: LogEntry,
    },
    ReviewCommentActivity {
        member: MemberSummary,
        when_created: String,
        /// The review associated with the activity.
        review: LogEntry,
        /// The comment associated with the activity.
        comment: ReviewComment,
    },
    ReviewLikeActivity {
        member: MemberSummary,
        when_created: String,
        /// The review associated with the activity.
        review: LogEntry,
    },
    WatchlistActivity {
        member: MemberSummary,
        when_created: String,
        /// The film associated with the activity. Includes a
        /// MemberFilmRelationship for the member who added the activity.
        film: FilmSummary,
    },
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "type")]
enum AbstractComment {
    ListComment {
        /// The LID of the comment.
        id: String,
        /// The member who posted the comment.
        member: MemberSummary,
        /// ISO 8601 format with UTC timezone, i.e. YYYY-MM-DDThh:mm:ssZ:
        /// "1997-08-29T07:14:00Z"
        when_created: String,
        /// ISO 8601 format with UTC timezone, i.e. YYYY-MM-DDThh:mm:ssZ:
        /// "1997-08-29T07:14:00Z"
        when_updated: String,
        /// The message portion of the comment in LBML. May contain the
        /// following HTML tags: `<br>` `<strong>` `<em>` `<b>` `<i>` `<a
        /// href="">` `<blockquote>`.
        comment_lbml: String,
        /// If Letterboxd moderators have removed the comment from the site,
        /// removedByAdmin will be true and comment will not be included.
        removed_by_admin: bool,
        /// If the comment owner has removed the comment from the site, deleted
        /// will be true and comment will not be included.
        deleted: bool,
        /// If the authenticated member has blocked the commenter, blocked will
        /// be true and comment will not be included.
        blocked: bool,
        /// If the content owner has blocked the commenter, blockedByOwner will
        /// be true and comment will not be included.
        blocked_by_owner: bool,
        /// If the authenticated member posted this comment, and the comment is
        /// still editable, this value shows the number of seconds remaining
        /// until the editing window closes.
        editable_window_expires_in: Option<usize>,
        /// The list on which the comment was posted.
        list: ListIdentifier,
        /// The message portion of the comment formatted as HTML.
        comment: String,
    },
    ReviewComment {
        /// The LID of the comment.
        id: String,
        /// The member who posted the comment.
        member: MemberSummary,
        /// ISO 8601 format with UTC timezone, i.e. YYYY-MM-DDThh:mm:ssZ:
        /// "1997-08-29T07:14:00Z"
        when_created: String,
        /// ISO 8601 format with UTC timezone, i.e. YYYY-MM-DDThh:mm:ssZ:
        /// "1997-08-29T07:14:00Z"
        when_updated: String,
        /// The message portion of the comment in LBML. May contain the
        /// following HTML tags: `<br>` `<strong>` `<em>` `<b>` `<i>` `<a
        /// href="">` `<blockquote>`.
        comment_lbml: String,
        /// If Letterboxd moderators have removed the comment from the site,
        /// removedByAdmin will be true and comment will not be included.
        removed_by_admin: bool,
        /// If the comment owner has removed the comment from the site, deleted
        /// will be true and comment will not be included.
        deleted: bool,
        /// If the authenticated member has blocked the commenter, blocked will
        /// be true and comment will not be included.
        blocked: bool,
        /// If the content owner has blocked the commenter, blockedByOwner will
        /// be true and comment will not be included.
        blocked_by_owner: bool,
        /// If the authenticated member posted this comment, and the comment is
        /// still editable, this value shows the number of seconds remaining
        /// until the editing window closes.
        editable_window_expires_in: Option<usize>,
        /// The review on which the comment was posted.
        review: ReviewIdentifier,
        /// The message portion of the comment formatted as HTML.
        comment: String,
    },
}

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type")]
pub enum AbstractSearchItem {
    /// Common fields:
    /// score A relevancy value that can be used to order results.
    ContributorSearchItem {
        /// A relevancy value that can be used to order results.
        score: f32,
        /// Contributor Details of the contributor.
        contributor: Contributor,
    },
    FilmSearchItem { score: f32, film: FilmSummary },
    ListSearchItem { score: f32, list: ListSummary },
    MemberSearchItem { score: f32, member: MemberSummary },
    ReviewSearchItem {
        score: f32,
        /// Details of the review.
        review: LogEntry,
    },
    TagSearchItem { score: f32, tag: String },
}

#[derive(Deserialize, Debug, Clone)]
pub struct AccessToken {
    /// The access token that grants the member access. Combine this with the
    /// token_type to form the Authorization header.
    pub access_token: String,
    /// The type of the access token. Use value: bearer
    pub token_type: String,
    /// The refresh token is used to obtain a new access token, after the
    /// access token expires, without needing to prompt the member for their
    /// credentials again. The refresh token only expires if it is explicitly
    /// invalidated by Letterboxd, in which case the member should be prompted
    /// for their credentials (or stored credentials used).
    pub refresh_token: String,
    /// The number of seconds before the access token expires.
    pub expires_in: usize,
}

#[derive(Deserialize, Debug, Clone)]
enum ActivityClass {
    OwnActivity,
    NotOwnActivity,
    IncomingActivity,
    NotIncomingActivity,
    NetworkActivity,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct ActivityRequest {
    /// The pagination cursor.
    cursor: Option<Cursor>,
    /// The number of items to include per page (default is 20, maximum is 100).
    per_page: Option<usize>,
    /// Only supported for paying members.
    /// Use include to specify the subset of activity to be returned. If
    /// neither include nor exclude is set, the activity types included depend
    /// on the where parameter:
    /// If where=OwnActivity is specified, all activity except
    /// FilmLikeActivity, FilmWatchActivity and InvitationAcceptedActivity is
    /// included.
    /// Otherwise all activity except FilmLikeActivity, FilmWatchActivity,
    /// FilmRatingActivity, FollowActivity, RegistrationActivity and
    /// InvitationAcceptedActivity is included.
    /// These defaults mimic those shown on the website.
    include: Option<Vec<ActivityType>>,
    /// Use where to reduce the subset of activity to be returned. If where is
    /// not set, all default activity types relating to the member are
    /// returned. If multiple values are supplied, only activity matching all
    /// terms will be returned, e.g.
    /// where=OwnActivity&where=NotIncomingActivity will return all activity by
    /// the member except their comments on their own lists and reviews.
    /// NetworkActivity is activity performed either by the member or their
    /// followers. Use where=NetworkActivity&where=NotOwnActivity to only see
    /// activity from followers. If you don’t specify any of NetworkActivity,
    /// OwnActivity or NotIncomingActivity, you will receive activity related
    /// to the member’s content from members outside their network (e.g.
    /// comments and likes on the member’s lists and reviews).
    #[serde(rename = "where")]
    where_activity: Option<Vec<ActivityClass>>,
}

#[derive(Deserialize, Debug, Clone)]
enum ActivityType {
    ReviewActivity,
    ReviewCommentActivity,
    ReviewLikeActivity,
    ListActivity,
    ListCommentActivity,
    ListLikeActivity,
    DiaryEntryActivity,
    FilmRatingActivity,
    FilmWatchActivity,
    FilmLikeActivity,
    WatchlistActivity,
    FollowActivity,
    RegistrationActivity,
    InvitationAcceptedActivity,
}

#[derive(Deserialize, Debug, Clone)]
struct ActivityResponse {
    /// The cursor to the next page of results.
    next: Option<Cursor>,
    /// The list of activity items.
    items: Vec<AbstractActivity>,
}

#[derive(Serialize, Debug, Clone)]
pub struct CommentCreationRequest {
    /// The message portion of the comment in LBML. May contain the following
    /// HTML tags: `<br>` `<strong>` `<em>` `<b>` `<i>` `<a href="">`
    /// `<blockquote>`. This field has a maximum size of 100,000 characters.
    comment: String,
}

#[derive(Deserialize, Debug, Clone)]
enum CommentUpdateMessageCode {
    MissingComment,
    CommentOnContentYouBlocked,
    CommentOnBlockedContent,
    CommentBan,
    CommentEditWindowExpired,
    CommentTooLong,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type")]
enum CommentUpdateMessage {
    Error {
        /// The error message code.
        code: CommentUpdateMessageCode,
        /// The error message text in human-readable form.
        title: String,
    },
    Success,
}

#[derive(Deserialize, Debug, Clone)]
struct CommentUpdateRequest {
    /// The message portion of the comment in LBML. May contain the following
    /// HTML tags: `<br>` `<strong>` `<em>` `<b>` `<i>` `<a href="">`
    /// `<blockquote>`. This field has a maximum size of 100,000 characters.
    comment: String,
}

#[derive(Deserialize, Debug, Clone)]
struct CommentUpdateResponse {
    /// The response object.
    data: AbstractComment,
    /// A list of messages the API client should show to the user.
    messages: Vec<CommentUpdateMessage>,
}

// TODO: Ordering
#[derive(Deserialize, Debug, Clone)]
enum CommentsRequestSort {
    Date,
    Updates,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct CommentsRequest {
    /// The pagination cursor.
    cursor: Option<Cursor>,
    /// The number of items to include per page (default is 20, maximum is 100).
    per_page: Option<usize>,
    /// Defaults to Date. The Updates sort order returns newest content first.
    /// Use this to get the most recently posted or edited comments, and pass
    /// include_deletions=true to remain consistent in the case where a comment
    /// has been deleted.
    sort: CommentsRequestSort,
    /// Use this to discover any comments that were deleted.
    include_deletions: bool,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct ContributionStatistics {
    /// The type of contribution.
    #[serde(rename = "type")]
    contribution_type: ContributionType,
    /// The number of films for this contribution type.
    film_count: usize,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum ContributionType {
    Director,
    Actor,
    Producer,
    Writer,
    Editor,
    Cinematography,
    ArtDirection,
    VisualEffects,
    Composer,
    Sound,
    Costumes,
    MakeUp,
    Studio,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Contributor {
    /// The LID of the contributor.
    pub id: String,
    /// The name of the contributor.
    pub name: String,
    /// An array of the types of contributions made, with a count of films for
    /// each contribution type.
    // TODO
    // statistics: ContributorStatistics,
    // A list of relevant URLs to this entity, on Letterboxd and external sites.
    pub links: Vec<Link>,
}

#[derive(Deserialize, Debug, Clone)]
struct ContributorStatistics {
    // The statistics for each contribution type.
    contributions: Vec<ContributionStatistics>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContributorSummary {
    /// The LID of the contributor.
    pub id: String,
    /// The name of the contributor.
    pub name: String,
    /// The character name if available (only if the contribution is as an
    /// Actor; see the type field in FilmContributions).
    pub character_name: Option<String>,
}

/// A cursor is a String value provided by the API. It should be treated as an
/// opaque value — don’t change it.
pub type Cursor = String;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DiaryDetails {
    /// The date the film was watched, if specified, in ISO 8601 format, i.e.
    /// YYYY-MM-DD
    pub diary_date: String,
    /// Will be true if the member has indicated (or it can be otherwise
    /// determined) that the member has seen the film prior to this date.
    pub rewatch: bool,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Film {
    /// The LID of the film.
    pub id: String,
    /// The title of the film.
    pub name: String,
    /// The original title of the film, if it was first released with a
    /// non-English title.
    pub original_name: Option<String>,
    /// The other names by which the film is known (including alternative
    /// titles and/or foreign translations).
    pub alternative_names: Vec<String>,
    /// The year in which the film was first released.
    pub release_year: u16,
    /// The tagline for the film.
    pub tagline: String,
    /// A synopsis of the film.
    pub description: String,
    /// The film’s duration (in minutes).
    pub run_time: u16,
    /// The film’s poster image (2:3 ratio in multiple sizes).
    pub poster: Image,
    /// The film’s backdrop image (16:9 ratio in multiple sizes).
    pub backdrop: Image,
    /// The backdrop’s vertical focal point, expressed as a proportion of the
    /// image’s height, using values between 0.0 and 1.0. Use when cropping the
    /// image into a shorter space, such as in the page for a film on the
    /// Letterboxd site.
    pub backdrop_focal_point: f32,
    /// The film’s trailer.
    pub trailer: FilmTrailer,
    /// The film’s genres.
    pub genres: Vec<Genre>,
    /// The film’s contributors (director, cast and crew) grouped by discipline.
    pub contributions: Vec<FilmContributions>,
    /// A list of relevant URLs to this entity, on Letterboxd and external
    /// sites.
    pub links: Vec<Link>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct FilmAutocompleteRequest {
    /// The number of items to include per page (default is 20, maximum is 100).
    per_page: Option<usize>,
    /// The word, partial word or phrase to match against.
    input: String,
}

#[derive(Deserialize, Debug, Clone)]
pub enum FilmAvailabilityService {
    Amazon,
    AmazonVideo,
    AmazonPrime,
    #[allow(non_camel_case_types)]
    iTunes,
    Netflix,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FilmAvailability {
    /// The service.
    pub service: FilmAvailabilityService,
    /// The service’s name.
    pub display_name: String,
    /// The regional store for the service. Not all countries are supported on
    /// all services.
    pub country: Country,
    /// The unique ID (if any) for the film on the store.
    pub id: String,
    /// The fully qualified URL for the film on this store.
    pub url: String,
}

// TODO: order
#[derive(Deserialize, Debug, Clone)]
pub enum Country {
    AIA,
    ARE,
    ARG,
    ARM,
    ATG,
    AUS,
    AUT,
    AZE,
    BEL,
    BFA,
    BGR,
    BHR,
    BHS,
    BLR,
    BLZ,
    BMU,
    BOL,
    BRA,
    BRB,
    BRN,
    BWA,
    CAN,
    CHE,
    CHL,
    CHN,
    COL,
    CPV,
    CRI,
    CYM,
    CYP,
    CZE,
    DEU,
    DMA,
    DNK,
    DOM,
    ECU,
    EGY,
    ESP,
    EST,
    FIN,
    FJI,
    FRA,
    FSM,
    GBR,
    GHA,
    GMB,
    GNB,
    GRC,
    GRD,
    GTM,
    HKG,
    HND,
    HUN,
    IDN,
    IND,
    IRL,
    ISR,
    ITA,
    JOR,
    JPN,
    KAZ,
    KEN,
    KGZ,
    KHM,
    KNA,
    LAO,
    LBN,
    LKA,
    LTU,
    LUX,
    LVA,
    MAC,
    MDA,
    MEX,
    MLT,
    MNG,
    MOZ,
    MUS,
    MYS,
    NAM,
    NER,
    NGA,
    NIC,
    NLD,
    NOR,
    NPL,
    NZL,
    OMN,
    PAN,
    PER,
    PHL,
    PNG,
    POL,
    PRT,
    PRY,
    QAT,
    ROU,
    RUS,
    SAU,
    SGP,
    SLV,
    SVK,
    SVN,
    SWE,
    SWZ,
    THA,
    TJK,
    TKM,
    TTO,
    TUR,
    TWN,
    UGA,
    UKR,
    USA,
    UZB,
    VEN,
    VGB,
    VNM,
    ZAF,
    ZWE,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FilmAvailabilityResponse {
    /// The list of stores where the film is available for streaming or
    /// purchasing, in order of preference. If the member has not specified
    /// their preferred stores for a service, the USA store will be assumed.
    pub items: Option<Vec<FilmAvailability>>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct FilmContribution {
    /// The type of contribution.
    #[serde(rename = "type")]
    contribution_type: ContributionType,
    /// The film.
    film: FilmSummary,
    /// The name of the character (only when type is Actor).
    character_name: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FilmContributions {
    /// The type of contribution.
    pub contribution_type: Option<ContributionType>,
    /// The list of contributors of the specified type for the film.
    pub contributors: Vec<ContributorSummary>,
}

// TODO: Ordering, Dedup
#[derive(Serialize, Debug, Clone)]
pub enum FilmStatus {
    Released,
    NotReleased,
    InWatchlist,
    NotInWatchlist,
    Watched,
    NotWatched,
    FeatureLength,
    NotFeatureLength,
}

// TODO: Ordering
#[derive(Serialize, Debug, Clone)]
pub enum FilmRelationshipType {
    Watched,
    NotWatched,
    Liked,
    NotLiked,
    InWatchlist,
    NotInWatchlist,
    Favorited,
}

#[derive(Serialize, Debug, Clone)]
enum FilmContributionsSort {
    FilmName,
    ReleaseDateLatestFirst,
    ReleaseDateEarliestFirst,
    RatingHighToLow,
    RatingLowToHigh,
    FilmDurationShortestFirst,
    FilmDurationLongestFirst,
    FilmPopularity,
    FilmPopularityThisWeek,
    FilmPopularityThisMonth,
    FilmPopularityThisYear,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct FilmContributionsRequest {
    /// The pagination cursor.
    cursor: Option<Cursor>,
    /// The number of items to include per page (default is 20, maximum is 100).
    per_page: Option<usize>,
    /// The order in which the films should be returned. Defaults to
    /// FilmPopularity, which is an all-time measurement of the amount of
    /// activity the film has received. The FilmPopularityWithFriends values
    /// are only available to signed-in members and consider popularity amongst
    /// the signed-in member’s friends.
    sort: FilmContributionsSort,
    /// The type of contribution.
    #[serde(rename = "type")]
    contribution_type: ContributionType,
    /// Specify the LID of a genre to limit films to those within the specified
    /// genre.
    genre: String,
    /// Specify the starting year of a decade (must end in 0) to limit films to
    /// those released during the decade. 1990
    decade: u16,
    /// Specify a year to limit films to those released during that year. 1994
    year: u16,
    /// Specify the ID of a supported service to limit films to those available
    /// from that service. The list of available services can be found by using
    /// the /films/film-services endpoint.
    service: String,
    /// Specify one or more values to limit the list of films accordingly.
    /// where=Watched&where=Released
    #[serde(rename = "where")]
    where_film_status: Vec<FilmStatus>,
    /// Specify the LID of a member to limit the returned films according to
    /// the value set in memberRelationship.
    member: String,
    /// Must be used in conjunction with member. Defaults to Watched. Specify
    /// the type of relationship to limit the returned films accordingly.
    member_relationship: FilmRelationshipType,
    /// Must be used in conjunction with member. Defaults to None, which only
    /// returns films from the member’s account. Use Only to return films from
    /// the member’s friends, and All to return films from both the member and
    /// their friends.
    include_friends: IncludeFriends,
    /// Specify a tag code to limit the returned films to those tagged
    /// accordingly.
    tag_code: String,
    /// Must be used with tag. Specify the LID of a member to focus the tag
    /// filter on the member.
    tagger: String,
    /// Must be used in conjunction with tagger. Defaults to None, which
    /// filters tags set by the member. Use Only to filter tags set by the
    /// member’s friends, and All to filter tags set by both the member and
    /// their friends.
    include_tagger_friends: IncludeFriends,
}

#[derive(Deserialize, Debug, Clone)]
struct FilmContributionsResponse {
    /// The cursor to the next page of results.
    next: Option<Cursor>,
    /// The list of contributions.
    items: Vec<FilmContribution>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FilmIdentifier {
    /// The LID of the film.
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FilmRelationship {
    /// Will be true if the member has indicated they’ve seen the film (via the
    /// ‘eye’ icon) or has a log entry for the film.
    pub watched: bool,
    /// Will be true if the member likes the film (via the ‘heart’ icon).
    pub liked: bool,
    /// Will be true if the member listed the film as one of their four
    /// favorites.
    pub favorited: bool,
    /// Will be true if the film is in the member’s watchlist.
    pub in_watchlist: bool,
    /// The member’s rating for the film.
    pub rating: Option<f32>,
    /// A list of LIDs for reviews the member has written for the film in the
    /// order they were added, with most recent reviews first.
    pub reviews: Vec<String>,
    /// A list of LIDs for log entries the member has added for the film in
    /// diary order, with most recent entries first.
    pub diary_entries: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub enum FilmRelationshipUpdateMessageCode {
    InvalidRatingValue,
    UnableToRemoveWatch,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum FilmRelationshipUpdateMessage {
    Error {
        /// The error message code.
        code: FilmRelationshipUpdateMessageCode,
        /// The error message text in human-readable form.
        title: String,
    },
}

/// When PATCHing a film relationship, you may send all of the current property
/// struct values, or just those you wish to change. Properties that violate
/// business rules (see watched below) or contain invalid values will be
/// ignored.
#[derive(Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct FilmRelationshipUpdateRequest {
    /// Set to true to change the film’s status for the authenticated member to
    /// ‘watched’ or false for ‘not watched’. If the status is changed to
    /// ‘watched’ and the film is in the member’s watchlist, it will be removed
    /// as part of this action. You may not change the status of a film to ‘not
    /// watched’ if there is existing activity (a review or diary entry) for
    /// the authenticated member—check the messages returned from this endpoint
    /// to ensure no such business rules have been violated.
    pub watched: Option<bool>,
    /// Set to true to change the film’s status for the authenticated member to
    /// ‘liked’ or false for ‘not liked’.
    pub liked: Option<bool>,
    /// Set to true to add the film to the authenticated member’s watchlist, or
    /// false to remove it.
    pub in_watchlist: Option<bool>,
    /// Accepts values between 0.5 and 5.0, with increments of 0.5, or null (to
    /// remove the rating).
    pub rating: Option<f32>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FilmRelationshipUpdateResponse {
    /// The response object.
    pub data: FilmRelationship,
    /// A list of messages the API client should show to the user.
    pub messages: Vec<FilmRelationshipUpdateMessage>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FilmServicesResponse {
    // The list of film services.
    pub items: Vec<Service>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FilmStatistics {
    /// The film for which statistics were requested.
    pub film: FilmIdentifier,
    /// The number of watches, ratings, likes, etc. for the film.
    pub counts: FilmStatisticsCounts,
    /// The weighted average rating of the film between 0.5 and 5.0. Will not
    /// be present if the film has not received sufficient ratings.
    pub rating: Option<f32>,
    /// A summary of the number of ratings at each increment between 0.5 and
    /// 5.0.
    pub ratings_histogram: Vec<RatingsHistogramBar>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FilmStatisticsCounts {
    /// The number of members who have watched the film.
    pub watches: usize,
    /// The number of members who have liked the film.
    pub likes: usize,
    /// The number of members who have rated the film.
    pub ratings: usize,
    /// The number of members who have the film as one of their four favorites.
    pub fans: usize,
    /// The number of lists in which the film appears.
    pub lists: usize,
    /// The number of reviews for the film.
    pub reviews: usize,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FilmSummary {
    /// The LID of the film.
    pub id: String,
    /// The title of the film.
    pub name: String,
    /// The original title of the film, if it was first released with a
    /// non-English title.
    pub original_name: Option<String>,
    /// The other names by which the film is known (including alternative
    /// titles and/or foreign translations).
    pub alternative_names: Option<Vec<String>>,
    /// The year in which the film was first released.
    pub release_year: Option<u16>,
    /// The list of directors for the film.
    pub directors: Vec<ContributorSummary>,
    /// The film’s poster image (2:3 ratio in multiple sizes).
    pub poster: Option<Image>,
    /// Relationships to the film for the authenticated member (if any) and
    /// other members where relevant.
    pub relationships: Vec<MemberFilmRelationship>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FilmTrailer {
    /// The YouTube ID of the trailer. "ICp4g9p_rgo".
    pub id: String,
    /// The YouTube URL for the trailer.
    /// "https://www.youtube.com/watch?v=ICp4g9p_rgo"
    pub url: String,
}

#[derive(Deserialize, Debug, Clone)]
struct FilmsAutocompleteResponse {
    // The list of films.
    items: Vec<FilmSummary>,
}

#[derive(Serialize, Debug, Clone)]
pub enum FilmRequestSort {
    FilmName,
    ReleaseDateLatestFirst,
    ReleaseDateEarliestFirst,
    RatingHighToLow,
    RatingLowToHigh,
    FilmDurationShortestFirst,
    FilmDurationLongestFirst,
    FilmPopularity,
    FilmPopularityThisWeek,
    FilmPopularityThisMonth,
    FilmPopularityThisYear,
    FilmPopularityWithFriends,
    FilmPopularityWithFriendsThisWeek,
    FilmPopularityWithFriendsThisMonth,
    FilmPopularityWithFriendsThisYear,
}

#[derive(Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct FilmsRequest {
    /// The pagination cursor.
    pub cursor: Option<Cursor>,
    /// The number of items to include per page (default is 20, maximum is 100).
    pub per_page: Option<usize>,
    /// The order in which the films should be returned. Defaults to
    /// FilmPopularity, which is an all-time measurement of the amount of
    /// activity the film has received. The FilmPopularityWithFriends values
    /// are only available to signed-in members and consider popularity amongst
    /// the signed-in member’s friends.
    pub sort: Option<FilmRequestSort>,
    /// Specify the LID of a genre to limit films to those within the specified
    /// genre.
    pub genre: Option<String>,
    /// Specify the starting year of a decade (must end in 0) to limit films to
    /// those released during the decade. 1990
    pub decade: Option<u16>,
    /// Specify a year to limit films to those released during that year. 1994
    pub year: Option<u16>,
    /// Specify the ID of a supported service to limit films to those available
    /// from that service. The list of available services can be found by using
    /// the /films/film-services endpoint.
    pub service: Option<String>,
    /// Specify one or more values to limit the list of films accordingly.
    /// where=Watched&where=Released
    #[serde(rename = "where")]
    pub where_film_status: Vec<FilmStatus>,
    /// Specify the LID of a member to limit the returned films according to
    /// the value set in memberRelationship.
    pub member: Option<String>,
    /// Must be used in conjunction with member. Defaults to Watched. Specify
    /// the type of relationship to limit the returned films accordingly.
    pub member_relationship: Option<FilmRelationshipType>,
    /// Must be used in conjunction with member. Defaults to None, which only
    /// returns films from the member’s account. Use Only to return films from
    /// the member’s friends, and All to return films from both the member and
    /// their friends.
    pub include_friends: Option<IncludeFriends>,
    /// Specify a tag code to limit the returned films to those tagged
    /// accordingly.
    pub tag_code: Option<String>,
    /// Must be used with tag. Specify the LID of a member to focus the tag
    /// filter on the member.
    pub tagger: Option<String>,
    /// Must be used in conjunction with tagger. Defaults to None, which
    /// filters tags set by the member. Use Only to filter tags set by the
    /// member’s friends, and All to filter tags set by both the member and
    /// their friends.
    pub include_tagger_friends: Option<IncludeFriends>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct FilmsResponse {
    /// The cursor to the next page of results.
    pub next: Option<Cursor>,
    /// The list of films.
    pub items: Vec<FilmSummary>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct ForgottenPasswordRequest {
    email_address: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Genre {
    /// The LID of the genre.
    pub id: String,
    /// The name of the genre.
    pub name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct GenresResponse {
    /// The list of genres.
    pub items: Vec<Genre>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Image {
    /// The available sizes for the image.
    pub sizes: Vec<ImageSize>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ImageSize {
    /// The image width in pixels.
    pub width: usize,
    /// The image height in pixels.
    pub height: usize,
    /// The URL to the image file.
    pub url: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Link {
    Letterboxd {
        ///   The object ID for the linked entity on the destination site.
        id: String,
        ///   The fully qualified URL on the destination site.
        url: String,
    },
    Tmdb {
        ///   The object ID for the linked entity on the destination site.
        id: String,
        ///   The fully qualified URL on the destination site.
        url: String,
    },
    Imdb {
        ///   The object ID for the linked entity on the destination site.
        id: String,
        ///   The fully qualified URL on the destination site.
        url: String,
    },
    Gwi {
        ///   The object ID for the linked entity on the destination site.
        id: String,
        ///   The fully qualified URL on the destination site.
        url: String,
    },
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct List {
    /// The LID of the list.
    pub id: String,
    /// The name of the list.
    pub name: String,
    /// The number of films in the list.
    pub film_count: usize,
    /// Will be true if the owner has elected to publish the list for other
    /// members to see.
    pub published: bool,
    /// Will be true if the owner has elected to make this a ranked list.
    pub ranked: bool,
    /// Will be true if the owner has added notes to any entries.
    pub has_entries_with_notes: bool,
    /// The list description in LBML. May contain the following HTML tags:
    /// `<br>` `<strong>` `<em>` `<b>` `<i>` `<a href="">` `<blockquote>`.
    pub description_lbml: Option<String>,
    /// The tags for the list.
    pub tags2: Vec<Tag>,
    /// The third-party service or services to which this list can be shared.
    /// Only included if the authenticated member is the list’s owner.
    pub can_share_on: Vec<ThirdPartyService>,
    /// The third-party service or services to which this list has been shared.
    /// Only included if the authenticated member is the list’s owner.
    pub shared_on: Vec<ThirdPartyService>,
    /// ISO 8601 format with UTC timezone, i.e. YYYY-MM-DDThh:mm:ssZ
    /// "1997-08-29T07:14:00Z"
    pub when_created: String,
    /// ISO 8601 format with UTC timezone, i.e. YYYY-MM-DDThh:mm:ssZ
    /// "1997-08-29T07:14:00Z"
    pub when_published: Option<String>,
    /// The member who owns the list.
    pub owner: MemberSummary,
    /// The list this was cloned from, if applicable.
    pub cloned_from: Option<ListIdentifier>,
    /// The first 12 entries in the list. To fetch more than 12 entries, and to
    /// fetch the entry notes, use the /list/{id}/entries endpoint.
    pub preview_entries: Vec<ListEntrySummary>,
    /// A list of relevant URLs to this entity, on Letterboxd and external
    /// sites.
    pub links: Vec<Link>,
    /// The list description formatted as HTML.
    pub description: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct ListComment {
    /// The LID of the comment.
    id: String,
    /// The member who posted the comment.
    member: MemberSummary,
    /// ISO 8601 format with UTC timezone, i.e. YYYY-MM-DDThh:mm:ssZ
    /// "1997-08-29T07:14:00Z"
    when_created: String,
    /// ISO 8601 format with UTC timezone, i.e. YYYY-MM-DDThh:mm:ssZ
    /// "1997-08-29T07:14:00Z"
    when_updated: String,
    /// The message portion of the comment in LBML. May contain the following
    /// HTML tags: `<br>` `<strong>` `<em>` `<b>` `<i>` `<a href="">`
    /// `<blockquote>`.
    comment_lbml: String,
    /// If Letterboxd moderators have removed the comment from the site,
    /// removedByAdmin will be true and comment will not be included.
    removed_by_admin: bool,
    /// If the comment owner has removed the comment from the site, deleted
    /// will be true and comment will not be included.
    deleted: bool,
    /// If the authenticated member has blocked the commenter, blocked will be
    /// true and comment will not be included.
    blocked: bool,
    /// If the list owner has blocked the commenter, blockedByOwner will be
    /// true and comment will not be included.
    blocked_by_owner: bool,
    /// If the authenticated member posted this comment, and the comment is
    /// still editable, this value shows the number of seconds remaining until
    /// the editing window closes.
    editable_window_expires_in: Option<usize>,
    /// The list on which the comment was posted.
    list: ListIdentifier,
    /// The message portion of the comment formatted as HTML.
    comment: String,
}

#[derive(Deserialize, Debug, Clone)]
struct ListCommentsResponse {
    /// The cursor to the next page of results.
    next: Option<Cursor>,
    /// The list of comments.
    items: Vec<ListComment>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListCreateEntry {
    /// The LID of the film.
    film: String,
    /// The entry’s rank in the list, numbered from 1. If not set, the entry
    /// will be appended to the end of the list. Sending two or more
    /// ListCreateEntrys with the same rank will return an error.
    rank: usize,
    /// The notes for the list entry in LBML. May contain the following HTML tags: `<br>` `<strong>` `<em>` `<b>` `<i>` `<a href="">` `<blockquote>`.
    notes: String,
    /// Set to true if the member has indicated that the notes field contains
    /// plot spoilers for the film.
    contains_spoilers: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub enum ListCreateMessageCode {
    ListNameIsBlank,
    UnknownFilmCode,
    InvalidRatingValue,
    DuplicateRank,
    EmptyPublicList,
    CloneSourceNotFound,
    SharingServiceNotAuthorized,
    CannotSharePrivateList,
    ListDescriptionIsTooLong,
    ListEntryNotesTooLong,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum ListCreateMessage {
    Error {
        /// The error message code.
        code: ListCreateMessageCode,
        /// The error message text in human-readable form.
        title: String,
    },
    Success,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ListCreateResponse {
    /// The response object.
    pub data: List,
    // A list of messages the API client should show to the user.
    pub messages: Vec<ListCreateMessage>,
}

#[derive(Serialize, Debug, Clone)]
pub struct ListCreationRequest {
    /// The name of the list.
    name: String,
    /// Set to true if the owner has elected to publish the list for other
    /// members to see.
    published: bool,
    /// Set to true if the owner has elected to make this a ranked list.
    ranked: bool,
    /// The list description in LBML. May contain the following HTML tags:
    /// `<br>` `<strong>` `<em>` `<b>` `<i>` `<a href="">` `<blockquote>`. This
    /// field has a maximum size of 100,000 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    /// The LID of a list to clone from. Only supported for paying members.
    #[serde(skip_serializing_if = "Option::is_none")]
    cloned_from: Option<String>,
    // The tags for the list.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tags: Vec<String>,
    /// The films that comprise the list. Required unless source is set.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    entries: Vec<ListCreateEntry>,
    /// The third-party service or services to which this list should be shared. Valid options are found in the MemberAccount.authorizedSharingServicesForLists (see the /me endpoint).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    share: Vec<ThirdPartyService>,
}

impl ListCreationRequest {
    pub fn new(name: String) -> Self {
        Self {
            name: name,
            published: false,
            ranked: false,
            description: None,
            cloned_from: None,
            tags: Vec::new(),
            entries: Vec::new(),
            share: Vec::new(),
        }
    }
}

#[derive(Serialize, Debug, Clone)]
enum ListEntriesRequestSort {
    ListRanking,
    WhenAddedToList,
    RatingHighToLow,
    RatingLowToHigh,
    FilmName,
    ReleaseDateLatestFirst,
    ReleaseDateEarliestFirst,
    FilmDurationShortestFirst,
    FilmDurationLongestFirst,
    FilmPopularity,
    FilmPopularityThisWeek,
    FilmPopularityThisMonth,
    FilmPopularityThisYear,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct ListEntriesRequest {
    /// The pagination cursor.
    cursor: Option<Cursor>,
    /// The number of items to include per page (default is 20, maximum is 100).
    per_page: Option<usize>,
    /// The order in which the entries should be returned. Defaults to
    /// ListRanking, which is the order specified by the list owner.
    sort: ListEntriesRequestSort,
    /// Specify the LID of a genre to limit films to those within the specified
    /// genre.
    genre: String,
    /// Specify the starting year of a decade (must end in 0) to limit films to
    /// those released during the decade. 1990
    decade: u16,
    /// Specify a year to limit films to those released during that year. 1994
    year: u16,
    /// Specify the ID of a supported service to limit films to those available
    /// from that service. The list of available services can be found by using
    /// the /films/film-services endpoint.
    service: String,
    /// Specify one or more values to limit the list of films accordingly.
    /// where=Watched&where=Released
    #[serde(rename = "where")]
    where_film_status: FilmStatus,
    /// Specify the LID of a member to limit the returned films according to
    /// the value set in memberRelationship.
    member: String,
    /// Must be used in conjunction with member. Defaults to Watched. Specify
    /// the type of relationship to limit the returned films accordingly.
    member_relationship: FilmRelationshipType,
    /// Must be used in conjunction with member. Defaults to None, which only
    /// returns films from the member’s account. Use Only to return films from
    /// the member’s friends, and All to return films from both the member and
    /// their friends.
    include_friends: IncludeFriends,
    /// Specify a tag code to limit the returned films to those tagged
    /// accordingly.
    tag_code: String,
    /// Must be used with tag. Specify the LID of a member to focus the tag
    /// filter on the member.
    tagger: String,
    /// Must be used in conjunction with tagger. Defaults to None, which
    /// filters tags set by the member. Use Only to filter tags set by the
    /// member’s friends, and All to filter tags set by both the member and
    /// their friends.
    include_tagger_friends: IncludeFriends,
}

#[derive(Deserialize, Debug, Clone)]
struct ListEntriesResponse {
    ///     The cursor to the next page of results.
    next: Option<Cursor>,
    // The list of entries.
    items: Vec<ListEntry>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct ListEntry {
    /// The entry’s rank in the list, numbered from 1.
    rank: usize,
    /// The notes for the list entry in LBML. May contain the following HTML tags: `<br>` `<strong>` `<em>` `<b>` `<i>` `<a href="">` `<blockquote>`.
    notes_lbml: String,
    /// Will be true if the member has indicated that the notes field contains
    /// plot spoilers for the film.
    contains_spoilers: bool,
    /// The film for this entry. Includes a MemberFilmRelationship for the
    /// member who created the list.
    film: FilmSummary,
    /// The notes for the list entry formatted as HTML.
    notes: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ListEntrySummary {
    /// The entry’s rank in the list, numbered from 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<usize>,
    /// The film for this entry.
    pub film: FilmSummary,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ListIdentifier {
    /// The LID of the list.
    pub id: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct ListRelationship {
    /// Will be true if the member likes the list (via the ‘heart’ icon). A
    /// member may not like their own list.
    liked: bool,
    /// Will be true if the member is subscribed to comment notifications for
    /// the list
    subscribed: bool,
    /// Defaults to Subscribed for the list’s owner, and NotSubscribed for
    /// other members. The subscription value may change when a member (other
    /// than the owner) posts a comment, as follows: the member will become
    /// automatically Subscribed unless they have previously Unsubscribed from
    /// the comment thread via the web interface or API, or unless they have
    /// disabled comment notifications in their profile settings.
    subscription_state: SubscriptionState,
    /// The authenticated member’s state with respect to adding comments for
    /// this list.
    comment_thread_state: CommentThreadState,
}

#[derive(Deserialize, Debug, Clone)]
enum ListRelationshipUpdateMessageCode {
    LikeBlockedContent,
    LikeOwnList,
    SubscribeWhenOptedOut,
    SubscribeToContentYouBlocked,
    SubscribeToBlockedContent,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type")]
enum ListRelationshipUpdateMessage {
    Error {
        /// The error message code.
        code: ListRelationshipUpdateMessageCode,
        /// The error message text in human-readable form.
        title: String,
    },
    Success,
}

#[derive(Deserialize, Debug, Clone)]
struct ListRelationshipUpdateRequest {
    /// Set to true if the member likes the list (via the ‘heart’ icon). A
    /// member may not like their own list.
    liked: bool,
    /// Set to true to subscribe the member to comment notifications for the list, or false to unsubscribe them. A value of true will be ignored if the member has disabled comment notifications in their profile settings.
    subscribed: bool,
}

#[derive(Deserialize, Debug, Clone)]
struct ListRelationshipUpdateResponse {
    /// The response object.
    data: ListRelationship,
    /// A list of messages the API client should show to the user.
    messages: Vec<ListRelationshipUpdateMessage>,
}


#[derive(Deserialize, Debug, Clone)]
struct ListStatistics {
    /// The list for which statistics were requested.
    list: ListIdentifier,
    /// The number of comments and likes for the list.
    counts: ListStatisticsCounts,
}

#[derive(Deserialize, Debug, Clone)]
struct ListStatisticsCounts {
    /// The number of comments for the list.
    comments: usize,
    /// The number of members who like the list.
    likes: usize,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListSummary {
    /// The LID of the list.
    pub id: String,
    /// The name of the list.
    pub name: String,
    /// The number of films in the list.
    pub film_count: usize,
    /// Will be true if the owner has elected to publish the list for other
    /// members to see.
    pub published: bool,
    /// Will be true if the owner has elected to make this a ranked list.
    pub ranked: bool,
    /// The list description in LBML. May contain the following HTML tags:
    /// `<br>` `<strong>` `<em>` `<b>` `<i>` `<a href="">` `<blockquote>`. The
    /// text is a preview extract, and may be truncated if it’s too long.
    pub description_lbml: Option<String>,
    /// Will be true if the list description was truncated because it’s very
    /// long.
    pub description_truncated: Option<bool>,
    /// The member who owns the list.
    pub owner: MemberSummary,
    /// The list this was cloned from, if applicable.
    pub cloned_from: Option<ListIdentifier>,
    /// The first 12 entries in the list. To fetch more than 12 entries, and to
    /// fetch the entry notes, use the /list/{id}/entries endpoint.
    pub preview_entries: Vec<ListEntrySummary>,
    /// The list description formatted as HTML. The text is a preview extract,
    /// and may be truncated if it’s too long.
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListUpdateEntry {
    /// The LID of the film.
    pub film: String,
    /// The entry’s rank in the list, numbered from 1. If not set, the entry
    /// will stay in the same place (if already in the list) or be appended to
    /// the end of the list (if not in the list). If set, any entries at or
    /// after this position will be incremented by one. Sending two or more
    /// ListUpdateEntrys with the same rank will return an error.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank: Option<usize>,
    /// The notes for the list entry in LBML. May contain the following HTML
    /// tags: `<br>` `<strong>` `<em>` `<b>` `<i>` `<a href="">`
    /// `<blockquote>`. This field has a maximum size of 100,000 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// Set to true if the member has indicated that the notes field contains
    /// plot spoilers for the film.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contains_spoilers: Option<bool>,
}

impl ListUpdateEntry {
    pub fn new(film: String) -> ListUpdateEntry {
        ListUpdateEntry {
            film: film,
            rank: None,
            notes: None,
            contains_spoilers: None,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub enum ListUpdateMessageCode {
    ListNameIsBlank,
    UnknownFilmCode,
    InvalidRatingValue,
    DuplicateRank,
    EmptyPublicList,
    SharingServiceNotAuthorized,
    CannotSharePrivateList,
    ListDescriptionIsTooLong,
    ListEntryNotesTooLong,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum ListUpdateMessage {
    Error {
        /// The error message code.
        code: ListUpdateMessageCode,
        /// The error message text in human-readable form.
        title: String,
    },
    Success,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListUpdateRequest {
    /// Set to true if the owner has elected to publish the list for other
    /// members to see.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published: Option<bool>,
    /// The name of the list.
    pub name: String,
    /// Set to true if the owner has elected to make this a ranked list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranked: Option<bool>,
    /// The list description in LBML. May contain the following HTML tags:
    /// `<br>` `<strong>` `<em>` `<b>` `<i>` `<a href="">` `<blockquote>`. This
    /// field has a maximum size of 100,000 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The tags for the list.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
    /// Specify the LIDs of films to be removed from the list.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub films_to_remove: Vec<String>,
    /// The specified entries will be inserted/appended to the list if they are
    /// not already present, or updated if they are present.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub entries: Vec<ListUpdateEntry>,
    /// The third-party service or services to which this list should be
    /// shared. Valid options are found in the ListRelationship (see the
    /// /list/{id}/me endpoint).
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub share: Vec<ThirdPartyService>,
}

impl ListUpdateRequest {
    pub fn new(name: String) -> ListUpdateRequest {
        ListUpdateRequest {
            published: None,
            name: name,
            ranked: None,
            description: None,
            tags: Vec::new(),
            films_to_remove: Vec::new(),
            entries: Vec::new(),
            share: Vec::new(),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct ListUpdateResponse {
    /// The response object.
    pub data: List,
    // A list of messages the API client should show to the user.
    pub messages: Vec<ListUpdateMessage>,
}

#[derive(Serialize, Debug, Clone)]
pub enum ListRequestSort {
    Date,
    WhenCreatedLatestFirst,
    WhenCreatedEarliestFirst,
    ListName,
    ListPopularity,
    ListPopularityThisWeek,
    ListPopularityThisMonth,
    ListPopularityThisYear,
    ListPopularityWithFriends,
    ListPopularityWithFriendsThisWeek,
    ListPopularityWithFriendsThisMonth,
    ListPopularityWithFriendsThisYear,
}

#[derive(Serialize, Debug, Clone)]
pub enum ListMemberRelationship {
    Owner,
    Liked,
}

#[derive(Serialize, Debug, Clone)]
pub enum ListStatus {
    Clean,
    Published,
    Unpublished,
}

#[derive(Serialize, Debug, Clone)]
pub enum ListRequestFilter {
    NoDuplicateMembers,
}

#[derive(Serialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListsRequest {
    /// The pagination cursor.
    pub cursor: Option<Cursor>,
    /// The number of items to include per page (default is 20, maximum is 100).
    pub per_page: Option<usize>,
    /// Defaults to Date, which returns lists that were most recently
    /// created/updated first. The ListPopularityWithFriends values are only
    /// available to signed-in members and consider popularity amongst the
    /// signed-in member’s friends.
    pub sort: Option<ListRequestSort>,
    /// Specify the LID of a film to return lists that include that film.
    pub film: Option<String>,
    /// Specify the LID of a list to return lists that were cloned from that
    /// list.
    pub cloned_from: Option<String>,
    /// Specify a tag code to limit the returned lists to those tagged
    /// accordingly. Must be used with member and memberRelationship=Owner.
    pub tag_code: Option<String>,
    /// Specify the LID of a member to return lists that are owned or liked by
    /// the member (or their friends, when used with includeFriends).
    pub member: Option<String>,
    /// Must be used in conjunction with member. Defaults to Owner, which
    /// returns lists owned by the specified member. Use Liked to return lists
    /// liked by the member.
    pub member_relationship: Option<ListMemberRelationship>,
    /// Must be used in conjunction with member. Defaults to None, which only
    /// returns lists from the member’s account. Use Only to return lists from
    /// the member’s friends, and All to return lists from both the member and
    /// their friends.
    pub include_friends: Option<IncludeFriends>,
    /// Specify Clean to return lists that do not contain profane language.
    /// Specify Published to return the member’s lists that have been made
    /// public. Note that unpublished lists for members other than the
    /// authenticated member are never returned. Specify NotPublished to return
    /// the authenticated member’s lists that have not been made public.
    #[serde(rename = "where")]
    pub where_list_status: Vec<ListStatus>,
    /// Specify NoDuplicateMembers to limit the list to only the first list for
    /// each member. filter=NoDuplicateMembers
    pub filter: Vec<ListRequestFilter>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ListsResponse {
    /// The cursor to the next page of results.
    next: Option<Cursor>,
    /// The list of lists.
    items: Vec<ListSummary>,
}

#[derive(Serialize, Debug, Clone)]
enum LogEntriesRequestSort {
    WhenAdded,
    Date,
    RatingHighToLow,
    RatingLowToHigh,
    ReleaseDateLatestFirst,
    ReleaseDateEarliestFirst,
    FilmName,
    FilmDurationShortestFirst,
    FilmDurationLongestFirst,
    ReviewPopularity,
    ReviewPopularityThisWeek,
    ReviewPopularityThisMonth,
    ReviewPopularityThisYear,
    ReviewPopularityWithFriends,
    ReviewPopularityWithFriendsThisWeek,
    ReviewPopularityWithFriendsThisMonth,
    ReviewPopularityWithFriendsThisYear,
    FilmPopularity,
    FilmPopularityThisWeek,
    FilmPopularityThisMonth,
    FilmPopularityThisYear,
    FilmPopularityWithFriends,
    FilmPopularityWithFriendsThisWeek,
    FilmPopularityWithFriendsThisMonth,
    FilmPopularityWithFriendsThisYear,
}

#[derive(Serialize, Debug, Clone)]
enum LogEntryRelationshipType {
    Owner,
    Liked,
}

#[derive(Serialize, Debug, Clone)]
enum LogEntryStatus {
    HasDiaryDate,
    HasReview,
    Clean,
    NoSpoilers,
    Released,
    NotReleased,
    FeatureLength,
    NotFeatureLength,
    InWatchlist,
    NotInWatchlist,
    Watched,
    NotWatched,
    Rated,
    NotRated,
}

#[derive(Serialize, Debug, Clone)]
enum LogEntryFilter {
    NoDuplicateMembers,
}

#[derive(Serialize, Debug, Clone)]
struct LogEntriesRequest {
    /// The pagination cursor.
    cursor: Option<Cursor>,
    /// The number of items to include per page (default is 20, maximum is 100).
    per_page: Option<usize>,
    /// The order in which the log entries should be returned. Defaults to
    /// WhenAdded, which orders by creation date, unless you specify
    /// where=HasDiaryDate in which case the default is Date.
    /// The ReviewPopularity values return reviews with more activity
    /// (likes/comments) first, and imply where=HasReview.
    /// The FilmPopularity values return reviews for more popular films first.
    /// The ReviewPopularityWithFriends and FilmPopularityWithFriends values
    /// are only available to signed-in members and consider popularity amongst
    /// the signed-in member’s friends.
    /// The Date value sorts by the diary date, and implies where=HasDiaryDate
    /// You may not specify a film when using ReleaseDateLatestFirst,
    /// ReleaseDateEarliestFirst, FilmName, FilmDurationShortestFirst,
    /// FilmDurationLongestFirst, or any of the FilmPopularity options.
    sort: LogEntriesRequestSort,
    /// Specify the LID of a film to return log entries for that film. Must not
    /// be included if the sort value is ReleaseDateLatestFirst,
    /// ReleaseDateEarliestFirst, FilmName, FilmDurationShortestFirst,
    /// FilmDurationLongestFirst, or any of the FilmPopularity options.
    film: String,
    /// Specify the LID of a member to limit the returned log entries according
    /// to the value set in memberRelationship.
    member: String,
    /// Must be used in conjunction with member. Use Owner to limit the
    /// returned log entries to those created by the specified member. Use
    /// Liked to limit the returned reviews to those liked by the specified
    /// member (implies where=HasReview).
    member_relationship: LogEntryRelationshipType,
    /// Must be used in conjunction with member. Specify the type of
    /// relationship to limit the returned films accordingly. e.g. Use Liked to
    /// limit the returned reviews to those for films liked by the member.
    film_member_relationship: FilmRelationshipType,
    /// Must be used in conjunction with member. Defaults to None, which only
    /// returns log entries created or liked by the member. Use Only to return
    /// log entries created or liked by the member’s friends, and All to return
    /// log entries created or liked by both the member and their friends.
    include_friends: IncludeFriends,
    /// If set, limits the returned log entries to those with date that falls
    /// during the specified year.
    year: u16,
    /// Accepts values of 1 through 12. Must be used with year. If set, limits
    /// the returned log entries to those with a date that falls during the
    /// specified month and year.
    month: u16,
    /// Accepts values of 1 through 52. Must be used with year. If set, limits
    /// the returned log entries to those with a date that falls during the
    /// specified week and year.
    week: u16,
    /// Accepts values of 1 through 31. Must be used with month and year. If
    /// set, limits the returned log entries to those with a date that falls on
    /// the specified day, month and year.
    day: u16,
    /// Allowable values are between 0.5 and 5.0, with increments of 0.5. If
    /// set, limits the returned log entries to those with a rating equal to or
    /// higher than the specified rating.
    min_rating: f32,
    /// Allowable values are between 0.5 and 5.0, with increments of 0.5. If
    /// set, limits the returned log entries to those with a rating equal to or
    /// lower than the specified rating.
    max_rating: f32,
    /// Specify the starting year of a decade (must end in 0) to limit films to
    /// those released during the decade. 1990
    film_decade: u16,
    /// Specify a year to limit films to those released during that year. 1994
    film_year: u16,
    /// The LID of the genre. If set, limits the returned log entries to those
    /// for films that match the specified genre.
    genre: String,
    /// Specify a tag code to limit the returned log entries to those tagged
    /// accordingly.
    tag_code: String,
    /// Must be used with tag. Specify the LID of a member to focus the tag
    /// filter on the member.
    tagger: String,
    /// Must be used in conjunction with tagger. Defaults to None, which
    /// filters tags set by the member. Use Only to filter tags set by the
    /// member’s friends, and All to filter tags set by both the member and
    /// their friends.
    include_tagger_friends: IncludeFriends,
    /// Specify the ID of a supported service to limit films to those available
    /// from that service. The list of available services can be found by using
    /// the /films/film-services endpoint.
    service: String,
    /// Specify one or more values to limit the returned log entries
    /// accordingly. All values except HasDiaryDate, HasReview, Clean and
    /// NoSpoilers refer to properties of the associated film rather than to
    /// the relevant log entry. Use HasDiaryDate to limit the returned log
    /// entries to those that appear in a member’s diary. Use HasReview to
    /// limit the returned log entries to those containing a review. Use Clean
    /// to exclude reviews that contain profane language. Use NoSpoilers to
    /// exclude reviews where the owner has indicated that the review text
    /// contains plot spoilers for the film. where=Clean&where=NoSpoilers
    #[serde(rename = "where")]
    where_logentry_status: Vec<LogEntryStatus>,
    /// Specify NoDuplicateMembers to return only the first log entry for each
    /// member. filter=NoDuplicateMembers
    filter: Vec<LogEntryFilter>,
}

#[derive(Deserialize, Debug, Clone)]
struct LogEntriesResponse {
    /// The cursor to the next page of results.
    next: Option<Cursor>,
    // The list of log entries.
    items: Vec<LogEntry>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LogEntry {
    /// The LID of the log entry.
    pub id: String,
    /// A descriptive title for the log entry.
    pub name: String,
    /// The member who created the log entry.
    pub owner: MemberSummary,
    /// The film being logged. Includes a MemberFilmRelationship for the member
    /// who created the log entry.
    pub film: FilmSummary,
    /// Details about the log entry, if present.
    pub diary_details: Option<DiaryDetails>,
    /// Review details for the log entry, if present.
    pub review: Option<Review>,
    /// The tags for the log entry.
    pub tags2: Vec<Tag>,
    /// The timestamp of when the log entry was created, in ISO 8601 format
    /// with UTC timezone, i.e. YYYY-MM-DDThh:mm:ssZ "1997-08-29T07:14:00Z"
    pub when_created: String,
    /// The timestamp of when the log entry was last updated, in ISO 8601
    /// format with UTC timezone, i.e. YYYY-MM-DDThh:mm:ssZ
    /// "1997-08-29T07:14:00Z"
    pub when_updated: String,
    /// The member’s rating for the film. Allowable values are between 0.5 and
    /// 5.0, with increments of 0.5.
    pub rating: f32,
    /// Will be true if the member likes the film (via the ‘heart’ icon).
    pub like: bool,
    /// Will be true if the log entry can have comments.
    pub commentable: bool,
    /// A list of relevant URLs to this entity, on Letterboxd and external
    /// sites.
    pub links: Vec<Link>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct LogEntryCreationRequest {
    /// The film being logged.
    film_id: String,
    /// Information about this log entry if adding to the member’s diary.
    diary_details: LogEntryCreationRequestDiaryDetails,
    /// Information about the review if adding a review.
    review: LogEntryCreationRequestReview,
    ///  The tags for the log entry.
    tags: Vec<String>,
    /// Allowable values are between 0.5 and 5.0, with increments of 0.5.
    rating: f32,
    /// Set to true if the member likes the film (via the ‘heart’ icon).
    like: bool,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct LogEntryCreationRequestDiaryDetails {
    /// The date the film was watched, if specified, in ISO 8601 format, i.e.
    /// YYYY-MM-DD
    diary_date: String,
    /// Set to true if the member has indicated (or it can be otherwise
    /// determined) that the member has seen the film prior to this date.
    rewatch: bool,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct LogEntryCreationRequestReview {
    /// The review text in LBML. May contain the following HTML tags: `<br>`
    /// `<strong>` `<em>` `<b>` `<i>` `<a href="">` `<blockquote>`. This field
    /// has a maximum size of 100,000 characters.
    text: String,
    /// Set to true if the member has indicated that the review field contains
    /// plot spoilers for the film.
    contains_spoilers: bool,
    /// The third-party service or services to which this review should be
    /// shared. Valid options are found in the
    /// MemberAccount.authorizedSharingServicesForReviews (see the /me
    /// endpoint).
    share: Vec<ThirdPartyService>,
}

#[derive(Deserialize, Debug, Clone)]
enum LogEntryUpdateMessageCode {
    InvalidRatingValue,
    InvalidDiaryDate,
    ReviewWithNoText,
    ReviewIsTooLong,
    LogEntryWithNoReviewOrDiaryDetails,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type")]
enum LogEntryUpdateMessage {
    Error {
        /// The error message code
        code: LogEntryUpdateMessageCode,
        /// The error message text in human-readable form.
        title: String,
    },
    Success,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct LogEntryUpdateRequest {
    /// Information about this log entry if adding to the member’s diary. Set
    /// to null to remove this log entry from the diary.
    diary_details: LogEntryUpdateRequestDiaryDetails,
    /// Information about the review. Set to null to remove the review from
    /// this log entry.
    review: LogEntryUpdateRequestReview,
    // The tags for the log entry.
    tags: Vec<String>,
    /// Accepts values between 0.5 and 5.0, with increments of 0.5, or null (to
    /// remove the rating).
    rating: f32,
    /// Set to true if the member likes the film (via the ‘heart’ icon).
    like: bool,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct LogEntryUpdateRequestDiaryDetails {
    /// The date the film was watched, if specified, in ISO 8601 format, i.e.
    /// YYYY-MM-DD
    diary_date: String,
    /// Set to true if the member has indicated (or it can be otherwise
    /// determined) that the member has seen the film prior to this date.
    rewatch: bool,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct LogEntryUpdateRequestReview {
    /// The review text in LBML. May contain the following HTML tags: `<br>`
    /// `<strong>` `<em>` `<b>` `<i>` `<a href="">` `<blockquote>`.
    text: String,
    /// Set to true if the member has indicated that the review field contains
    /// plot spoilers for the film.
    contains_spoilers: bool,
    // The third-party service or services to which this review should be shared. Valid options are found in the ReviewRelationship.canShareOn (see the /log-entry/{id}/me endpoint).
    share: Vec<ThirdPartyService>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct Member {
    /// The LID of the member.
    id: String,
    /// The member’s Letterboxd username. Usernames must be between 2 and 15
    /// characters long and may only contain upper or lowercase letters,
    /// numbers or the underscore (_) character.
    username: String,
    /// The given name of the member.
    given_name: String,
    /// The family name of the member.
    family_name: String,
    /// A convenience method that returns the member’s given name and family
    /// name concatenated with a space, if both are set, or just their given
    /// name or family name, if one is set, or their username, if neither is
    /// set. Will never be empty.
    display_name: String,
    /// A convenience method that returns the member’s given name, if set, or
    /// their username. Will never be empty.
    short_name: String,
    /// The member’s preferred pronoun set. Use the /members/pronouns endpoint
    /// to request all available pronoun sets.
    pronoun: Pronoun,
    /// The member’s Twitter username, if they have authenticated their account.
    twitter_username: String,
    /// The member’s bio in LBML. May contain the following HTML tags: `<br>`
    /// `<strong>` `<em>` `<b>` `<i>` `<a href="">` `<blockquote>`.
    bio_lbml: String,
    /// The member’s location.
    location: String,
    /// The member’s website URL. URLs are not validated, so sanitizing may be
    /// required.
    website: String,
    /// The member’s avatar image at multiple sizes.
    avatar: Image,
    /// The member’s backdrop image at multiple sizes, sourced from the first
    /// film in the member’s list of favorite films, if available. Only
    /// returned for Patron members.
    backdrop: Image,
    /// The vertical focal point of the member’s backdrop image, if available.
    /// Expressed as a proportion of the image’s height, using values between
    /// 0.0 and 1.0. Use when cropping the image into a shorter space, such as
    /// in the page for a film on the Letterboxd site.
    backdrop_focal_point: f32,
    /// The member’s account type.
    member_status: MemberStatus,
    /// A summary of the member’s favorite films, up to a maximum of four.
    favorite_films: Vec<FilmSummary>,
    /// A link to the member’s profile page on the Letterboxd website.
    links: Vec<Link>,
    /// The member’s bio formatted as HTML.
    bio: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct MemberAccount {
    /// The member’s email address.
    email_address: String,
    /// Will be true if the member has validated their emailAddress via an
    /// emailed link.
    email_address_validated: bool,
    /// Defaults to false for new accounts. Indicates whether the member has
    /// elected for their content to appear in the API (other than in the /me
    /// endpoint).
    private_account: bool,
    /// Defaults to true for new accounts. Indicates whether the member has
    /// elected to appear in the People section of the Letterboxd website.
    include_in_people_section: bool,
    /// Defaults to false for new accounts. Indicates whether the member has
    /// elected to hide their Watchlist from other members.
    private_watchlist: bool,
    /// Defaults to true for new accounts. Indicates whether the member has elected to receive email notifications when they receive a new follower.
    email_when_followed: bool,
    /// Defaults to true for new accounts. Indicates whether the member has
    /// elected to receive email notifications when new comments are posted in
    /// threads they are subscribed to.
    email_comments: bool,
    /// Defaults to true for new accounts. Indicates whether the member has
    /// elected to receive regular email news (including ‘Call Sheet’) from
    /// Letterboxd.
    email_news: bool,
    /// Defaults to true for new accounts. Indicates whether the member has
    /// elected to receive a weekly email digest of new and popular content
    /// (called ‘Rushes’).
    email_rushes: bool,
    /// Defaults to false for new accounts. Indicates whether the member has
    /// commenting privileges. Commenting is disabled on new accounts until the
    /// member’s emailAddress is validated. At present canComment is synonymous
    /// with emailAddressValidated (unless the member is suspended) but this
    /// may change in future.
    can_comment: bool,
    /// Indicates whether the member is suspended from commenting due to a
    /// breach of the Community Policy.
    suspended: bool,
    /// Indicates whether the member is able to clone other members’ lists.
    /// Determined by Letterboxd based upon memberStatus.
    can_clone_lists: bool,
    /// Indicates whether the member is able to filter activity by type.
    /// Determined by Letterboxd based upon memberStatus.
    can_filter_activity: bool,
    /// The services the member has authorized Letterboxd to share lists to.
    /// More services may be added in the future.
    authorized_sharing_services_for_lists: Vec<ThirdPartyService>,
    /// The services the member has authorized Letterboxd to share reviews to.
    /// More services may be added in the future.
    authorized_sharing_services_for_reviews: Vec<ThirdPartyService>,
    /// The number of days the member has left in their subscription. Only
    /// returned for paying members.
    membership_days_remaining: usize,
    /// Standard member details.
    member: Member,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MemberFilmRelationship {
    /// The member.
    pub member: MemberSummary,
    /// The relationship details.
    pub relationship: FilmRelationship,
}

// TODO: dedup and order
#[derive(Deserialize, Debug, Clone)]
enum MemberRelationshipType {
    IsFollowing,
    IsFollowedBy,
}

#[derive(Serialize, Debug, Clone)]
pub enum MemberFilmRelationshipsRequestSort {
    Date,
    Name,
    MemberPopularity,
    MemberPopularityThisWeek,
    MemberPopularityThisMonth,
    MemberPopularityThisYear,
    MemberPopularityWithFriends,
    MemberPopularityWithFriendsThisWeek,
    MemberPopularityWithFriendsThisMonth,
    MemberPopularityWithFriendsThisYear,
}

#[derive(Serialize, Debug, Clone, Default)]
pub struct MemberFilmRelationshipsRequest {
    /// The pagination cursor.
    pub cursor: Option<Cursor>,
    /// The number of items to include per page (default is 20, maximum is 100).
    pub per_page: Option<usize>,
    /// Defaults to Date, which has different semantics based on the request:
    /// When review is specified, members who most recently liked the review
    /// appear first.
    /// When list is specified, members who most recently liked the list appear
    /// first.
    /// When film is specified and filmRelationship=Watched, members who most
    /// recently watched the film appear first.
    /// When film is specified and filmRelationship=Liked, members who most
    /// recently liked the film appear first.
    /// When member is specified and memberRelationship=IsFollowing, most
    /// recently followed members appear first.
    /// When member is specified and memberRelationship=IsFollowedBy, most
    /// recent followers appear first.
    /// Otherwise, members who most recently joined the site appear first.
    /// The PopularWithFriends values are only available to authenticated
    /// members and consider popularity amongst the member’s friends.
    pub sort: Option<MemberFilmRelationshipsRequestSort>,
    /// Specify the LID of a member to return members who follow or are
    /// followed by that member.
    pub member: Option<String>,
    /// Must be used in conjunction with member. Defaults to IsFollowing, which
    /// returns the list of members followed by the member. Use IsFollowedBy to
    /// return the list of members that follow the member.
    pub member_relationship: Option<FilmRelationshipType>,
    /// Must be used in conjunction with film. Defaults to Watched, which
    /// returns the list of members who have seen the film. Specify the type of
    /// relationship to limit the returned members accordingly.
    pub film_relationship: Option<FilmRelationshipType>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MemberFilmRelationshipsResponse {
    /// The cursor to the next page of results.
    pub next: Cursor,
    /// The list of film relationships for members.
    pub items: Vec<MemberFilmRelationship>,
}

#[derive(Deserialize, Debug, Clone)]
struct MemberIdentifier {
    /// The LID of the member.
    id: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct MemberRelationship {
    /// Will be true if the authenticated member follows the member identified
    /// by ID.
    following: bool,
    /// Will be true if the member identified by ID follows the authenticated
    /// member.
    followed_by: bool,
    /// Will be true if the authenticated member has blocked the member
    /// identified by ID.
    blocking: bool,
    /// Will be true if the member identified by ID has blocked the
    /// authenticated member.
    blocked_by: bool,
}

#[derive(Deserialize, Debug, Clone)]
enum MemberRelationshipUpdateMessageCode {
    BlockYourself,
    FollowYourself,
    FollowBlockedMember,
    FollowMemberYouBlocked,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type")]
enum MemberRelationshipUpdateMessage {
    Error {
        /// The error message code.
        code: MemberRelationshipUpdateMessageCode,
        /// The error message text in human-readable form.
        title: String,
    },
    Success,
}

#[derive(Deserialize, Debug, Clone)]
struct MemberRelationshipUpdateRequest {
    /// Set to true if the authenticated member wishes to follow the member
    /// identified by ID, or false if they wish to unfollow. A member may not
    /// follow their own account, or the account of a member they have blocked
    /// or that has blocked them.
    following: bool,
    /// Set to true if the authenticated member wishes to block the member
    /// identified by ID, or false if they wish to unblock. A member may not
    /// block their own account.
    blocking: bool,
}

#[derive(Deserialize, Debug, Clone)]
struct MemberRelationshipUpdateResponse {
    /// The response object.
    data: MemberRelationship,
    /// A list of messages the API client should show to the user.
    messages: Vec<MemberRelationshipUpdateMessage>,
}

#[derive(Deserialize, Debug, Clone)]
enum MemberSettingsUpdateMessageCode {
    IncorrectCurrentPassword,
    BlankPassword,
    InvalidEmailAddress,
    InvalidFavoriteFilm,
    BioTooLong,
    InvalidPronounOption,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type")]
enum MemberSettingsUpdateMessage {
    Error {
        /// The error message code.
        code: MemberSettingsUpdateMessageCode,
        /// The error message text in human-readable form.
        title: String,
    },
    Success,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct MemberSettingsUpdateRequest {
    /// The member’s email address.
    email_address: String,
    /// The member’s current password. Required when updating the password.
    current_password: String,
    /// The member’s new password.
    password: String,
    /// The given name of the member.
    given_name: String,
    /// The family name of the member.
    family_name: String,
    /// The LID of the member’s preferred pronoun set. Use the
    /// /members/pronouns endpoint to request all available pronoun sets.
    pronoun: String,
    /// The member’s location.
    location: String,
    /// The member’s website URL. URLs are not validated, so sanitizing may be
    /// required.
    website: String,
    /// The member’s bio in LBML. May contain the following HTML tags: `<br>`
    /// `<strong>` `<em>` `<b>` `<i>` `<a href="">` `<blockquote>`. This field
    /// has a maximum size of 100,000 characters.
    bio: String,
    /// The LIDs of the member’s favorite films, in order, up to a maximum of
    /// four.
    favorite_films: Vec<String>,
    /// Set to true to prevent the member’s content from appearing in API
    /// requests other than the /me endpoint.
    private_account: bool,
    /// Set to false to remove the account from the People section of the
    /// Letterboxd website.
    include_in_people_section: bool,
    /// Set to true if the member wishes to receive email notifications when
    /// they receive a new follower.
    email_when_followed: bool,
    /// Set to true if the member wishes to receive email notifications when
    /// new comments are posted in threads they are subscribed to.
    email_comments: bool,
    /// Set to true if the member wishes to receive regular email news
    /// (including ‘Call Sheet’) from Letterboxd.
    email_news: bool,
    /// Set to true if the member wishes to receive a weekly email digest of
    /// new and popular content (called ‘Rushes’).
    email_rushes: bool,
}

#[derive(Deserialize, Debug, Clone)]
struct MemberSettingsUpdateResponse {
    /// The response object.
    data: MemberAccount,
    /// A list of messages the API client should show to the user.
    messages: Vec<MemberSettingsUpdateMessage>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct MemberStatistics {
    /// The member for which statistics were requested.
    member: MemberIdentifier,
    /// The number of watches, ratings, likes, etc. for the member.
    counts: MemberStatisticsCounts,
    /// A summary of the number of ratings the member has made for each
    /// increment between 0.5 and 5.0. Returns only the integer increments
    /// between 1.0 and 5.0 if the member never (or rarely) awards half-star
    /// ratings.
    ratings_histogram: Vec<RatingsHistogramBar>,
    /// A list of years the member has year-in-review pages for. Only supported
    /// for paying members.
    years_in_review: Vec<u16>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct MemberStatisticsCounts {
    /// The number of films the member has liked.
    film_likes: usize,
    /// The number of lists the member has liked.
    list_likes: usize,
    /// The number of reviews the member has liked.
    review_likes: usize,
    /// The number of films the member has watched. This is a distinct total —
    /// films with multiple log entries are only counted once.
    watches: usize,
    /// The number of films the member has rated.
    ratings: usize,
    /// The number of films the member has reviewed.
    reviews: usize,
    /// The number of entries the member has in their diary.
    diary_entries: usize,
    /// The number of entries the member has in their diary for the current
    /// year. The current year rolls over at midnight on 31 December in New
    /// Zealand Daylight Time (GMT + 13).
    diary_entries_this_year: usize,
    /// The number of unique films the member has in their diary for the
    /// current year. The current year rolls over at midnight on 31 December in
    /// New Zealand Daylight Time (GMT + 13).
    films_in_diary_this_year: usize,
    /// The number of films the member has in their watchlist.
    watchlist: usize,
    /// The number of lists for the member. Includes unpublished lists if the
    /// request is made for the authenticated member.
    lists: usize,
    /// The number of unpublished lists for the member. Only included if the
    /// request is made for the authenticated member.
    unpublished_lists: usize,
    /// The number of members who follow the member.
    followers: usize,
    /// The number of members the member is following.
    following: usize,
    /// The number of tags the member has used for lists.
    list_tags: usize,
    /// The number of tags the member has used for diary entries and reviews.
    film_tags: usize,
}

#[derive(Clone, Debug, Deserialize)]
pub enum MemberStatus {
    Crew,
    Patron,
    Pro,
    Member,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemberSummary {
    /// The LID of the member.
    pub id: String,
    /// The member’s Letterboxd username. Usernames must be between 2 and 15
    /// characters long and
    /// may only contain upper or lowercase letters, numbers or the underscore
    /// (_) character.
    pub username: String,
    /// The given name of the member.
    pub given_name: Option<String>,
    /// The family name of the member.
    pub family_name: Option<String>,
    /// A convenience method that returns the member’s given name and family
    /// name concatenated with
    /// a space, if both are set, or just their given name or family name, if
    /// one is set, or their
    /// username, if neither is set. Will never be empty.
    pub display_name: String,
    /// A convenience method that returns the member’s given name, if set, or
    /// their username. Will never be empty.
    pub short_name: String,
    /// The member’s preferred pronoun set. Use the /members/pronouns endpoint
    /// to request all available pronoun sets.
    pub pronoun: Pronoun,
    /// The member’s avatar image at multiple sizes.
    pub avatar: Image,
    /// The member’s account type.
    pub member_status: MemberStatus,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MemberTag {
    /// The tag code.
    code: String,
    /// The tag text as entered by the tagger.
    display_tag: String,
    /// Counts of the member’s uses of this tag.
    counts: MemberTagCounts,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct MemberTagCounts {
    /// The number of films the member has used this tag on.
    films: usize,
    /// The number of log entries the member has used this tag on.
    log_entries: usize,
    /// The number of diary entries the member has used this tag on.
    diary_entries: usize,
    /// The number of reviews the member has used this tag on.
    reviews: usize,
    /// The number of lists the member has used this tag on.
    lists: usize,
}

#[derive(Clone, Debug, Serialize)]
struct MemberTagsRequest {
    /// A case-insensitive prefix match. E.g. “pro” will match “pro”, “project”
    /// and “Professional”. An empty input will match all tags.
    input: String,
}

#[derive(Clone, Debug, Deserialize)]
struct MemberTagsResponse {
    /// The list of tag items, ordered by frequency of use.
    items: Vec<MemberTag>,
}

#[derive(Clone, Debug, Serialize)]
enum MembersRequestSort {
    Date,
    Name,
    MemberPopularity,
    MemberPopularityThisWeek,
    MemberPopularityThisMonth,
    MemberPopularityThisYear,
    MemberPopularityWithFriends,
    MemberPopularityWithFriendsThisWeek,
    MemberPopularityWithFriendsThisMonth,
    MemberPopularityWithFriendsThisYear,
}

// TODO: name
#[derive(Clone, Debug, Serialize)]
enum MembersRequestRelationship {
    IsFollowing,
    IsFollowedBy,
}

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct MembersRequest {
    /// The pagination cursor.
    cursor: Option<Cursor>,
    /// The number of items to include per page (default is 20, maximum is 100).
    per_page: Option<usize>,
    /// Defaults to Date, which has different semantics based on the request:
    /// When review is specified, members who most recently liked the review
    /// appear first.
    /// When list is specified, members who most recently liked the list appear
    /// first.
    /// When film is specified and filmRelationship=Watched, members who most
    /// recently watched the film appear first.
    /// When film is specified and filmRelationship=Liked, members who most
    /// recently liked the film appear first.
    /// When member is specified and memberRelationship=IsFollowing, most
    /// recently followed members appear first.
    /// When member is specified and memberRelationship=IsFollowedBy, most
    /// recent followers appear first.
    /// Otherwise, members who most recently joined the site appear first.
    /// The PopularWithFriends values are only available to authenticated
    /// members and consider popularity amongst the member’s friends.
    sort: MembersRequestSort,
    /// Specify the LID of a member to return members who follow or are
    /// followed by that member.
    member: String,
    /// Must be used in conjunction with member. Defaults to IsFollowing, which
    /// returns the list of members followed by the member. Use IsFollowedBy to
    /// return the list of members that follow the member.
    member_relationship: MembersRequestRelationship,
    /// Specify the LID of a film to return members who have interacted with
    /// that film.
    film: String,
    /// Must be used in conjunction with film. Defaults to Watched, which
    /// returns the list of members who have seen the film. Specify the type of
    /// relationship to limit the returned members accordingly. You must
    /// specify a member in order to use the InWatchlist relationship.
    film_relationship: FilmRelationship,
    /// Specify the LID of a list to return members who like that list.
    list: String,
    /// Specify the LID of a review to return members who like that review.
    review: String,
}

#[derive(Clone, Debug, Deserialize)]
struct MembersResponse {
    /// The cursor to the next page of results.
    next: Option<Cursor>,
    /// The list of members.
    items: Vec<MemberSummary>,
}

#[derive(Clone, Debug, Deserialize)]
struct OAuthError {
    /// The error code, usually invalid_grant.
    error: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Pronoun {
    /// The LID for this pronoun set.
    pub id: String,
    /// A label to describe this pronoun set.
    pub label: String,
    /// The pronoun to use when the member is the subject. "She went to the
    /// movies."
    pub subject_pronoun: String,
    /// The pronoun to use when the member is the object. "I went with her to
    /// the cinema."
    pub object_pronoun: String,
    /// The adjective to use when describing a specified thing or things
    /// belonging to or associated with a member previously mentioned. "He
    /// bought his tickets."
    pub possessive_adjective: String,
    /// The pronoun to use when referring to a specified thing or things
    /// belonging to or associated with a member previously mentioned. "That
    /// popcorn was hers."
    pub possessive_pronoun: String,
    /// The pronoun to use to refer back to the member. "He saw himself as a
    /// great director."
    pub reflexive: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PronounsResponse {
    /// The list of pronouns.
    items: Vec<Pronoun>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RatingsHistogramBar {
    /// The rating increment between 0.5 and 5.0.
    pub rating: f32,
    /// The height of this rating increment’s entry in a unit-height histogram,
    /// normalized between 0.0 and 1.0. The increment(s) with the highest
    /// number of ratings will always return 1.0 (unless there are no ratings
    /// for the film).
    pub normalized_weight: f32,
    /// The number of ratings made at this increment.
    pub count: usize,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
struct RegisterRequest {
    /// The username for the new account. Use the /auth/username-check endpoint
    /// to check availability.
    username: String,
    /// The password for the new account.
    password: String,
    /// The email address for the new account.
    email_address: String,
    /// Set to true if the person creating the account has agreed to being at
    /// least 13 years of age, and to accepting Letterboxd’s Terms of Use.
    accept_terms_of_use: bool,
}

#[derive(Serialize, Debug, Clone)]
enum ReportCommentReason {
    Spoilers,
    Spam,
    Plagiarism,
    Other,
}

#[derive(Serialize, Debug, Clone)]
struct ReportCommentRequest {
    ///  The reason why the comment was reported.
    reason: ReportCommentReason,
    /// An optional, explanatory message to accompany the report. Required if
    /// the reason is Plagiarism or Other.
    message: Option<String>,
}

#[derive(Serialize, Debug, Clone)]
enum ReportFilmReason {
    Duplicate,
    NotAFilm,
    Other,
}

#[derive(Serialize, Debug, Clone)]
struct ReportFilmRequest {
    /// The reason why the film was reported.
    reason: ReportFilmReason,
    /// An optional, explanatory message to accompany the report. Required if
    /// the reason is Duplicate or Other.
    message: Option<String>,
}

#[derive(Serialize, Debug, Clone)]
enum ReportListReason {
    Spoilers,
    Spam,
    Plagiarism,
    Other,
}

#[derive(Serialize, Debug, Clone)]
struct ReportListRequest {
    /// The reason why the list was reported.
    reason: ReportListReason,
    /// An optional, explanatory message to accompany the report. Required if
    /// the reason is Plagiarism or Other.
    message: Option<String>,
}

#[derive(Serialize, Debug, Clone)]
enum ReportMemberReason {
    SpamAccount,
    Other,
}

#[derive(Serialize, Debug, Clone)]
struct ReportMemberRequest {
    /// The reason why the member was reported.
    reason: ReportMemberReason,
    /// An optional, explanatory message to accompany the report. Required if
    /// the reason is Other.
    message: Option<String>,
}

#[derive(Serialize, Debug, Clone)]
enum ReportReviewReason {
    Spoilers,
    Spam,
    Plagiarism,
    Other,
}

#[derive(Serialize, Debug, Clone)]
struct ReportReviewRequest {
    /// The reason why the review was reported.
    reason: ReportReviewReason,
    /// An optional, explanatory message to accompany the report. Required if
    /// the reason is Plagiarism or Other.
    message: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum ThirdPartyService {
    Facebook,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Review {
    /// The review text in LBML. May contain the following HTML tags: `<br>`
    /// `<strong>` `<em>` `<b>` `<i>` `<a href="">` `<blockquote>`.
    pub lbml: String,
    /// Will be true if the member has indicated that the review field contains
    /// plot spoilers for the film.
    pub contains_spoilers: bool,
    /// The third-party service or services to which this review can be shared.
    /// Only included if the authenticated member is the review’s owner.
    pub can_share_on: Option<ThirdPartyService>,
    /// The third-party service or services to which this review has been shared. Only included if the authenticated member is the review’s owner.
    pub shared_on: Option<ThirdPartyService>,
    /// The timestamp when this log entry’s review was first published, in ISO
    /// 8601 format with UTC timezone, i.e. YYYY-MM-DDThh:mm:ssZ
    /// "1997-08-29T07:14:00Z"
    pub when_reviewed: String,
    /// The review text formatted as HTML.
    pub text: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct ReviewComment {
    /// The LID of the comment.
    id: String,
    /// The member who posted the comment.
    member: MemberSummary,
    /// ISO 8601 format with UTC timezone, i.e. YYYY-MM-DDThh:mm:ssZ
    /// "1997-08-29T07:14:00Z"
    when_created: String,
    /// ISO 8601 format with UTC timezone, i.e. YYYY-MM-DDThh:mm:ssZ
    /// "1997-08-29T07:14:00Z"
    when_updated: String,
    /// The message portion of the comment in LBML. May contain the following
    /// HTML tags: `<br>` `<strong>` `<em>` `<b>` `<i>` `<a href="">`
    /// `<blockquote>`.
    comment_lbml: String,
    /// If Letterboxd moderators have removed the comment from the site,
    /// removedByAdmin will be true and comment will not be included.
    removed_by_admin: bool,
    /// If the comment owner has removed the comment from the site, deleted
    /// will be true and comment will not be included.
    deleted: bool,
    /// If the authenticated member has blocked the commenter, blocked will be
    /// true and comment will not be included.
    blocked: bool,
    /// If the review owner has blocked the commenter, blockedByOwner will be
    /// true and comment will not be included.
    blocked_by_owner: bool,
    /// If the authenticated member posted this comment, and the comment is
    /// still editable, this value shows the number of seconds remaining until
    /// the editing window closes.
    editable_window_expires_in: Option<usize>,
    /// The review on which the comment was posted.
    review: ReviewIdentifier,
    /// The message portion of the comment formatted as HTML.
    comment: String,
}

#[derive(Deserialize, Debug, Clone)]
struct ReviewCommentsResponse {
    /// The cursor to the next page of results.
    next: Option<Cursor>,
    // The list of comments.
    items: Vec<ReviewComment>,
}

#[derive(Deserialize, Debug, Clone)]
struct ReviewIdentifier {
    /// The LID of the log entry.
    id: String,
}

// TODO: order
#[derive(Deserialize, Debug, Clone)]
enum CommentThreadState {
    /// `CanComment` means the authenticated member is authorized to add a
    /// comment. All other
    /// values mean the authenticated member is not authorized to add a comment.
    CanComment,
    /// `Banned` means the Letterboxd community managers have restricted the
    /// member’s ability to
    /// comment on the site.
    Banned,
    /// `Blocked` means the owner has blocked the member from adding comments.
    Blocked,
    /// `NotCommentable` means that it is invalid to try to add comments to
    /// this content.
    NotCommentable,
}

// TODO: order
/// `NotSubscribed` and `Unsubscribed` are maintained as separate states so the
/// UI can, if needed,
/// indicate to the member how their subscription state will be affected
/// if/when they post a
/// comment.
#[derive(Deserialize, Debug, Clone)]
enum SubscriptionState {
    Subscribed,
    NotSubscribed,
    Unsubscribed,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct ReviewRelationship {
    /// Will be true if the member likes the review (via the ‘heart’ icon). A
    /// member may not like their own review.
    liked: bool,
    /// Will be true if the member is subscribed to comment notifications for
    /// the review
    subscribed: bool,
    /// Defaults to Subscribed for the review’s author, and NotSubscribed for
    /// other members. The subscription value may change when a member (other
    /// than the owner) posts a comment, as follows: the member will become
    /// automatically Subscribed unless they have previously Unsubscribed from
    /// the comment thread via the web interface or API, or unless they have
    /// disabled comment notifications in their profile settings.
    subscription_state: SubscriptionState,
    /// The authenticated member’s state with respect to adding comments for
    /// this review.
    comment_thread_state: CommentThreadState,
}

#[derive(Deserialize, Debug, Clone)]
enum ReviewRelationshipUpdateMessageCode {
    LikeBlockedContent,
    LikeOwnReview,
    LikeLogEntryWithoutReview,
    SubscribeWhenOptedOut,
    SubscribeToContentYouBlocked,
    SubscribeToBlockedContent,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "type")]
enum ReviewRelationshipUpdateMessage {
    Error {
        /// The error message code.
        code: ReviewRelationshipUpdateMessageCode,
        /// The error message text in human-readable form.
        title: String,
    },
    Success,
}

#[derive(Serialize, Debug, Clone)]
struct ReviewRelationshipUpdateRequest {
    /// Set to true if the member likes the review (via the ‘heart’ icon). A
    /// member may not like their own review.
    liked: bool,
    /// Set to true to subscribe the member to comment notifications for the
    /// review, or false to unsubscribe them. A value of true will be ignored
    /// if the member has disabled comment notifications in their profile
    /// settings.
    subscribed: bool,
}

#[derive(Deserialize, Debug, Clone)]
struct ReviewRelationshipUpdateResponse {
    /// The response object.
    data: ReviewRelationship,
    /// A list of messages the API client should show to the user.
    messages: Vec<ReviewRelationshipUpdateMessage>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct ReviewStatistics {
    /// The log entry for which statistics were requested.
    log_entry: ReviewIdentifier,
    /// The number of comments and likes for the review.
    counts: ReviewStatisticsCounts,
}

#[derive(Deserialize, Debug, Clone)]
struct ReviewStatisticsCounts {
    /// The number of comments for the review.
    comments: usize,
    /// The number of members who like the review.
    likes: usize,
}

#[derive(Deserialize, Debug, Clone)]
struct ReviewUpdateResponse {
    /// The response object.
    data: LogEntry,
    /// A list of messages the API client should show to the user.
    messages: Vec<LogEntryUpdateMessage>,
}

#[derive(Serialize, Debug, Clone)]
pub enum SearchMethod {
    FullText,
    Autocomplete,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SearchRequest {
    /// The pagination cursor.
    pub cursor: Option<Cursor>,
    /// The number of items to include per page (default is 20, maximum is 100).
    pub per_page: Option<usize>,
    /// The word, partial word or phrase to search for.
    pub input: String,
    /// The type of search to perform. Defaults to FullText, which performs a
    /// standard search considering text in all fields. Autocomplete only
    /// searches primary fields.
    pub search_method: Option<SearchMethod>,
    // The types of results to search for. Default to all SearchResultTypes.
    pub include: Option<Vec<SearchResultType>>,
    /// The type of contributor to search for. Implies
    /// include=ContributorSearchItem.
    pub contribution_type: Option<ContributionType>,
}

impl SearchRequest {
    pub fn new(input: String) -> SearchRequest {
        SearchRequest {
            cursor: None,
            per_page: None,
            input: input,
            search_method: None,
            include: None,
            contribution_type: None,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct SearchResponse {
    /// The cursor to the next page of results.
    pub next: Option<Cursor>,
    /// The list of search results.
    pub items: Vec<AbstractSearchItem>,
}

#[derive(Serialize, Debug, Clone)]
pub enum SearchResultType {
    ContributorSearchItem,
    FilmSearchItem,
    ListSearchItem,
    MemberSearchItem,
    /// Details of the review.
    ReviewSearchItem,
    TagSearchItem,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Service {
    /// The LID of the service.
    pub id: String,
    /// The name of the service.
    pub name: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    /// The tag code.
    pub code: String,
    /// The tag text as entered by the tagger.
    pub display_tag: String,
}

#[derive(Deserialize, Debug, Clone)]
struct TagsResponse {
    /// The list of tags, ordered by frequency of use.
    items: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
enum UsernameCheckResult {
    Available,
    NotAvailable,
    TooShort,
    TooLong,
    Invalid,
}

#[derive(Deserialize, Debug, Clone)]
struct UsernameCheckResponse {
    /// Will be Available if the username is available to register, or
    /// NotAvailable if used by another member (or attached to a deactivated
    /// account, or otherwise reserved). May return an appropriate error value
    /// if the username doesn’t meet Letterboxd’s requirements: Usernames must
    /// be between 2 and 15 characters long and may only contain upper or
    /// lowercase letters, numbers or the underscore (_) character.
    result: UsernameCheckResult,
}

#[derive(Serialize, Debug, Clone)]
pub enum WatchlistSort {
    Added,
    FilmName,
    ReleaseDateLatestFirst,
    ReleaseDateEarliestFirst,
    RatingHighToLow,
    RatingLowToHigh,
    FilmDurationShortestFirst,
    FilmDurationLongestFirst,
    FilmPopularity,
    FilmPopularityThisWeek,
    FilmPopularityThisMonth,
    FilmPopularityThisYear,
}

// TODO: order
#[derive(Serialize, Debug, Clone)]
pub enum IncludeFriends {
    None,
    All,
    Only,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct WatchlistRequest {
    /// The pagination cursor.
    cursor: Option<Cursor>,
    /// The number of items to include per page (default is 20, maximum is 100).
    per_page: Option<usize>,
    /// The order in which the entries should be returned. Defaults to Added,
    /// which is the order that the films were added to the watchlist, most
    /// recent first.
    sort: WatchlistSort,
    /// Specify the LID of a genre to limit films to those within the specified
    /// genre.
    genre: String,
    /// Specify the starting year of a decade (must end in 0) to limit films to
    /// those released during the decade. 1990
    decade: u16,
    /// Specify a year to limit films to those released during that year. 1994
    year: u16,
    /// Specify the ID of a supported service to limit films to those available
    /// from that service. The list of available services can be found by using
    /// the /films/film-services endpoint.
    service: String,
    /// Specify one or more values to limit the list of films accordingly.
    /// where=Watched&where=Released
    where_film_status: Vec<FilmStatus>,
    /// Specify the LID of a member to limit the returned films according to
    /// the value set in memberRelationship. The member and memberRelationship
    /// parameters can be used to compute comparisons between the watchlist
    /// owner and another member.
    member: String,
    /// Must be used in conjunction with member. Defaults to Watched. Specify
    /// the type of relationship to limit the returned films accordingly.
    member_relationship: FilmRelationshipType,
    /// Must be used in conjunction with member. Defaults to None, which only
    /// returns films from the member’s account. Use Only to return films from
    /// the member’s friends, and All to return films from both the member and
    /// their friends.
    include_friends: IncludeFriends,
    /// Specify a tag code to limit the returned films to those tagged
    /// accordingly.
    tag_code: String,
    /// Must be used with tag. Specify the LID of a member to focus the tag
    /// filter on the member.
    tagger: String,
    /// Must be used in conjunction with tagger. Defaults to None, which
    /// filters tags set by the member. Use Only to filter tags set by the
    /// member’s friends, and All to filter tags set by both the member and
    /// their friends.
    include_tagger_friends: IncludeFriends,
}
