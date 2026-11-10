//! Reading-direction helpers: one place that knows whether the interface runs left to
//! right or right to left, so layout sites ask instead of hard-coding a side.
//!
//! The layout helpers use only long-stable egui APIs; `text_direction` needs the patched
//! 0.37 fork PR-1 moved to (`TextDirection` does not exist on stock 0.36).
//!
//! Icons that must NOT mirror (kept identical in both directions): play, close, check,
//! alerts, plus, search, and the up/down chevrons. Only sideways pointers flip.

/// Whether the interface currently reads right to left (Arabic today).
pub fn rtl() -> bool {
    crate::i18n::is_rtl()
}

/// The text base direction for painted interface chrome (labels, headings, pills): every
/// paragraph reads right to left when mirrored, left to right otherwise. Editable and
/// data content instead keeps the shaper's `Auto` default, which follows each paragraph's
/// own first strong character.
pub fn text_direction() -> egui::epaint::text::TextDirection {
    use egui::epaint::text::TextDirection;
    pick(rtl(), TextDirection::LeftToRight, TextDirection::RightToLeft)
}

/// Pick the left-to-right (`ltr`) or mirrored (`rtl`) variant.
pub fn pick<T>(rtl: bool, ltr: T, rtl_val: T) -> T {
    if rtl { rtl_val } else { ltr }
}

/// A horizontal row starting at the reading edge: widgets run left to right, or right to
/// left when the interface is mirrored.
pub fn row(valign: egui::Align) -> egui::Layout {
    pick(rtl(), egui::Layout::left_to_right(valign), egui::Layout::right_to_left(valign))
}

/// A horizontal row packed against the trailing edge (a dialog's dismiss button, a toast's
/// close icon): the mirror image of [`row`].
pub fn row_end(valign: egui::Align) -> egui::Layout {
    pick(rtl(), egui::Layout::right_to_left(valign), egui::Layout::left_to_right(valign))
}

/// Show `add_contents` in a [`row`] layout, like `Ui::horizontal`: the row only takes
/// the height its widgets need, so the cross alignment centres within the row rather
/// than the whole remaining window.
pub fn show_row<R>(
    ui: &mut egui::Ui,
    valign: egui::Align,
    add_contents: impl FnOnce(&mut egui::Ui) -> R,
) -> egui::InnerResponse<R> {
    let size = egui::vec2(ui.available_size_before_wrap().x, ui.spacing().interact_size.y);
    ui.allocate_ui_with_layout(size, row(valign), add_contents)
}

/// Show `add_contents` in a [`row`] layout that wraps to a new line at the edge, like
/// `Ui::horizontal_wrapped`.
pub fn show_row_wrapped<R>(
    ui: &mut egui::Ui,
    add_contents: impl FnOnce(&mut egui::Ui) -> R,
) -> egui::InnerResponse<R> {
    let size = egui::vec2(ui.available_width(), ui.spacing().interact_size.y);
    ui.allocate_ui_with_layout(size, row(egui::Align::Center).with_main_wrap(true), add_contents)
}

/// A vertical stack whose lines hug the reading edge: left in a left-to-right interface,
/// right when mirrored.
pub fn column() -> egui::Layout {
    pick(rtl(), egui::Layout::top_down(egui::Align::Min), egui::Layout::top_down(egui::Align::Max))
}

/// Show `add_contents` in a [`column`] layout, like `Ui::vertical`: the stack hugs the
/// reading edge. Use this (not `Ui::vertical`) for stacks inside a [`show_row`): a plain
/// vertical hugs the physical left, so in a mirrored row its used rect starts at the
/// row's left edge and the row's remaining widgets are left with no room at all.
pub fn show_column<R>(
    ui: &mut egui::Ui,
    add_contents: impl FnOnce(&mut egui::Ui) -> R,
) -> egui::InnerResponse<R> {
    ui.scope_builder(egui::UiBuilder::new().layout(column()), add_contents)
}

/// Whether a side panel that sits on `left` in a left-to-right interface stays on the left.
fn panel_stays_left(left: bool, rtl: bool) -> bool {
    left != rtl
}

/// A side panel on its mirrored side: `left` in a left-to-right interface, the opposite
/// side when mirrored.
pub fn side_panel(id: &'static str, left: bool) -> egui::Panel {
    if panel_stays_left(left, rtl()) {
        egui::Panel::left(id)
    } else {
        egui::Panel::right(id)
    }
}

/// The chevron a collapsed tree row shows: it points into the hidden body, so right in a
/// left-to-right interface and left when mirrored.
pub fn chevron_collapsed() -> &'static str {
    pick(rtl(), "chevron-right", "chevron-left")
}

/// Whether the sidebar fold button points right: the left sidebar's folded state in a
/// left-to-right interface, and the right sidebar's open state when mirrored. The button
/// points into the window when folded and at the edge when open, on either side.
fn fold_points_right(folded: bool, rtl: bool) -> bool {
    folded != rtl
}

/// The sidebar's fold/unfold chevron: it points into the window when folded and at the
/// edge when open, on whichever side the sidebar sits.
pub fn sidebar_fold(folded: bool) -> &'static str {
    pick(fold_points_right(folded, rtl()), "chevron-left", "chevron-right")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pick_returns_the_matching_side() {
        assert_eq!(pick(false, "ltr", "rtl"), "ltr");
        assert_eq!(pick(true, "ltr", "rtl"), "rtl");
    }

    #[test]
    fn side_panels_swap_sides_when_mirrored() {
        assert!(panel_stays_left(true, false));
        assert!(!panel_stays_left(false, false));
        assert!(!panel_stays_left(true, true));
        assert!(panel_stays_left(false, true));
    }

    #[test]
    fn fold_chevron_points_into_the_window_when_folded() {
        // Sidebar on the left: folded points right (inward), open points left (at the edge).
        assert!(fold_points_right(true, false));
        assert!(!fold_points_right(false, false));
        // Sidebar on the right: folded points left (inward), open points right (at the edge).
        assert!(!fold_points_right(true, true));
        assert!(fold_points_right(false, true));
    }
}
