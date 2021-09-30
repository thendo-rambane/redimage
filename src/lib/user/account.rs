use serde::{Deserialize, Serialize};

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
