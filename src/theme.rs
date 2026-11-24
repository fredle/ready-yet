//! The app's look: colours for light and dark, type, spacing, fonts and the
//! few painted pieces (cards, buttons, pills, icons, progress, the segmented
//! control). Everything visual that is not layout lives here, so the window
//! code in `app.rs` only says what goes where.
//!
//! The build stage owns this file: pick the app's own accent (the six
//! `accent*`/`on_accent` values in each palette), add the painted pieces the
//! app needs, and delete the ones it does not use. The contrast test at the
//! bottom must keep passing.

#![allow(dead_code)]

use std::path::PathBuf;
use std::sync::Arc;

use eframe::egui::{
    self, Color32, CornerRadius, FontFamily, FontId, Pos2, Rect, Sense, Shape, Stroke,
    StrokeKind, TextStyle, Theme, Vec2, pos2, vec2,
};

/// Corner radius of cards.
pub const CARD_RADIUS: u8 = 12;
/// Corner radius of buttons, inputs and the segmented control.
pub const CONTROL_RADIUS: u8 = 8;
/// Height of buttons and single-line controls.
pub const CONTROL_HEIGHT: f32 = 32.0;
/// Height of the main button.
pub const PRIMARY_HEIGHT: f32 = 44.0;
/// Text sizes.
pub const BODY_SIZE: f32 = 14.5;
pub const SMALL_SIZE: f32 = 12.5;
pub const TITLE_SIZE: f32 = 20.0;

/// Font family for headings and the main button. Always defined; it falls
/// back to the regular face when Segoe UI Semibold is missing.
pub fn semibold() -> FontFamily {
    FontFamily::Name("semibold".into())
}

pub fn body() -> FontId {
    FontId::new(BODY_SIZE, FontFamily::Proportional)
}

pub fn small() -> FontId {
    FontId::new(SMALL_SIZE, FontFamily::Proportional)
}

pub fn title() -> FontId {
    FontId::new(TITLE_SIZE, semibold())
}

#[derive(Clone, Copy, Debug)]
pub struct Palette {
    /// Window background.
    pub bg: Color32,
    /// Cards and inputs.
    pub surface: Color32,
    /// A row under the pointer.
    pub hover: Color32,
    pub border: Color32,
    pub text: Color32,
    pub muted: Color32,
    /// Fills: the main button, the selected segment, progress.
    pub accent: Color32,
    /// Accent-coloured text, dark enough to read on surfaces and on
    /// `accent_soft`.
    pub accent_text: Color32,
    /// Text on an accent fill.
    pub on_accent: Color32,
    /// Accent at low strength, for tiles, pills and highlights.
    pub accent_soft: Color32,
    /// Resting fill of secondary buttons.
    pub control: Color32,
    pub control_hover: Color32,
    /// Unchecked boxes, progress tracks and the segmented control's track.
    pub track: Color32,
    pub success: Color32,
    pub success_soft: Color32,
    pub warn: Color32,
    pub warn_soft: Color32,
    pub error: Color32,
    pub error_soft: Color32,
}

const fn hex(rgb: u32) -> Color32 {
    Color32::from_rgb((rgb >> 16) as u8, (rgb >> 8) as u8, rgb as u8)
}

const LIGHT: Palette = Palette {
    bg: hex(0xF5F6F8),
    surface: hex(0xFFFFFF),
    hover: hex(0xF3F6FB),
    border: hex(0xE2E5EB),
    text: hex(0x1A1C22),
    muted: hex(0x5F6471),
    accent: hex(0x2563EB),
    accent_text: hex(0x1D4FC4),
    on_accent: hex(0xFFFFFF),
    accent_soft: hex(0xE8EFFD),
    control: hex(0xFFFFFF),
    control_hover: hex(0xEFF1F5),
    track: hex(0xE9EBF0),
    success: hex(0x17794A),
    success_soft: hex(0xE5F4EC),
    warn: hex(0x8F5600),
    warn_soft: hex(0xFBF0DC),
    error: hex(0xC0302E),
    error_soft: hex(0xFCEBEA),
};

const DARK: Palette = Palette {
    bg: hex(0x121318),
    surface: hex(0x1B1D24),
    hover: hex(0x22252E),
    border: hex(0x2C2F3A),
    text: hex(0xEAECF1),
    muted: hex(0x989DAB),
    accent: hex(0x6E9BFF),
    accent_text: hex(0x9DBBFF),
    on_accent: hex(0x10131A),
    accent_soft: hex(0x1F2B47),
    control: hex(0x23262F),
    control_hover: hex(0x2C303B),
    track: hex(0x2E313C),
    success: hex(0x5CCB8F),
    success_soft: hex(0x1B3328),
    warn: hex(0xE5AE55),
    warn_soft: hex(0x3A2E19),
    error: hex(0xFF8A85),
    error_soft: hex(0x3E2226),
};

pub fn palette(ctx: &egui::Context) -> Palette {
    match ctx.theme() {
        Theme::Dark => DARK,
        Theme::Light => LIGHT,
    }
}

fn visuals(p: Palette, mut v: egui::Visuals) -> egui::Visuals {
    let radius = CornerRadius::same(CONTROL_RADIUS);
    v.panel_fill = p.bg;
    v.window_fill = p.surface;
    v.window_stroke = Stroke::new(1.0, p.border);
    v.window_corner_radius = CornerRadius::same(CARD_RADIUS);
    v.menu_corner_radius = CornerRadius::same(CONTROL_RADIUS);
    v.extreme_bg_color = p.surface;
    v.faint_bg_color = p.bg;
    v.hyperlink_color = p.accent_text;
    v.warn_fg_color = p.warn;
    v.error_fg_color = p.error;
    v.selection.bg_fill = p.accent;
    v.selection.stroke = Stroke::new(1.0, p.on_accent);
    v.slider_trailing_fill = true;
    v.handle_shape = egui::style::HandleShape::Circle;

    let w = &mut v.widgets;
    w.noninteractive.bg_fill = p.surface;
    w.noninteractive.weak_bg_fill = p.surface;
    w.noninteractive.bg_stroke = Stroke::new(1.0, p.border);
    w.noninteractive.fg_stroke = Stroke::new(1.0, p.text);
    w.noninteractive.corner_radius = radius;

    w.inactive.bg_fill = p.track;
    w.inactive.weak_bg_fill = p.control;
    w.inactive.bg_stroke = Stroke::new(1.0, p.border);
    w.inactive.fg_stroke = Stroke::new(1.0, p.text);
    w.inactive.corner_radius = radius;

    w.hovered.bg_fill = p.control_hover;
    w.hovered.weak_bg_fill = p.control_hover;
    w.hovered.bg_stroke = Stroke::new(1.0, p.accent);
    w.hovered.fg_stroke = Stroke::new(1.5, p.text);
    w.hovered.corner_radius = radius;
    w.hovered.expansion = 0.0;

    w.active.bg_fill = p.accent_soft;
    w.active.weak_bg_fill = p.accent_soft;
    w.active.bg_stroke = Stroke::new(1.0, p.accent);
    w.active.fg_stroke = Stroke::new(1.5, p.text);
    w.active.corner_radius = radius;
    w.active.expansion = 0.0;

    w.open.bg_fill = p.control_hover;
    w.open.weak_bg_fill = p.control_hover;
    w.open.bg_stroke = Stroke::new(1.0, p.accent);
    w.open.fg_stroke = Stroke::new(1.0, p.text);
    w.open.corner_radius = radius;
    v
}

/// Colours, type sizes and spacing. Call once at start.
pub fn apply(ctx: &egui::Context) {
    ctx.set_visuals_of(Theme::Light, visuals(LIGHT, egui::Visuals::light()));
    ctx.set_visuals_of(Theme::Dark, visuals(DARK, egui::Visuals::dark()));
    ctx.all_styles_mut(|style| {
        style.spacing.item_spacing = Vec2::new(8.0, 8.0);
        style.spacing.button_padding = Vec2::new(12.0, 6.0);
        style.spacing.interact_size.y = CONTROL_HEIGHT;
        style.spacing.slider_width = 150.0;
        style.spacing.icon_width = 18.0;
        style.spacing.combo_width = 110.0;
        for (text_style, size) in [
            (TextStyle::Body, BODY_SIZE),
            (TextStyle::Button, BODY_SIZE),
            (TextStyle::Small, 12.0),
            (TextStyle::Heading, TITLE_SIZE),
        ] {
            style
                .text_styles
                .insert(text_style, FontId::new(size, FontFamily::Proportional));
        }
    });
}

/// Uses Windows' Segoe UI for the interface, so the app looks at home, with
/// Windows' East Asian fonts behind it so file names in any script show.
/// egui's built-in fonts stay as the last fallback.
pub fn install_fonts(ctx: &egui::Context) {
    let fonts_dir = PathBuf::from(std::env::var("WINDIR").unwrap_or_else(|_| "C:\\Windows".into()))
        .join("Fonts");
    let mut fonts = egui::FontDefinitions::default();
    let add = |fonts: &mut egui::FontDefinitions, file: &str| -> bool {
        let Ok(bytes) = std::fs::read(fonts_dir.join(file)) else {
            return false;
        };
        fonts
            .font_data
            .insert(file.to_owned(), Arc::new(egui::FontData::from_owned(bytes)));
        true
    };

    let mut regular = fonts
        .families
        .get(&FontFamily::Proportional)
        .cloned()
        .unwrap_or_default();
    if add(&mut fonts, "segoeui.ttf") {
        regular.insert(0, "segoeui.ttf".to_owned());
    }
    let mut strong = regular.clone();
    if add(&mut fonts, "seguisb.ttf") {
        strong.insert(0, "seguisb.ttf".to_owned());
    }
    for file in ["YuGothR.ttc", "msyh.ttc", "malgun.ttf"] {
        if add(&mut fonts, file) {
            regular.push(file.to_owned());
            strong.push(file.to_owned());
        }
    }
    fonts.families.insert(FontFamily::Proportional, regular);
    fonts.families.insert(semibold(), strong);
    ctx.set_fonts(fonts);
}

/// A rounded panel that groups related things.
pub fn card(p: Palette) -> egui::Frame {
    egui::Frame::new()
        .fill(p.surface)
        .stroke(Stroke::new(1.0, p.border))
        .corner_radius(CARD_RADIUS)
        .inner_margin(egui::Margin::symmetric(16, 12))
}

/// A small heading above a group of controls.
pub fn section_label(ui: &mut egui::Ui, text: &str, p: Palette) {
    ui.label(
        egui::RichText::new(text)
            .family(semibold())
            .size(SMALL_SIZE)
            .color(p.muted),
    );
}

/// The one filled button on the screen: the thing to press next.
pub fn primary_button<'a>(p: Palette, text: impl Into<String>, enabled: bool, width: f32) -> egui::Button<'a> {
    let label = egui::RichText::new(text)
        .family(semibold())
        .size(15.5)
        .color(if enabled { p.on_accent } else { p.muted });
    egui::Button::new(label)
        .fill(if enabled { p.accent } else { p.track })
        .stroke(Stroke::NONE)
        .corner_radius(10)
        .min_size(Vec2::new(width, PRIMARY_HEIGHT))
}

/// A quiet bordered button for everything that is not the next step.
pub fn secondary_button<'a>(p: Palette, text: impl Into<String>) -> egui::Button<'a> {
    egui::Button::new(egui::RichText::new(text).color(p.text))
        .fill(p.control)
        .stroke(Stroke::new(1.0, p.border))
        .corner_radius(CONTROL_RADIUS)
        .min_size(Vec2::new(0.0, CONTROL_HEIGHT))
}

/// A one-line text cut to `width` with an ellipsis.
pub fn truncated(
    painter: &egui::Painter,
    text: &str,
    font: FontId,
    color: Color32,
    width: f32,
) -> Arc<egui::Galley> {
    let mut job = egui::text::LayoutJob::simple_singleline(text.to_owned(), font, color);
    job.wrap = egui::text::TextWrapping::truncate_at_width(width.max(1.0));
    painter.layout_job(job)
}

/// A small rounded label such as a status, laid out like a widget.
pub fn pill(ui: &mut egui::Ui, text: &str, fg: Color32, bg: Color32) -> egui::Response {
    let max = ui.available_width().max(40.0);
    let galley = truncated(ui.painter(), text, FontId::new(12.0, semibold()), fg, max - 20.0);
    let (rect, response) = ui.allocate_exact_size(vec2(galley.size().x + 20.0, 24.0), Sense::hover());
    ui.painter().rect_filled(rect, 12, bg);
    ui.painter().galley(rect.center() - galley.size() / 2.0, galley, fg);
    response
}

/// Icons drawn with lines, so they never depend on a font having a glyph.
/// Never put emoji or symbol characters in labels: draw the icon here.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Icon {
    Close,
    Play,
    Stop,
    Plus,
    Folder,
    Retry,
}

pub fn paint_icon(painter: &egui::Painter, centre: Pos2, icon: Icon, color: Color32) {
    let stroke = Stroke::new(1.6, color);
    let c = centre;
    match icon {
        Icon::Close => {
            let d = 4.5;
            painter.line_segment([c + vec2(-d, -d), c + vec2(d, d)], stroke);
            painter.line_segment([c + vec2(d, -d), c + vec2(-d, d)], stroke);
        }
        Icon::Play => {
            let points = vec![c + vec2(-3.5, -5.5), c + vec2(5.5, 0.0), c + vec2(-3.5, 5.5)];
            painter.add(Shape::convex_polygon(points, color, Stroke::NONE));
        }
        Icon::Stop => {
            painter.rect_filled(Rect::from_center_size(c, vec2(10.0, 10.0)), 2, color);
        }
        Icon::Plus => {
            let d = 5.0;
            painter.line_segment([c + vec2(-d, 0.0), c + vec2(d, 0.0)], stroke);
            painter.line_segment([c + vec2(0.0, -d), c + vec2(0.0, d)], stroke);
        }
        Icon::Folder => {
            let body = Rect::from_center_size(c + vec2(0.0, 1.0), vec2(14.0, 10.0));
            painter.rect_stroke(body, 2, stroke, StrokeKind::Middle);
            let tab = [
                body.left_top(),
                body.left_top() + vec2(0.0, -2.5),
                body.left_top() + vec2(5.0, -2.5),
                body.left_top() + vec2(6.5, 0.0),
            ];
            painter.add(Shape::line(tab.to_vec(), stroke));
        }
        Icon::Retry => {
            // Three quarters of a circle with an arrow head at its end.
            let r = 5.5;
            let points: Vec<Pos2> = (0..=18)
                .map(|i| {
                    let a = -0.4 + i as f32 / 18.0 * std::f32::consts::PI * 1.6;
                    c + vec2(a.cos() * r, a.sin() * r)
                })
                .collect();
            let end = c + vec2((-0.4f32).cos() * r, (-0.4f32).sin() * r);
            painter.add(Shape::line(points, stroke));
            painter.line_segment([end, end + vec2(-0.5, -4.0)], stroke);
            painter.line_segment([end, end + vec2(-4.0, 0.5)], stroke);
        }
    }
}

/// A round, borderless icon button with a soft hover disc.
pub fn icon_button(ui: &mut egui::Ui, icon: Icon, p: Palette) -> egui::Response {
    let (rect, response) = ui.allocate_exact_size(vec2(28.0, 28.0), Sense::click());
    let hot = ui
        .ctx()
        .animate_bool_with_time(response.id, response.hovered(), 0.12);
    if hot > 0.0 {
        ui.painter()
            .circle_filled(rect.center(), rect.width() / 2.0, p.track.gamma_multiply(hot));
    }
    let color = if response.hovered() { p.text } else { p.muted };
    paint_icon(ui.painter(), rect.center(), icon, color);
    response.on_hover_cursor(egui::CursorIcon::PointingHand)
}

/// A rounded progress track. `None` is an unknown amount: a short segment
/// sweeps along the track instead.
pub fn progress_track(painter: &egui::Painter, rect: Rect, fraction: Option<f32>, time: f64, p: Palette) {
    let radius = rect.height() / 2.0;
    painter.rect_filled(rect, radius, p.track);
    let fill = match fraction {
        Some(f) => Rect::from_min_size(rect.min, vec2(rect.width() * f.clamp(0.0, 1.0), rect.height())),
        None => {
            let span = rect.width() * 0.3;
            let t = (time * 0.8).fract() as f32;
            let left = rect.left() - span + (rect.width() + span) * t;
            Rect::from_x_y_ranges(left.max(rect.left())..=(left + span).min(rect.right()), rect.y_range())
        }
    };
    if fill.width() > 0.5 {
        painter.rect_filled(fill, radius, p.accent);
    }
}

/// Choices side by side on one track; the chosen one is filled with the
/// accent, which slides when the choice changes. Returns the newly chosen
/// index, if any.
pub fn segmented(ui: &mut egui::Ui, id: egui::Id, labels: &[&str], selected: usize, p: Palette) -> Option<usize> {
    let width = ui.available_width();
    let (rect, _) = ui.allocate_exact_size(vec2(width, CONTROL_HEIGHT), Sense::hover());
    let enabled = ui.is_enabled();
    ui.painter().rect_filled(rect, CONTROL_RADIUS, p.track);
    let n = labels.len().max(1) as f32;
    let inner = rect.shrink(3.0);
    let seg_w = inner.width() / n;
    let slide = ui.ctx().animate_value_with_time(id, selected as f32, 0.15);
    let knob = Rect::from_min_size(pos2(inner.left() + seg_w * slide, inner.top()), vec2(seg_w, inner.height()));
    let knob_fill = if enabled { p.accent } else { p.muted };
    ui.painter().rect_filled(knob, CONTROL_RADIUS - 2, knob_fill);
    let mut chosen = None;
    for (i, label) in labels.iter().enumerate() {
        let cell = Rect::from_min_size(pos2(inner.left() + seg_w * i as f32, inner.top()), vec2(seg_w, inner.height()));
        let response = ui.interact(cell, id.with(i), Sense::click());
        if response.clicked() && i != selected {
            chosen = Some(i);
        }
        let on = i == selected;
        if response.hovered() && !on {
            ui.painter().rect_filled(cell, CONTROL_RADIUS - 2, p.surface.gamma_multiply(0.7));
        }
        let fg = if on { p.on_accent } else { p.text };
        let galley = truncated(ui.painter(), label, FontId::new(13.5, semibold()), fg, cell.width() - 8.0);
        ui.painter().galley(cell.center() - galley.size() / 2.0, galley, fg);
        let _ = response.on_hover_cursor(egui::CursorIcon::PointingHand);
    }
    chosen
}

/// A checkbox with squarer corners than the buttons: at this size the
/// button radius would make it look like a radio button.
pub fn checkbox(ui: &mut egui::Ui, checked: &mut bool, text: impl Into<egui::WidgetText>) -> egui::Response {
    ui.scope(|ui| {
        let widgets = &mut ui.visuals_mut().widgets;
        for state in [&mut widgets.inactive, &mut widgets.hovered, &mut widgets.active] {
            state.corner_radius = CornerRadius::same(5);
        }
        ui.checkbox(checked, text)
    })
    .inner
}

/// WCAG 2 contrast ratio between two colours (1 to 21).
#[cfg(test)]
fn contrast(a: Color32, b: Color32) -> f32 {
    fn channel(c: u8) -> f32 {
        let c = c as f32 / 255.0;
        if c <= 0.03928 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    }
    fn luminance(c: Color32) -> f32 {
        0.2126 * channel(c.r()) + 0.7152 * channel(c.g()) + 0.0722 * channel(c.b())
    }
    let (la, lb) = (luminance(a), luminance(b));
    (la.max(lb) + 0.05) / (la.min(lb) + 0.05)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contrast_of_known_pairs() {
        assert!((contrast(Color32::BLACK, Color32::WHITE) - 21.0).abs() < 0.01);
        assert!((contrast(Color32::WHITE, Color32::WHITE) - 1.0).abs() < 0.01);
    }

    /// Every text colour reaches WCAG AA (4.5:1) on what it is drawn on.
    #[test]
    fn text_is_readable_in_both_palettes() {
        for (name, p) in [("light", LIGHT), ("dark", DARK)] {
            let pairs = [
                ("text/bg", p.text, p.bg),
                ("text/surface", p.text, p.surface),
                ("text/hover", p.text, p.hover),
                ("text/track", p.text, p.track),
                ("muted/bg", p.muted, p.bg),
                ("muted/surface", p.muted, p.surface),
                ("muted/hover", p.muted, p.hover),
                ("muted/track", p.muted, p.track),
                ("on_accent/accent", p.on_accent, p.accent),
                ("accent_text/surface", p.accent_text, p.surface),
                ("accent_text/accent_soft", p.accent_text, p.accent_soft),
                ("success/surface", p.success, p.surface),
                ("warn/surface", p.warn, p.surface),
                ("error/surface", p.error, p.surface),
                ("success/success_soft", p.success, p.success_soft),
                ("warn/warn_soft", p.warn, p.warn_soft),
                ("error/error_soft", p.error, p.error_soft),
                ("text/control", p.text, p.control),
            ];
            for (pair, fg, bg) in pairs {
                let ratio = contrast(fg, bg);
                assert!(ratio >= 4.5, "{name} {pair}: {ratio:.2}");
            }
        }
    }
}
