//! Reading-direction helpers: one place that knows whether the interface runs left to
//! right or right to left, so layout sites ask instead of hard-coding a side.
//!
//! Deliberately small on purpose: every helper here uses only egui APIs that have been
//! stable for years, so this module compiles unchanged on stock egui 0.36 and on the
//! patched 0.37 fork the full right-to-left work (PR-1 and later) moves to. Paragraph text
//! direction (`LayoutJob::direction`) is a 0.37-fork API and lands separately in PR-2.
//!
//! Icons that must NOT mirror (kept identical in both directions): play, close, check,
//! alerts, plus, search, and the up/down chevrons. Only sideways pointers flip.

/// Whether the interface currently reads right to left (Arabic today).
pub fn rtl() -> bool {
    crate::i18n::is_rtl()
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

/// Show `add_contents` in a [`row`] layout.
pub fn show_row<R>(
    ui: &mut egui::Ui,
    valign: egui::Align,
    add_contents: impl FnOnce(&mut egui::Ui) -> R,
) -> egui::InnerResponse<R> {
    ui.with_layout(row(valign), add_contents)
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

/// Whether the sidebar fold button points into the window (rather than at the edge).
fn fold_points_inward(folded: bool, rtl: bool) -> bool {
    folded != rtl
}

/// The sidebar's fold/unfold chevron: it points into the window when folded and at the
/// edge when open, on whichever side the sidebar sits.
pub fn sidebar_fold(folded: bool) -> &'static str {
    pick(fold_points_inward(folded, rtl()), "chevron-left", "chevron-right")
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
        assert!(fold_points_inward(true, false));
        assert!(!fold_points_inward(false, false));
        // Sidebar on the right: folded points left (inward), open points right (at the edge).
        assert!(fold_points_inward(true, true));
        assert!(!fold_points_inward(false, true));
    }
}
