//! Right-to-left interface tests: with Arabic active, the chrome mirrors and the text
//! engine reads right to left.
//!
//! The language is a process-global [`i18n`] setting, and tests in one binary run on
//! threads, so every test here holds `LANG` while it sets the language and asserts.

use craft_anything_engine::Mode;
use craft_anything_ui_egui::{direction, i18n, theme, CraftApp};
use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;

static LANG: std::sync::Mutex<()> = std::sync::Mutex::new(());

fn harness(make: impl FnOnce() -> CraftApp + 'static) -> Harness<'static, CraftApp> {
    Harness::builder().with_size(egui::vec2(1280.0, 820.0)).with_max_steps(32).build_eframe(move |cc| {
        CraftApp::setup_context(&cc.egui_ctx);
        make()
    })
}

fn arabic_app(mode: Mode) -> CraftApp {
    i18n::set_language("ar");
    let engine = std::sync::Arc::new(craft_anything_engine::Engine::new(craft_anything_engine::Session::new(
        craft_anything_engine::Settings { mode, first_run_done: true, language: "ar".into(), ..Default::default() },
    )));
    craft_anything_ui_egui::demo::app(engine)
}

/// Arabic reads right to left; English reads left to right.
#[test]
fn text_direction_follows_the_language() {
    let _lang = LANG.lock().unwrap();
    i18n::set_language("ar");
    assert!(direction::rtl());
    assert_eq!(direction::text_direction(), egui::epaint::text::TextDirection::RightToLeft);
    i18n::set_language("en");
    assert!(!direction::rtl());
    assert_eq!(direction::text_direction(), egui::epaint::text::TextDirection::LeftToRight);
}

/// Rows start at the reading edge, trailing rows pack against the other one, and stacks
/// hug the reading edge.
#[test]
fn row_layouts_follow_the_language() {
    let _lang = LANG.lock().unwrap();
    i18n::set_language("ar");
    assert_eq!(direction::row(egui::Align::Center).main_dir(), egui::Direction::RightToLeft);
    assert_eq!(direction::row_end(egui::Align::Center).main_dir(), egui::Direction::LeftToRight);
    assert_eq!(direction::column().cross_align(), egui::Align::Max);
    i18n::set_language("en");
    assert_eq!(direction::row(egui::Align::Center).main_dir(), egui::Direction::LeftToRight);
    assert_eq!(direction::row_end(egui::Align::Center).main_dir(), egui::Direction::RightToLeft);
    assert_eq!(direction::column().cross_align(), egui::Align::Min);
}

/// A collapsed row's chevron points into the hidden body: right in English, left in Arabic.
#[test]
fn disclosure_chevron_points_into_the_body() {
    let _lang = LANG.lock().unwrap();
    i18n::set_language("en");
    assert_eq!(direction::chevron_collapsed(), "chevron-right");
    i18n::set_language("ar");
    assert_eq!(direction::chevron_collapsed(), "chevron-left");
}

/// The theme carries the fork's chrome flag: scroll bars, menus, popups and the resize
/// grip mirror with it.
#[test]
fn theme_sets_the_chrome_direction_flag() {
    let _lang = LANG.lock().unwrap();
    let ctx = egui::Context::default();
    i18n::set_language("ar");
    theme::apply(&ctx, theme::ThemeKind::Studio, false);
    assert!(ctx.global_style().right_to_left);
    i18n::set_language("en");
    theme::apply(&ctx, theme::ThemeKind::Studio, false);
    assert!(!ctx.global_style().right_to_left);
}

/// In Arabic the title bar's app controls flow from the left, while the OS window buttons
/// keep the physical right.
#[test]
fn titlebar_controls_swap_sides_but_the_window_buttons_do_not() {
    let _lang = LANG.lock().unwrap();
    let mut h = harness(|| arabic_app(Mode::Beginner));
    h.run_steps(4);
    assert!(direction::rtl());
    // The Clean room / Deep switch is a title-bar control: it moves to the left half.
    let label = i18n::tr("Clean room");
    let toggle = h.get_by_label(label.as_str());
    assert!(toggle.rect().center().x < 640.0, "toggle on the left: {:?}", toggle.rect());
    if !craft_anything_ui_egui::titlebar::NATIVE_FRAME {
        let close_label = i18n::tr("Close");
        let close = h.get_by_label(close_label.as_str());
        assert!(close.rect().center().x > 1000.0, "Close pinned right: {:?}", close.rect());
    }
}

/// The Arabic settings dialog lays out: its tabs and every row's control keep real room.
/// A plain vertical inside a mirrored row once stranded the rest of the row with zero
/// width, collapsing the dialog (a debug panic; overlapped rows in release).
#[test]
fn arabic_settings_rows_keep_room_for_their_controls() {
    let _lang = LANG.lock().unwrap();
    let mut h = harness(|| arabic_app(Mode::Beginner));
    h.run_steps(3);
    h.state_mut().state.settings_open = true;
    h.run_steps(4);
    let tab_label = i18n::tr("General");
    let tab = h.get_by_label(tab_label.as_str());
    assert!(tab.rect().width() > 100.0, "settings tabs keep their width: {:?}", tab.rect());
    let change_label = i18n::tr("Change…");
    let change = h.get_by_label(change_label.as_str());
    assert!(change.rect().width() > 20.0, "row controls keep their room: {:?}", change.rect());
    // Mirrored row order: the control sits on the opposite side from its heading, at the
    // row's left, while the tab it belongs to is at the dialog's right.
    assert!(change.rect().center().x < tab.rect().center().x, "control left of tabs");
}
