use chrono::{DateTime, Utc};
use log::info;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use tokio::time::{interval, Duration};

use crate::cursor_tracker::CursorTracker;
use crate::keyboard_tracker::KeyboardTracker;

pub struct AFKTracker {
    last_activity: Arc<RwLock<DateTime<Utc>>>,
    afk_start: Arc<Mutex<Option<DateTime<Utc>>>>,
    afk_threshold: Duration,
    cursor_tracker: Arc<CursorTracker>,
    keyboard_tracker: Arc<KeyboardTracker>,
    // db: Arc<DBContext>,
}

impl AFKTracker {
    pub fn new(
        cursor_tracker: Arc<CursorTracker>,
        keyboard_tracker: Arc<KeyboardTracker>,
        afk_timeout: u64,
        // db: Arc<DBContext>,
    ) -> Self {
        Self {
            last_activity: Arc::new(RwLock::new(Utc::now())),
            afk_start: Arc::new(Mutex::new(None)),
            afk_threshold: Duration::from_secs(afk_timeout),
            cursor_tracker,
            keyboard_tracker,
            // db,
        }
    }

    pub fn start_tracking(self: Arc<Self>) {
        let last_activity = Arc::clone(&self.last_activity);
        let afk_start = Arc::clone(&self.afk_start);
        let afk_threshold = self.afk_threshold;
        let cursor_tracker = Arc::clone(&self.cursor_tracker);
        let keyboard_tracker = Arc::clone(&self.keyboard_tracker);
        // let db = Arc::clone(&self.db);

        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(1));
            loop {
                interval.tick().await;

                let now = Utc::now();
                let last_activity_time = *last_activity.read().await;
                let mut afk_time = afk_start.lock().await;

                // Detect user activity (mouse/keyboard)
                let mouse_buttons = cursor_tracker.get_pressed_mouse_buttons();
                let keys_pressed = keyboard_tracker.get_pressed_keys();

                let mouse_active = cursor_tracker.has_mouse_moved();
                let mouse_clicked = mouse_buttons.left
                    || mouse_buttons.right
                    || mouse_buttons.middle
                    || mouse_buttons.other;
                let keyboard_active = !keys_pressed.is_empty();

                let activity_detected = mouse_active || mouse_clicked || keyboard_active;

                if activity_detected {
                    *last_activity.write().await = now;

                    if let Some(afk_start_time) = *afk_time {
                        let afk_duration = (now - afk_start_time).num_seconds();
                        info!(
                            "User returned at: {} (AFK Duration: {}s)",
                            now, afk_duration
                        );

                        // let db = db.clone();
                        // let afk_event = AFKEvent {
                        //     id: None,
                        //     afk_start: afk_start_time,
                        //     afk_end: Some(now),
                        //     duration: Some(afk_duration),
                        // };

                        // drop(tokio::spawn(async move {
                        //     if let Err(e) = afk_event.insert(&db).await {
                        //         error!("Failed to insert AFK event: {:?}", e);
                        //     }
                        // }));
                    }
                    *afk_time = None;
                } else {
                    let idle_duration = (now - last_activity_time).num_seconds();
                    if idle_duration >= afk_threshold.as_secs() as i64 && afk_time.is_none() {
                        info!("User went AFK at: {}", now);
                        *afk_time = Some(now);
                    }
                }
            }
        });
    }
}
