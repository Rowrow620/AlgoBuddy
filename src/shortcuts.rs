use std::fmt;

use eframe::egui::{Event, Key};

pub(crate) const SHORTCUTS_STORAGE_KEY: &str = "algobuddy_shortcuts_v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum ShortcutAction {
    PlayPause,
    PreviousStep,
    NextStep,
    ResetTimeline,
    SpeedUp,
    SpeedDown,
    ZoomIn,
    ZoomOut,
    ResetZoom,
}

impl ShortcutAction {
    pub(crate) const ALL: [Self; 9] = [
        Self::PlayPause,
        Self::PreviousStep,
        Self::NextStep,
        Self::ResetTimeline,
        Self::SpeedUp,
        Self::SpeedDown,
        Self::ZoomIn,
        Self::ZoomOut,
        Self::ResetZoom,
    ];

    pub(crate) const fn settings_label(self) -> &'static str {
        match self {
            Self::PlayPause => "Play / Pause Timeline",
            Self::PreviousStep => "Previous Step",
            Self::NextStep => "Next Step",
            Self::ResetTimeline => "Reset Timeline to Step 1",
            Self::SpeedUp => "Speed Up Playback",
            Self::SpeedDown => "Slow Down Playback",
            Self::ZoomIn => "Zoom In Visualization",
            Self::ZoomOut => "Zoom Out Visualization",
            Self::ResetZoom => "Reset Visualization Zoom to 100%",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub(crate) struct ShortcutBindings {
    play_pause: Key,
    previous_step: Key,
    next_step: Key,
    reset_timeline: Key,
    speed_up: Key,
    speed_down: Key,
    zoom_in: Key,
    zoom_out: Key,
    reset_zoom: Key,
}

impl Default for ShortcutBindings {
    fn default() -> Self {
        Self {
            play_pause: Key::Space,
            previous_step: Key::ArrowLeft,
            next_step: Key::ArrowRight,
            reset_timeline: Key::R,
            speed_up: Key::ArrowUp,
            speed_down: Key::ArrowDown,
            zoom_in: Key::Plus,
            zoom_out: Key::Minus,
            reset_zoom: Key::Num0,
        }
    }
}

impl ShortcutBindings {
    pub(crate) const fn key(self, action: ShortcutAction) -> Key {
        match action {
            ShortcutAction::PlayPause => self.play_pause,
            ShortcutAction::PreviousStep => self.previous_step,
            ShortcutAction::NextStep => self.next_step,
            ShortcutAction::ResetTimeline => self.reset_timeline,
            ShortcutAction::SpeedUp => self.speed_up,
            ShortcutAction::SpeedDown => self.speed_down,
            ShortcutAction::ZoomIn => self.zoom_in,
            ShortcutAction::ZoomOut => self.zoom_out,
            ShortcutAction::ResetZoom => self.reset_zoom,
        }
    }

    pub(crate) fn key_label(self, action: ShortcutAction) -> &'static str {
        key_display_label(self.key(action))
    }

    pub(crate) fn hint(self, action: ShortcutAction, description: &str) -> String {
        format!("{description} (Shortcut: {})", self.key_label(action))
    }

    pub(crate) fn action_for_key(self, key: Key) -> Option<ShortcutAction> {
        let key = normalize_shortcut_key(key);
        ShortcutAction::ALL
            .into_iter()
            .find(|&action| normalize_shortcut_key(self.key(action)) == key)
    }

    pub(crate) fn try_rebind(
        &mut self,
        action: ShortcutAction,
        key: Key,
    ) -> Result<(), RebindError> {
        let key = normalize_shortcut_key(key);
        if !is_bindable_shortcut_key(key) {
            return Err(RebindError::UnsupportedKey(key));
        }

        if let Some(assigned_action) = self.action_for_key(key) {
            if assigned_action != action {
                return Err(RebindError::AlreadyAssigned {
                    key,
                    action: assigned_action,
                });
            }
        }

        self.set_key(action, key);
        Ok(())
    }

    pub(crate) fn validate(self) -> Result<(), RebindError> {
        for (index, action) in ShortcutAction::ALL.into_iter().enumerate() {
            let key = normalize_shortcut_key(self.key(action));
            if !is_bindable_shortcut_key(key) {
                return Err(RebindError::UnsupportedKey(key));
            }

            if let Some(assigned_action) = ShortcutAction::ALL[..index]
                .iter()
                .copied()
                .find(|&other| normalize_shortcut_key(self.key(other)) == key)
            {
                return Err(RebindError::AlreadyAssigned {
                    key,
                    action: assigned_action,
                });
            }
        }
        Ok(())
    }

    pub(crate) fn validated_or_default(mut self) -> Self {
        for action in ShortcutAction::ALL {
            self.set_key(action, normalize_shortcut_key(self.key(action)));
        }
        if self.validate().is_ok() {
            self
        } else {
            Self::default()
        }
    }

    fn set_key(&mut self, action: ShortcutAction, key: Key) {
        match action {
            ShortcutAction::PlayPause => self.play_pause = key,
            ShortcutAction::PreviousStep => self.previous_step = key,
            ShortcutAction::NextStep => self.next_step = key,
            ShortcutAction::ResetTimeline => self.reset_timeline = key,
            ShortcutAction::SpeedUp => self.speed_up = key,
            ShortcutAction::SpeedDown => self.speed_down = key,
            ShortcutAction::ZoomIn => self.zoom_in = key,
            ShortcutAction::ZoomOut => self.zoom_out = key,
            ShortcutAction::ResetZoom => self.reset_zoom = key,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RebindError {
    UnsupportedKey(Key),
    AlreadyAssigned { key: Key, action: ShortcutAction },
}

impl fmt::Display for RebindError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Self::UnsupportedKey(key) => write!(
                formatter,
                "{} cannot be used as an AlgoBuddy shortcut.",
                key_display_label(key)
            ),
            Self::AlreadyAssigned { key, action } => write!(
                formatter,
                "{} is already assigned to {}.",
                key_display_label(key),
                action.settings_label()
            ),
        }
    }
}

pub(crate) const BINDABLE_KEYS: &[Key] = &[
    Key::Space,
    Key::ArrowLeft,
    Key::ArrowRight,
    Key::ArrowUp,
    Key::ArrowDown,
    Key::A,
    Key::B,
    Key::C,
    Key::D,
    Key::E,
    Key::F,
    Key::G,
    Key::H,
    Key::I,
    Key::J,
    Key::K,
    Key::L,
    Key::M,
    Key::N,
    Key::O,
    Key::P,
    Key::Q,
    Key::R,
    Key::S,
    Key::T,
    Key::U,
    Key::V,
    Key::W,
    Key::X,
    Key::Y,
    Key::Z,
    Key::Num0,
    Key::Num1,
    Key::Num2,
    Key::Num3,
    Key::Num4,
    Key::Num5,
    Key::Num6,
    Key::Num7,
    Key::Num8,
    Key::Num9,
    Key::Plus,
    Key::Minus,
];

pub(crate) fn normalize_shortcut_key(key: Key) -> Key {
    if key == Key::Equals {
        Key::Plus
    } else {
        key
    }
}

pub(crate) fn is_bindable_shortcut_key(key: Key) -> bool {
    BINDABLE_KEYS.contains(&normalize_shortcut_key(key))
}

pub(crate) fn key_display_label(key: Key) -> &'static str {
    match normalize_shortcut_key(key) {
        Key::Space => "Space",
        Key::ArrowLeft => "Left Arrow",
        Key::ArrowRight => "Right Arrow",
        Key::ArrowUp => "Up Arrow",
        Key::ArrowDown => "Down Arrow",
        Key::Plus => "+ / =",
        Key::Minus => "-",
        Key::Colon => ":",
        Key::Comma => ",",
        Key::Backslash => "\\",
        Key::Slash => "/",
        Key::Pipe => "|",
        Key::Questionmark => "?",
        Key::OpenBracket => "[",
        Key::CloseBracket => "]",
        Key::Backtick => "`",
        Key::Period => ".",
        Key::Semicolon => ";",
        Key::Quote => "'",
        key => key.name(),
    }
}

pub(crate) fn plain_pressed_key(event: &Event) -> Option<Key> {
    match event {
        Event::Key {
            key,
            pressed: true,
            repeat: false,
            modifiers,
            ..
        } if !modifiers.alt && !modifiers.ctrl && !modifiers.command && !modifiers.mac_cmd => {
            Some(*key)
        }
        _ => None,
    }
}

pub(crate) fn shortcut_actions_for_events(
    bindings: ShortcutBindings,
    events: &[Event],
) -> Vec<ShortcutAction> {
    let mut actions = Vec::new();
    for event in events {
        let Some(key) = plain_pressed_key(event) else {
            continue;
        };
        let Some(action) = bindings.action_for_key(key) else {
            continue;
        };
        if !actions.contains(&action) {
            actions.push(action);
        }
    }
    actions
}

#[cfg(test)]
mod tests {
    use super::*;
    use eframe::egui::Modifiers;

    fn key_event(key: Key, modifiers: Modifiers, repeat: bool) -> Event {
        Event::Key {
            key,
            physical_key: None,
            pressed: true,
            repeat,
            modifiers,
        }
    }

    #[test]
    fn defaults_are_unique_and_bindable() {
        let bindings = ShortcutBindings::default();
        assert_eq!(bindings.key(ShortcutAction::PlayPause), Key::Space);
        assert_eq!(bindings.key(ShortcutAction::PreviousStep), Key::ArrowLeft);
        assert_eq!(bindings.key(ShortcutAction::NextStep), Key::ArrowRight);
        assert_eq!(bindings.key(ShortcutAction::ResetTimeline), Key::R);
        assert_eq!(bindings.key(ShortcutAction::SpeedUp), Key::ArrowUp);
        assert_eq!(bindings.key(ShortcutAction::SpeedDown), Key::ArrowDown);
        assert_eq!(bindings.key(ShortcutAction::ZoomIn), Key::Plus);
        assert_eq!(bindings.key(ShortcutAction::ZoomOut), Key::Minus);
        assert_eq!(bindings.key(ShortcutAction::ResetZoom), Key::Num0);
        assert_eq!(bindings.validate(), Ok(()));
    }

    #[test]
    fn rebinding_updates_lookup_and_dynamic_hint() {
        let mut bindings = ShortcutBindings::default();
        bindings
            .try_rebind(ShortcutAction::PlayPause, Key::P)
            .unwrap();

        assert_eq!(
            bindings.action_for_key(Key::P),
            Some(ShortcutAction::PlayPause)
        );
        assert_eq!(bindings.action_for_key(Key::Space), None);
        assert_eq!(
            bindings.hint(ShortcutAction::PlayPause, "Play timeline"),
            "Play timeline (Shortcut: P)"
        );
    }

    #[test]
    fn conflicting_and_unsupported_bindings_are_rejected() {
        let mut bindings = ShortcutBindings::default();
        assert!(matches!(
            bindings.try_rebind(ShortcutAction::PlayPause, Key::ArrowRight),
            Err(RebindError::AlreadyAssigned {
                action: ShortcutAction::NextStep,
                ..
            })
        ));
        assert!(matches!(
            bindings.try_rebind(ShortcutAction::PlayPause, Key::Escape),
            Err(RebindError::UnsupportedKey(Key::Escape))
        ));
        assert_eq!(bindings.key(ShortcutAction::PlayPause), Key::Space);
    }

    #[test]
    fn equals_is_an_alias_for_plus() {
        let bindings = ShortcutBindings::default();
        assert_eq!(
            bindings.action_for_key(Key::Equals),
            Some(ShortcutAction::ZoomIn)
        );
        assert_eq!(key_display_label(Key::Plus), "+ / =");
    }

    #[test]
    fn bindings_round_trip_and_missing_fields_use_defaults() {
        let mut bindings = ShortcutBindings::default();
        bindings
            .try_rebind(ShortcutAction::PlayPause, Key::P)
            .unwrap();
        let serialized = serde_json::to_string(&bindings).unwrap();
        assert_eq!(
            serde_json::from_str::<ShortcutBindings>(&serialized).unwrap(),
            bindings
        );

        let partial: ShortcutBindings = serde_json::from_str(r#"{"play_pause":"P"}"#).unwrap();
        assert_eq!(partial.key(ShortcutAction::PlayPause), Key::P);
        assert_eq!(partial.key(ShortcutAction::NextStep), Key::ArrowRight);
    }

    #[test]
    fn invalid_saved_bindings_fall_back_to_defaults() {
        let bindings = ShortcutBindings {
            next_step: Key::ArrowLeft,
            ..ShortcutBindings::default()
        };
        assert_ne!(bindings.validate(), Ok(()));
        assert_eq!(bindings.validated_or_default(), ShortcutBindings::default());
    }

    #[test]
    fn event_matching_accepts_shift_but_not_command_modifiers() {
        let bindings = ShortcutBindings::default();
        let shifted_equals = key_event(
            Key::Equals,
            Modifiers {
                shift: true,
                ..Modifiers::NONE
            },
            false,
        );
        assert_eq!(
            shortcut_actions_for_events(bindings, &[shifted_equals]),
            vec![ShortcutAction::ZoomIn]
        );

        let control_r = key_event(
            Key::R,
            Modifiers {
                ctrl: true,
                command: true,
                ..Modifiers::NONE
            },
            false,
        );
        assert!(shortcut_actions_for_events(bindings, &[control_r]).is_empty());
    }

    #[test]
    fn repeated_key_events_do_not_retrigger_actions() {
        let bindings = ShortcutBindings::default();
        let repeated_space = key_event(Key::Space, Modifiers::NONE, true);
        assert!(shortcut_actions_for_events(bindings, &[repeated_space]).is_empty());
        assert_eq!(
            plain_pressed_key(&key_event(Key::F11, Modifiers::NONE, true)),
            None
        );
    }
}
