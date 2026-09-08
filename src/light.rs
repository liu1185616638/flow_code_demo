use gpui_kit::*;
use gpui_kit::component::Root;
use gpui_kit::component::theme::Theme;

fn mapped_hex(hex: u32) -> u32 {
    match hex {
        // Khaslana / CodeFlow semantic surfaces
        0x0f1115 => 0xf4f7fb,
        0x15181e => 0xffffff,
        0x1b1f27 => 0xf7f9fc,
        0x20252f => 0xeaf0f7,
        0x2a303b => 0xd9e1ea,
        0xe6e9ef => 0x1f2937,
        0x8f98a8 => 0x596579,
        0x646d7c => 0x8793a3,
        0x6c8cff => 0x4f6ff7,
        0x202a49 => 0xe8eeff,
        0x5ccf8a => 0x168f62,
        0xf16d7a => 0xd94b5b,
        0xe9a35f => 0xd9822b,
        0xa78bfa => 0x7557d9,
        0x67d5e8 => 0x168db5,

        // Diff surfaces
        0x14251c => 0xeaf8f0,
        0x9ee6b8 => 0x18794e,
        0x2a171c => 0xfdecef,
        0xf0a0aa => 0xb93649,
        0x171e31 => 0xecf2ff,
        0x93a9f6 => 0x4568c9,
        0xb6bdc9 => 0x374151,
        0x555e6e => 0x98a3b2,

        // Floating surfaces / command palette / status badges
        0x3a4351 => 0xc7d1dd,
        0x171b22 => 0xffffff,
        0x342a19 => 0xfff3df,

        // Keep pure white as white where the app intentionally asks for it.
        0xffffff => 0xffffff,
        other => {
            // The original showcase occasionally uses additional very-dark
            // neutral hex values inline. Lift those automatically while
            // preserving colored semantic accents.
            let r = ((other >> 16) & 0xff) as i32;
            let g = ((other >> 8) & 0xff) as i32;
            let b = (other & 0xff) as i32;
            let max = r.max(g).max(b);
            let min = r.min(g).min(b);
            if max < 78 && max - min < 22 {
                if max < 32 { 0xf8fafc } else { 0xf1f5f9 }
            } else {
                other
            }
        }
    }
}

fn configure_component_theme(cx: &mut App) {
    let theme = Theme::global_mut(cx);

    // General application palette. These values also drive GPUI Kit controls,
    // which makes buttons feel native to the light workspace instead of sitting
    // on top as stark white blocks.
    theme.colors.background = gpui_kit::rgb(0xf4f7fb);
    theme.colors.foreground = gpui_kit::rgb(0x1f2937);
    theme.colors.border = gpui_kit::rgb(0xd9e1ea);
    theme.colors.input = gpui_kit::rgb(0xcbd5e1);
    theme.colors.muted = gpui_kit::rgb(0xf1f5f9);
    theme.colors.muted_foreground = gpui_kit::rgb(0x718096);
    theme.colors.accent = gpui_kit::rgb(0xe8eeff);
    theme.colors.accent_foreground = gpui_kit::rgb(0x3156d8);

    // Primary action: restrained indigo instead of a saturated solid block.
    theme.colors.primary = gpui_kit::rgb(0x4f6ff7);
    theme.colors.primary_hover = gpui_kit::rgb(0x4262e6);
    theme.colors.primary_active = gpui_kit::rgb(0x3554d6);
    theme.colors.primary_foreground = gpui_kit::rgb(0xffffff);
    theme.colors.button_primary = gpui_kit::rgb(0x4f6ff7);
    theme.colors.button_primary_hover = gpui_kit::rgb(0x4262e6);
    theme.colors.button_primary_active = gpui_kit::rgb(0x3554d6);
    theme.colors.button_primary_foreground = gpui_kit::rgb(0xffffff);

    // Default toolbar / navigation buttons: very light cool gray surfaces,
    // subtle border, dark text. This fixes the current bright-white button wall.
    theme.colors.button = gpui_kit::rgb(0xf8fafc);
    theme.colors.button_hover = gpui_kit::rgb(0eef3f8);
    theme.colors.button_active = gpui_kit::rgb(0xe2e9f1);
    theme.colors.button_foreground = gpui_kit::rgb(0x334155);
    theme.colors.button_secondary = gpui_kit::rgb(0xf8fafc);
    theme.colors.button_secondary_hover = gpui_kit::rgb(0xeef3f8);
    theme.colors.button_secondary_active = gpui_kit::rgb(0xe2e9f1);
    theme.colors.button_secondary_foreground = gpui_kit::rgb(0x334155);

    // Keep the visual language calm and desktop-tool-like.
    theme.radius = px(7.0);
    theme.radius_lg = px(10.0);
    theme.shadow = false;
    theme.focus_ring = false;

    Theme::sync_base(cx);
}

mod showcase {
    // An explicit local rgb wins over the glob import inside app.rs. This lets
    // us reuse the entire interactive demo and translate only its visual token
    // layer instead of duplicating the application logic.
    fn rgb(hex: u32) -> gpui_kit::Hsla {
        gpui_kit::rgb(super::mapped_hex(hex))
    }

    include!("app.rs");

    pub fn open(cx: &mut gpui_kit::App) {
        cx.spawn(async move |cx| {
            cx.open_window(gpui_kit::WindowOptions::default(), |window, cx| {
                let view = cx.new(|_| FlowCodeDemo {
                    page: Page::Workspace,
                    command_open: false,
                });
                cx.new(|cx| Root::new(view, window, cx).bg(rgb(BG)))
            })
            .expect("failed to open Flow Code light showcase window");
        })
        .detach();
    }
}

fn main() {
    let app = gpui_kit::application().with_assets(gpui_kit::assets::Assets);

    app.run(move |cx| {
        gpui_kit::init(cx);
        configure_component_theme(cx);
        showcase::open(cx);
    });
}
