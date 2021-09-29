use reqwest::{blocking::Client, header::USER_AGENT};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct RedditResponse<T> {
    kind: String,
    data: T,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Listing<List> {
    after: String,
    before: Option<String>,
    dist: i32,
    modhash: Option<String>,
    geo_filter: String,
    children: Vec<RedditResponse<List>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct AuthResponse {
    access_token: String,
    expires_in: i32,
    scope: String,
    token_type: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct AuthRequestData {
    grant_type: String,
    username: String,
    password: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ExperimentalFeature {
    owner: String,
    variant: String,
    experiment_id: i32,
}
// type ExperimentalFeature = Option<ExperimentalFeature>;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Features {
    mod_service_mute_writes: bool,
    promoted_trend_blanks: bool,
    show_amp_link: bool,
    chat: Option<bool>,
    is_email_permission_required: bool,
    mod_awards: bool,
    mweb_xpromo_revamp_v3: Option<ExperimentalFeature>,
    mweb_xpromo_revamp_v2: Option<ExperimentalFeature>,
    awards_on_streams: bool,
    mweb_xpromo_modal_listing_click_daily_dismissible_ios: bool,
    chat_subreddit: bool,
    cookie_consent_banner: bool,
    modlog_copyright_removal: bool,
    do_not_track: bool,
    mod_service_mute_reads: bool,
    chat_user_settings: bool,
    use_pref_account_deployment: bool,
    mweb_xpromo_interstitial_comments_ios: bool,
    noreferrer_to_noopener: bool,
    premium_subscriptions_table: bool,
    mweb_xpromo_interstitial_comments_android: bool,
    chat_group_rollout: bool,
    resized_styles_images: bool,
    spez_modal: bool,
    mweb_xpromo_modal_listing_click_daily_dismissible_android: bool,
    expensive_coins_package: bool,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
struct Dimensions(i32, i32);
#[derive(Serialize, Deserialize, Debug, Clone)]
struct RichTextFlair {
    /** The string representation of the emoji */
    a: Option<String>,
    /** The type of the flair entry */
    e: String,
    /** URL of the emoji image */
    u: Option<String>,
    /** The text content of a text flair */
    t: Option<String>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Subreddit {
    accept_followers: bool,
    accounts_active: Option<i32>,
    accounts_active_is_fuzzed: bool,
    active_user_count: Option<i32>,
    advertiser_category: String,
    all_original_content: bool,
    allow_chat_post_creation: Option<bool>,
    allow_discovery: bool,
    allow_galleries: bool,
    allow_images: bool,
    allow_polls: bool,
    allow_prediction_contributors: bool,
    allow_predictions: bool,
    allow_predictions_tournament: bool,
    allow_videogifs: bool,
    allow_videos: bool,
    banner_background_color: String,
    banner_background_image: String,
    banner_img: String,
    banner_size: Option<Dimensions>,
    can_assign_link_flair: bool,
    can_assign_user_flair: bool,
    collapse_deleted_comments: bool,
    comment_score_hide_mins: i32,
    community_icon: String,
    community_reviewed: bool,
    created: f32,
    created_utc: f32,
    description: String,
    description_html: Option<String>,
    disable_contributor_requests: bool,
    display_name: String,
    display_name_prefixed: String,
    emojis_custom_size: Option<Dimensions>,
    emojis_enabled: bool,
    free_form_reports: bool,
    has_menu_widget: bool,
    header_img: Option<String>,
    header_size: Option<Dimensions>,
    header_title: Option<String>,
    hide_ads: bool,
    icon_img: String,
    icon_size: Option<Dimensions>,
    id: String,
    is_chat_post_feature_enabled: Option<bool>,
    is_crosspostable_subreddit: Option<bool>,
    is_default_banner: Option<bool>,
    is_default_icon: Option<Vec<bool>>,
    is_enrolled_in_new_modmail: Option<String>,
    key_color: String,
    lang: String,
    link_flair_enabled: bool,
    link_flair_position: String,
    mobile_banner_image: String,
    name: String,
    notification_level: Option<String>,
    original_content_tag_enabled: bool,
    over18: bool,
    prediction_leaderboard_entry_type: String,
    primary_color: String,
    public_description: String,
    public_description_html: Option<String>,
    public_traffic: bool,
    quarantine: bool,
    restrict_commenting: bool,
    restrict_posting: bool,
    should_archive_posts: bool,
    show_media: bool,
    show_media_preview: bool,
    spoilers_enabled: bool,
    submission_type: String,
    submit_link_label: Option<String>,
    submit_text: String,
    submit_text_html: Option<String>,
    submit_text_label: Option<String>,
    subreddit_type: String,
    subscribers: i32,
    suggested_comment_sort: Option<String>,
    title: String,
    url: String,
    user_can_flair_in_sr: Option<String>,
    user_flair_background_color: Option<String>,
    user_flair_css_class: Option<String>,
    user_flair_enabled_in_sr: bool,
    user_flair_position: String,
    user_flair_richtext: Vec<RichTextFlair>,
    user_flair_template_id: Option<String>,
    user_flair_text: Option<String>,
    user_flair_text_color: Option<String>,
    user_flair_type: String,
    user_has_favorited: bool,
    user_is_banned: bool,
    user_is_contributor: bool,
    user_is_moderator: bool,
    user_is_muted: bool,
    user_is_subscriber: bool,
    user_sr_flair_enabled: Option<bool>,
    user_sr_theme_enabled: bool,
    videostream_links_count: Option<i32>,
    whitelist_status: Option<String>,
    wiki_enabled: Option<bool>,
    wls: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Account {
    created: f32,
    created_utc: f32,
    comment_karma: i32,
    has_mod_mail: bool,
    id: String,
    name: String,
    // Number of Reddit coins, only returned for your own user
    coins: Option<i32>,
    // Only returned for your own user
    features: Option<Features>,
    // /** Only returned for your own user */
    force_password_reset: Option<bool>,
    // /** Only returned for your own user */
    gold_creddits: Option<i32>,
    // /** Only returned for your own user */
    gold_expiration: Option<i32>,
    // /** Only returned for your own user */
    has_android_subscription: Option<bool>,
    // /** Only returned for your own user */
    has_external_account: Option<bool>,
    // /** Only returned for your own user */
    has_ios_subscription: Option<bool>,
    // /** Only returned for your own user */
    has_mail: Option<bool>,
    // /** Only returned for your own user */
    has_paypal_subscription: Option<bool>,
    // /** Only returned for your own user */
    has_stripe_subscription: Option<bool>,
    has_subscribed: bool,
    // /** Only returned for your own user */
    has_subscribed_to_premium: Option<bool>,
    has_verified_mail: Option<bool>,
    // /** Only returned for your own user */
    has_visited_new_profile: Option<bool>,
    hide_from_robots: bool,
    // /** Image URL of the user's avatar */
    icon_img: String,
    // /** Only returned for your own user */
    in_beta: Option<bool>,
    // /** Only returned for your own user */
    in_chat: Option<bool>,
    // /** Only returned for your own user */
    in_redesign_beta: Option<bool>,
    // /** Only returned for your own user */
    inbox_count: Option<i32>,
    is_employee: bool,
    // /** Only returned for other users, not yourself */
    is_friend: Option<bool>,
    is_gold: bool,
    is_mod: bool,
    // /** Only returned for your own user */
    is_sponsor: Option<bool>,
    // /** Only returned for your own user */
    is_suspended: Option<bool>,
    link_karma: i32,
    modhash: Option<String>,
    // /** Only returned for your own user */
    new_modmail_exists: Option<bool>,
    // /** Only returned for your own user */
    num_friends: Option<i32>,
    // /** Only returned for your own user */
    oauth_client_id: Option<String>,
    // /** Only returned for your own user */
    over_18: Option<bool>,
    // /** Only returned for your own user */
    pref_autoplay: Option<bool>,
    // /** Only returned for your own user */
    pref_clickgadget: Option<i32>,
    // /** Only returned for your own user */
    pref_geopopular: Option<String>,
    // /** Only returned for your own user */
    pref_nightmode: Option<bool>,
    // /** Only returned for your own user */
    pref_no_profanity: Option<bool>,
    pref_show_snoovatar: bool,
    // /** Only returned for your own user */
    pref_show_trending: Option<bool>,
    // /** Only returned for your own user */
    pref_show_twitter: Option<bool>,
    // /** Only returned for your own user */
    pref_top_karma_subreddits: Option<bool>,
    // /** Only returned for your own user */
    pref_video_autoplay: Option<bool>,
    // /** Only returned for your own user */
    seen_layout_switch: Option<bool>,
    // /** Only returned for your own user */
    seen_premium_adblock_modal: Option<bool>,
    // /** Only returned for your own user */
    seen_redesign_modal: Option<bool>,
    // /** Only returned for your own user */
    seen_subreddit_chat_ftux: Option<bool>,
    // subreddit: Subreddit | null;
    // /** Only returned for your own user */
    suspension_expiration_utc: Option<i32>,
    verified: bool,
}

pub struct RedditApi {
    username: String,
    password: String,
    client_id: String,
    client_secret: Option<String>,
    auth_response: Option<AuthResponse>,
    current_account: Option<Account>,
}

impl RedditApi {
    pub fn new() -> Self {
        RedditApi {
            username: std::env::var("REDDIT_USERNAME").unwrap(),
            password: std::env::var("REDDIT_PASSWORD").unwrap(),
            client_id: std::env::var("REDDIT_CLIENT_ID").unwrap(),
            client_secret: std::env::var("REDDIT_CLIENT_SECRET").ok(),
            auth_response: None,
            current_account: None,
        }
    }
    pub fn authenticate(&mut self) -> Result<AuthResponse, String> {
        //TODO {Thendo}: Handle Errors properly
        let client = Client::new();
        let request_url = format!("https://www.reddit.com/api/v1/access_token");
        let response = client
            .post(request_url)
            .basic_auth(self.client_id.as_str(), self.client_secret.as_ref())
            .body(format!(
                "grant_type=password&username={}&password={}",
                self.username.as_str(),
                self.password.as_str()
            ))
            .send();
        match response {
            Ok(res) => match res.text() {
                Ok(res_text) => match serde_json::from_str::<AuthResponse>(&res_text) {
                    Ok(auth_response) => {
                        self.auth_response = Some(auth_response.clone());
                        Ok(auth_response.clone())
                    }
                    Err(error) => {
                        self.auth_response = None;
                        //TODO {Thendo}: Handle Errors properly
                        Err(error.to_string())
                    }
                },
                //TODO {Thendo}: Handle Errors properly
                Err(error) => Err(error.to_string()),
            },
            //TODO {Thendo}: Handle Errors properly
            Err(err) => Err(err.to_string()),
        }
    }
    pub fn get_account(&mut self) -> Result<Account, String> {
        if let Some(account) = self.current_account.clone() {
            return Ok(account);
        };
        let client = Client::new();
        let request_url = format!("https://oauth.reddit.com/api/v1/me/?raw_json=1");
        let access_token = match self.auth_response.as_ref() {
            Some(auth_response) => Some(auth_response.access_token.as_str()),
            _ => None,
        };
        if access_token.is_some() {
            let response = client
                .get(request_url)
                .bearer_auth(access_token.unwrap())
                .header(
                    USER_AGENT,
                    format!("RustClient:redimage by {}", self.username.as_str()),
                )
                .send();
            match response {
                Ok(res) => match res.text() {
                    Ok(response_text) => {
                        let user_account = serde_json::from_str::<Account>(&response_text);
                        match user_account {
                            Ok(account) => {
                                self.current_account = Some(account);
                                Ok(self.current_account.clone().unwrap())
                            }
                            Err(error) => Err(error.to_string()),
                        }
                    }
                    //TODO {Thendo}: Handle Errors properly
                    Err(error) => Err(error.to_string()),
                },
                //TODO {Thendo}: Handle Errors properly
                Err(error) => Err(error.to_string()),
            }
        } else {
            //TODO {Thendo}: Handle Errors properly
            Err("Unauthenticated".into())
        }
    }
    pub fn get_following(&self, limit: i32) -> Result<RedditResponse<Listing<Subreddit>>, String> {
        let client = Client::new();
        let request_url = format!(
            "https://oauth.reddit.com/subreddits/mine/?limit={limit}&raw_json=1",
            limit = limit // username = "niasrevenge"
                          // username = self.username
        );
        let access_token = match self.auth_response.as_ref() {
            Some(auth_response) => Some(auth_response.access_token.as_str()),
            _ => None,
        };
        if let Some(access_token) = access_token {
            let response = client
                .get(request_url)
                .bearer_auth(access_token)
                .header(
                    USER_AGENT,
                    format!("RustClient:redimage by {}", self.username.as_str()),
                )
                .send();
            match response {
                Ok(res) => match res.text() {
                    Ok(response_text) => {
                        let accounts_followed = serde_json::from_str::<
                            RedditResponse<Listing<Subreddit>>,
                        >(&response_text);
                        match accounts_followed {
                            Ok(accounts_followed) => Ok(accounts_followed),
                            Err(error) => Err(error.to_string()),
                        }
                    }
                    //TODO {Thendo}: Handle Errors properly
                    Err(error) => Err(error.to_string()),
                },
                //TODO {Thendo}: Handle Errors properly
                Err(error) => Err(error.to_string()),
            }
        } else {
            //TODO {Thendo}: Handle Errors properly
            Err("Unauthenticated".into())
        }
    }
}
