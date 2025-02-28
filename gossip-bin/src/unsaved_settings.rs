use gossip_lib::{Error, RunState, Storage, GLOBALS};
use nostr_types::PublicKey;
use paste::paste;

macro_rules! load_setting {
    ($field:ident) => {
        paste! {
            GLOBALS.db().[<read_setting_ $field>]()
        }
    };
}

macro_rules! default_setting {
    ($field:ident) => {
        paste! {
            Storage::[<get_default_setting_ $field>]()
        }
    };
}

macro_rules! save_setting {
    ($field:ident, $slf:ident, $txn:ident) => {
        paste! {
            GLOBALS.db().[<write_setting_ $field>](&$slf.$field, Some(&mut $txn))?;
        }
    };
}

/// Settings are stored in GLOBALS.db() individually. Usually we don't need them together
/// as an object. But the UI uses this to cache changes before committing them.
///
/// NOTE: It is recommended to NOT use this structure. Instead, just interact with each
/// setting key individually via `GLOBALS.db()`
#[derive(Clone, Debug, PartialEq)]
pub struct UnsavedSettings {
    // ID settings
    pub public_key: Option<PublicKey>,
    pub log_n: u8,
    pub login_at_startup: bool,

    // Network settings
    pub offline: bool,
    pub load_avatars: bool,
    pub load_media: bool,
    pub check_nip05: bool,
    pub automatically_fetch_metadata: bool,
    pub relay_connection_requires_approval: bool,
    pub relay_auth_requires_approval: bool,

    // Relay settings
    pub num_relays_per_person: u8,
    pub max_relays: u8,

    // Feed Settings
    pub load_more_count: u64,

    // Event Selection
    pub reposts: bool,
    pub show_long_form: bool,
    pub show_mentions: bool,
    pub direct_messages: bool,
    pub future_allowance_secs: u64,

    // Event Content Settings
    pub hide_mutes_entirely: bool,
    pub reactions: bool,
    pub enable_zap_receipts: bool,
    pub show_media: bool,
    pub approve_content_warning: bool,
    pub show_deleted_events: bool,
    pub avoid_spam_on_unsafe_relays: bool,
    pub apply_spam_filter_on_incoming_events: bool,
    pub apply_spam_filter_on_threads: bool,
    pub apply_spam_filter_on_inbox: bool,
    pub apply_spam_filter_on_global: bool,

    // Posting Settings
    pub pow: u8,
    pub set_client_tag: bool,
    pub set_user_agent: bool,
    pub delegatee_tag: String,

    // UI settings
    pub max_fps: u32,
    pub recompute_feed_periodically: bool,
    pub feed_recompute_interval_ms: u32,
    pub feed_thread_scroll_to_main_event: bool,
    pub theme_variant: String,
    pub dark_mode: bool,
    pub follow_os_dark_mode: bool,
    pub override_dpi: Option<u32>,
    pub highlight_unread_events: bool,
    pub feed_newest_at_bottom: bool,
    pub posting_area_at_top: bool,
    pub status_bar: bool,
    pub image_resize_algorithm: String,
    pub inertial_scrolling: bool,
    pub mouse_acceleration: f32,
    pub wgpu_renderer: bool,

    // Staletime settings
    pub relay_list_becomes_stale_minutes: u64,
    pub metadata_becomes_stale_minutes: u64,
    pub nip05_becomes_stale_if_valid_hours: u64,
    pub nip05_becomes_stale_if_invalid_minutes: u64,
    pub avatar_becomes_stale_hours: u64,
    pub media_becomes_stale_hours: u64,

    // Websocket settings
    pub max_websocket_message_size_kb: usize,
    pub max_websocket_frame_size_kb: usize,
    pub websocket_accept_unmasked_frames: bool,
    pub websocket_connect_timeout_sec: u64,
    pub websocket_ping_frequency_sec: u64,

    // HTTP settings
    pub fetcher_connect_timeout_sec: u64,
    pub fetcher_timeout_sec: u64,
    pub fetcher_max_requests_per_host: usize,
    pub fetcher_host_exclusion_on_low_error_secs: u64,
    pub fetcher_host_exclusion_on_med_error_secs: u64,
    pub fetcher_host_exclusion_on_high_error_secs: u64,

    // Database settings
    pub prune_period_days: u64,
    pub cache_prune_period_days: u64,

    pub blossom_servers: String,
}

impl Default for UnsavedSettings {
    fn default() -> UnsavedSettings {
        UnsavedSettings {
            public_key: default_setting!(public_key),
            log_n: default_setting!(log_n),
            login_at_startup: default_setting!(login_at_startup),
            offline: default_setting!(offline),
            load_avatars: default_setting!(load_avatars),
            load_media: default_setting!(load_media),
            check_nip05: default_setting!(check_nip05),
            automatically_fetch_metadata: default_setting!(automatically_fetch_metadata),
            relay_connection_requires_approval: default_setting!(
                relay_connection_requires_approval
            ),
            relay_auth_requires_approval: default_setting!(relay_auth_requires_approval),
            num_relays_per_person: default_setting!(num_relays_per_person),
            max_relays: default_setting!(max_relays),
            load_more_count: default_setting!(load_more_count),
            reposts: default_setting!(reposts),
            show_long_form: default_setting!(show_long_form),
            show_mentions: default_setting!(show_mentions),
            direct_messages: default_setting!(direct_messages),
            future_allowance_secs: default_setting!(future_allowance_secs),
            hide_mutes_entirely: default_setting!(hide_mutes_entirely),
            reactions: default_setting!(reactions),
            enable_zap_receipts: default_setting!(enable_zap_receipts),
            show_media: default_setting!(show_media),
            approve_content_warning: default_setting!(approve_content_warning),
            show_deleted_events: default_setting!(show_deleted_events),
            avoid_spam_on_unsafe_relays: default_setting!(avoid_spam_on_unsafe_relays),
            apply_spam_filter_on_incoming_events: default_setting!(
                apply_spam_filter_on_incoming_events
            ),
            apply_spam_filter_on_threads: default_setting!(apply_spam_filter_on_threads),
            apply_spam_filter_on_inbox: default_setting!(apply_spam_filter_on_inbox),
            apply_spam_filter_on_global: default_setting!(apply_spam_filter_on_global),
            pow: default_setting!(pow),
            set_client_tag: default_setting!(set_client_tag),
            set_user_agent: default_setting!(set_user_agent),
            delegatee_tag: default_setting!(delegatee_tag),
            max_fps: default_setting!(max_fps),
            recompute_feed_periodically: default_setting!(recompute_feed_periodically),
            feed_recompute_interval_ms: default_setting!(feed_recompute_interval_ms),
            feed_thread_scroll_to_main_event: default_setting!(feed_thread_scroll_to_main_event),
            theme_variant: default_setting!(theme_variant),
            dark_mode: default_setting!(dark_mode),
            follow_os_dark_mode: default_setting!(follow_os_dark_mode),
            override_dpi: default_setting!(override_dpi),
            highlight_unread_events: default_setting!(highlight_unread_events),
            feed_newest_at_bottom: default_setting!(feed_newest_at_bottom),
            posting_area_at_top: default_setting!(posting_area_at_top),
            status_bar: default_setting!(status_bar),
            image_resize_algorithm: default_setting!(image_resize_algorithm),
            inertial_scrolling: default_setting!(inertial_scrolling),
            mouse_acceleration: default_setting!(mouse_acceleration),
            wgpu_renderer: default_setting!(wgpu_renderer),
            relay_list_becomes_stale_minutes: default_setting!(relay_list_becomes_stale_minutes),
            metadata_becomes_stale_minutes: default_setting!(metadata_becomes_stale_minutes),
            nip05_becomes_stale_if_valid_hours: default_setting!(
                nip05_becomes_stale_if_valid_hours
            ),
            nip05_becomes_stale_if_invalid_minutes: default_setting!(
                nip05_becomes_stale_if_invalid_minutes
            ),
            avatar_becomes_stale_hours: default_setting!(avatar_becomes_stale_hours),
            media_becomes_stale_hours: default_setting!(media_becomes_stale_hours),
            max_websocket_message_size_kb: default_setting!(max_websocket_message_size_kb),
            max_websocket_frame_size_kb: default_setting!(max_websocket_frame_size_kb),
            websocket_accept_unmasked_frames: default_setting!(websocket_accept_unmasked_frames),
            websocket_connect_timeout_sec: default_setting!(websocket_connect_timeout_sec),
            websocket_ping_frequency_sec: default_setting!(websocket_ping_frequency_sec),
            fetcher_connect_timeout_sec: default_setting!(fetcher_connect_timeout_sec),
            fetcher_timeout_sec: default_setting!(fetcher_timeout_sec),
            fetcher_max_requests_per_host: default_setting!(fetcher_max_requests_per_host),
            fetcher_host_exclusion_on_low_error_secs: default_setting!(
                fetcher_host_exclusion_on_low_error_secs
            ),
            fetcher_host_exclusion_on_med_error_secs: default_setting!(
                fetcher_host_exclusion_on_med_error_secs
            ),
            fetcher_host_exclusion_on_high_error_secs: default_setting!(
                fetcher_host_exclusion_on_high_error_secs
            ),
            prune_period_days: default_setting!(prune_period_days),
            cache_prune_period_days: default_setting!(prune_period_days),
            blossom_servers: default_setting!(blossom_servers),
        }
    }
}

impl UnsavedSettings {
    pub fn load() -> UnsavedSettings {
        UnsavedSettings {
            public_key: load_setting!(public_key),
            log_n: load_setting!(log_n),
            login_at_startup: load_setting!(login_at_startup),
            offline: load_setting!(offline),
            load_avatars: load_setting!(load_avatars),
            load_media: load_setting!(load_media),
            check_nip05: load_setting!(check_nip05),
            automatically_fetch_metadata: load_setting!(automatically_fetch_metadata),
            relay_connection_requires_approval: load_setting!(relay_connection_requires_approval),
            relay_auth_requires_approval: load_setting!(relay_auth_requires_approval),
            num_relays_per_person: load_setting!(num_relays_per_person),
            max_relays: load_setting!(max_relays),
            load_more_count: load_setting!(load_more_count),
            reposts: load_setting!(reposts),
            show_long_form: load_setting!(show_long_form),
            show_mentions: load_setting!(show_mentions),
            direct_messages: load_setting!(direct_messages),
            future_allowance_secs: load_setting!(future_allowance_secs),
            hide_mutes_entirely: load_setting!(hide_mutes_entirely),
            reactions: load_setting!(reactions),
            enable_zap_receipts: load_setting!(enable_zap_receipts),
            show_media: load_setting!(show_media),
            approve_content_warning: load_setting!(approve_content_warning),
            show_deleted_events: load_setting!(show_deleted_events),
            avoid_spam_on_unsafe_relays: load_setting!(avoid_spam_on_unsafe_relays),
            apply_spam_filter_on_incoming_events: load_setting!(
                apply_spam_filter_on_incoming_events
            ),
            apply_spam_filter_on_threads: load_setting!(apply_spam_filter_on_threads),
            apply_spam_filter_on_inbox: load_setting!(apply_spam_filter_on_inbox),
            apply_spam_filter_on_global: load_setting!(apply_spam_filter_on_global),
            pow: load_setting!(pow),
            set_client_tag: load_setting!(set_client_tag),
            set_user_agent: load_setting!(set_user_agent),
            delegatee_tag: load_setting!(delegatee_tag),
            max_fps: load_setting!(max_fps),
            recompute_feed_periodically: load_setting!(recompute_feed_periodically),
            feed_recompute_interval_ms: load_setting!(feed_recompute_interval_ms),
            feed_thread_scroll_to_main_event: load_setting!(feed_thread_scroll_to_main_event),
            theme_variant: load_setting!(theme_variant),
            dark_mode: load_setting!(dark_mode),
            follow_os_dark_mode: load_setting!(follow_os_dark_mode),
            override_dpi: load_setting!(override_dpi),
            highlight_unread_events: load_setting!(highlight_unread_events),
            feed_newest_at_bottom: load_setting!(feed_newest_at_bottom),
            posting_area_at_top: load_setting!(posting_area_at_top),
            status_bar: load_setting!(status_bar),
            image_resize_algorithm: load_setting!(image_resize_algorithm),
            inertial_scrolling: load_setting!(inertial_scrolling),
            mouse_acceleration: load_setting!(mouse_acceleration),
            wgpu_renderer: load_setting!(wgpu_renderer),
            relay_list_becomes_stale_minutes: load_setting!(relay_list_becomes_stale_minutes),
            metadata_becomes_stale_minutes: load_setting!(metadata_becomes_stale_minutes),
            nip05_becomes_stale_if_valid_hours: load_setting!(nip05_becomes_stale_if_valid_hours),
            nip05_becomes_stale_if_invalid_minutes: load_setting!(
                nip05_becomes_stale_if_invalid_minutes
            ),
            avatar_becomes_stale_hours: load_setting!(avatar_becomes_stale_hours),
            media_becomes_stale_hours: load_setting!(media_becomes_stale_hours),
            max_websocket_message_size_kb: load_setting!(max_websocket_message_size_kb),
            max_websocket_frame_size_kb: load_setting!(max_websocket_frame_size_kb),
            websocket_accept_unmasked_frames: load_setting!(websocket_accept_unmasked_frames),
            websocket_connect_timeout_sec: load_setting!(websocket_connect_timeout_sec),
            websocket_ping_frequency_sec: load_setting!(websocket_ping_frequency_sec),
            fetcher_connect_timeout_sec: load_setting!(fetcher_connect_timeout_sec),
            fetcher_timeout_sec: load_setting!(fetcher_timeout_sec),
            fetcher_max_requests_per_host: load_setting!(fetcher_max_requests_per_host),
            fetcher_host_exclusion_on_low_error_secs: load_setting!(
                fetcher_host_exclusion_on_low_error_secs
            ),
            fetcher_host_exclusion_on_med_error_secs: load_setting!(
                fetcher_host_exclusion_on_med_error_secs
            ),
            fetcher_host_exclusion_on_high_error_secs: load_setting!(
                fetcher_host_exclusion_on_high_error_secs
            ),
            prune_period_days: load_setting!(prune_period_days),
            cache_prune_period_days: load_setting!(cache_prune_period_days),
            blossom_servers: load_setting!(blossom_servers),
        }
    }

    pub fn save(&self) -> Result<(), Error> {
        let mut txn = GLOBALS.db().get_write_txn()?;
        save_setting!(public_key, self, txn);
        save_setting!(log_n, self, txn);
        save_setting!(login_at_startup, self, txn);
        save_setting!(offline, self, txn);
        save_setting!(load_avatars, self, txn);
        save_setting!(load_media, self, txn);
        save_setting!(check_nip05, self, txn);
        save_setting!(automatically_fetch_metadata, self, txn);
        save_setting!(relay_connection_requires_approval, self, txn);
        save_setting!(relay_auth_requires_approval, self, txn);
        save_setting!(num_relays_per_person, self, txn);
        save_setting!(max_relays, self, txn);
        save_setting!(load_more_count, self, txn);
        save_setting!(reposts, self, txn);
        save_setting!(show_long_form, self, txn);
        save_setting!(show_mentions, self, txn);
        save_setting!(direct_messages, self, txn);
        save_setting!(future_allowance_secs, self, txn);
        save_setting!(hide_mutes_entirely, self, txn);
        save_setting!(reactions, self, txn);
        save_setting!(enable_zap_receipts, self, txn);
        save_setting!(show_media, self, txn);
        save_setting!(approve_content_warning, self, txn);
        save_setting!(show_deleted_events, self, txn);
        save_setting!(avoid_spam_on_unsafe_relays, self, txn);
        save_setting!(apply_spam_filter_on_incoming_events, self, txn);
        save_setting!(apply_spam_filter_on_threads, self, txn);
        save_setting!(apply_spam_filter_on_inbox, self, txn);
        save_setting!(apply_spam_filter_on_global, self, txn);
        save_setting!(pow, self, txn);
        save_setting!(set_client_tag, self, txn);
        save_setting!(set_user_agent, self, txn);
        save_setting!(delegatee_tag, self, txn);
        save_setting!(max_fps, self, txn);
        save_setting!(recompute_feed_periodically, self, txn);
        save_setting!(feed_recompute_interval_ms, self, txn);
        save_setting!(feed_thread_scroll_to_main_event, self, txn);
        save_setting!(theme_variant, self, txn);
        save_setting!(dark_mode, self, txn);
        save_setting!(follow_os_dark_mode, self, txn);
        save_setting!(override_dpi, self, txn);
        save_setting!(highlight_unread_events, self, txn);
        save_setting!(feed_newest_at_bottom, self, txn);
        save_setting!(posting_area_at_top, self, txn);
        save_setting!(status_bar, self, txn);
        save_setting!(image_resize_algorithm, self, txn);
        save_setting!(inertial_scrolling, self, txn);
        save_setting!(mouse_acceleration, self, txn);
        save_setting!(wgpu_renderer, self, txn);
        save_setting!(relay_list_becomes_stale_minutes, self, txn);
        save_setting!(metadata_becomes_stale_minutes, self, txn);
        save_setting!(nip05_becomes_stale_if_valid_hours, self, txn);
        save_setting!(nip05_becomes_stale_if_invalid_minutes, self, txn);
        save_setting!(avatar_becomes_stale_hours, self, txn);
        save_setting!(media_becomes_stale_hours, self, txn);
        save_setting!(max_websocket_message_size_kb, self, txn);
        save_setting!(max_websocket_frame_size_kb, self, txn);
        save_setting!(websocket_accept_unmasked_frames, self, txn);
        save_setting!(websocket_connect_timeout_sec, self, txn);
        save_setting!(websocket_ping_frequency_sec, self, txn);
        save_setting!(fetcher_connect_timeout_sec, self, txn);
        save_setting!(fetcher_timeout_sec, self, txn);
        save_setting!(fetcher_max_requests_per_host, self, txn);
        save_setting!(fetcher_host_exclusion_on_low_error_secs, self, txn);
        save_setting!(fetcher_host_exclusion_on_med_error_secs, self, txn);
        save_setting!(fetcher_host_exclusion_on_high_error_secs, self, txn);
        save_setting!(prune_period_days, self, txn);
        save_setting!(cache_prune_period_days, self, txn);
        save_setting!(blossom_servers, self, txn);
        txn.commit()?;

        let runstate = *GLOBALS.read_runstate.borrow();
        if self.offline && runstate == RunState::Online {
            let _ = GLOBALS.write_runstate.send(RunState::Offline);
        } else if !self.offline && runstate == RunState::Offline {
            let _ = GLOBALS.write_runstate.send(RunState::Online);
        }

        Ok(())
    }
}
