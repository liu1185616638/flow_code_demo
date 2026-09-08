use gpui_kit::component::button::*;
use gpui_kit::component::*;
use gpui_kit::*;

const BG: u32 = 0x0f1115;
const SURFACE: u32 = 0x15181e;
const SURFACE_RAISED: u32 = 0x1b1f27;
const SURFACE_SOFT: u32 = 0x20252f;
const BORDER: u32 = 0x2a303b;
const TEXT: u32 = 0xe6e9ef;
const MUTED: u32 = 0x8f98a8;
const SUBTLE: u32 = 0x646d7c;
const PRIMARY: u32 = 0x6c8cff;
const PRIMARY_SOFT: u32 = 0x202a49;
const GREEN: u32 = 0x5ccf8a;
const RED: u32 = 0xf16d7a;
const ORANGE: u32 = 0xe9a35f;

pub struct FlowCodeDemo;

impl FlowCodeDemo {
    fn section_label(text: &'static str) -> impl IntoElement {
        div()
            .px(px(14.0))
            .pt(px(14.0))
            .pb(px(7.0))
            .text_size(px(10.0))
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(rgb(SUBTLE))
            .child(text)
    }

    fn nav_item(label: &'static str, shortcut: &'static str, active: bool) -> impl IntoElement {
        div()
            .mx(px(7.0))
            .h(px(32.0))
            .px(px(9.0))
            .flex()
            .items_center()
            .justify_between()
            .rounded(px(7.0))
            .bg(rgb(if active { PRIMARY_SOFT } else { SURFACE }))
            .text_color(rgb(if active { TEXT } else { MUTED }))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .child(
                        div()
                            .size(px(7.0))
                            .rounded(px(4.0))
                            .bg(rgb(if active { PRIMARY } else { BORDER })),
                    )
                    .child(label),
            )
            .child(
                div()
                    .text_size(px(10.0))
                    .text_color(rgb(SUBTLE))
                    .child(shortcut),
            )
    }

    fn repo_item(label: &'static str, badge: Option<&'static str>, active: bool) -> impl IntoElement {
        div()
            .mx(px(7.0))
            .h(px(30.0))
            .px(px(9.0))
            .flex()
            .items_center()
            .justify_between()
            .rounded(px(6.0))
            .bg(rgb(if active { SURFACE_SOFT } else { SURFACE }))
            .text_color(rgb(if active { TEXT } else { MUTED }))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(7.0))
                    .child(
                        div()
                            .text_size(px(11.0))
                            .text_color(rgb(if active { PRIMARY } else { SUBTLE }))
                            .child("◆"),
                    )
                    .child(label),
            )
            .when_some(badge, |this, badge| {
                this.child(
                    div()
                        .px(px(6.0))
                        .py(px(1.0))
                        .rounded(px(10.0))
                        .bg(rgb(SURFACE_RAISED))
                        .text_size(px(10.0))
                        .text_color(rgb(SUBTLE))
                        .child(badge),
                )
            })
    }

    fn changed_file(
        status: &'static str,
        path: &'static str,
        detail: &'static str,
        selected: bool,
    ) -> impl IntoElement {
        div()
            .mx(px(6.0))
            .px(px(9.0))
            .py(px(8.0))
            .rounded(px(7.0))
            .bg(rgb(if selected { PRIMARY_SOFT } else { SURFACE }))
            .flex()
            .gap(px(9.0))
            .child(
                div()
                    .mt(px(2.0))
                    .w(px(18.0))
                    .text_size(px(11.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(match status {
                        "M" => ORANGE,
                        "A" => GREEN,
                        "D" => RED,
                        _ => MUTED,
                    }))
                    .child(status),
            )
            .child(
                div()
                    .flex_1()
                    .min_w(px(0.0))
                    .flex()
                    .flex_col()
                    .gap(px(2.0))
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(rgb(if selected { TEXT } else { MUTED }))
                            .child(path),
                    )
                    .child(
                        div()
                            .text_size(px(10.0))
                            .text_color(rgb(SUBTLE))
                            .child(detail),
                    ),
            )
    }

    fn code_line(
        number: &'static str,
        prefix: &'static str,
        text: &'static str,
        tone: Option<&'static str>,
    ) -> impl IntoElement {
        let (bg, fg) = match tone {
            Some("add") => (0x14251c, 0x9ee6b8),
            Some("remove") => (0x2a171c, 0xf0a0aa),
            Some("hunk") => (0x171e31, 0x93a9f6),
            _ => (BG, 0xb6bdc9),
        };

        div()
            .min_h(px(23.0))
            .flex()
            .items_center()
            .bg(rgb(bg))
            .text_size(px(11.0))
            .font_family("monospace")
            .child(
                div()
                    .w(px(42.0))
                    .px(px(8.0))
                    .text_align(TextAlign::Right)
                    .text_color(rgb(0x555e6e))
                    .child(number),
            )
            .child(
                div()
                    .w(px(20.0))
                    .text_align(TextAlign::Center)
                    .text_color(rgb(fg))
                    .child(prefix),
            )
            .child(
                div()
                    .flex_1()
                    .min_w(px(0.0))
                    .pr(px(12.0))
                    .text_color(rgb(fg))
                    .child(text),
            )
    }

    fn inspector_row(label: &'static str, value: &'static str) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .justify_between()
            .gap(px(12.0))
            .py(px(5.0))
            .child(
                div()
                    .text_size(px(11.0))
                    .text_color(rgb(SUBTLE))
                    .child(label),
            )
            .child(
                div()
                    .min_w(px(0.0))
                    .text_size(px(11.0))
                    .text_color(rgb(TEXT))
                    .child(value),
            )
    }

    fn status_item(text: &'static str, accent: bool) -> impl IntoElement {
        div()
            .h_full()
            .px(px(10.0))
            .flex()
            .items_center()
            .border_r_1()
            .border_color(rgb(BORDER))
            .text_size(px(10.0))
            .text_color(rgb(if accent { PRIMARY } else { MUTED }))
            .child(text)
    }

    fn render_titlebar(&self) -> impl IntoElement {
        div()
            .h(px(46.0))
            .flex_none()
            .flex()
            .items_center()
            .justify_between()
            .px(px(10.0))
            .border_b_1()
            .border_color(rgb(BORDER))
            .bg(rgb(SURFACE))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(10.0))
                    .child(
                        div()
                            .size(px(27.0))
                            .rounded(px(8.0))
                            .flex()
                            .items_center()
                            .justify_center()
                            .bg(rgb(PRIMARY))
                            .text_color(rgb(0xffffff))
                            .font_weight(FontWeight::BOLD)
                            .child("C"),
                    )
                    .child(
                        div()
                            .text_size(px(13.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(TEXT))
                            .child("CodeFlow"),
                    )
                    .child(
                        div()
                            .h(px(22.0))
                            .w(px(1.0))
                            .bg(rgb(BORDER)),
                    )
                    .child(
                        div()
                            .px(px(9.0))
                            .py(px(5.0))
                            .rounded(px(6.0))
                            .bg(rgb(SURFACE_RAISED))
                            .text_size(px(11.0))
                            .text_color(rgb(MUTED))
                            .child("khaslana  /  master"),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(6.0))
                    .child(Button::new("fetch").label("Fetch").on_click(|_, _, _| {}))
                    .child(Button::new("pull").label("Pull").on_click(|_, _, _| {}))
                    .child(
                        Button::new("push")
                            .primary()
                            .label("Push")
                            .on_click(|_, _, _| {}),
                    )
                    .child(Button::new("settings").label("Settings").on_click(|_, _, _| {})),
            )
    }

    fn render_sidebar(&self) -> impl IntoElement {
        div()
            .w(px(224.0))
            .h_full()
            .flex_none()
            .flex()
            .flex_col()
            .border_r_1()
            .border_color(rgb(BORDER))
            .bg(rgb(SURFACE))
            .child(Self::section_label("WORKSPACE"))
            .child(Self::nav_item("工作区", "⌘1", true))
            .child(Self::nav_item("提交记录", "⌘2", false))
            .child(Self::nav_item("提交图谱", "", false))
            .child(Self::nav_item("工作流", "⌘3", false))
            .child(Self::section_label("CODE"))
            .child(Self::nav_item("代码理解", "", false))
            .child(Self::nav_item("符号浏览", "", false))
            .child(Self::nav_item("AI Review", "", false))
            .child(Self::section_label("REPOSITORY"))
            .child(Self::repo_item("master", Some("HEAD"), true))
            .child(Self::repo_item("dev_lcc", Some("+2"), false))
            .child(Self::repo_item("feature/ui-kit", None, false))
            .child(Self::repo_item("Remotes", Some("3"), false))
            .child(Self::repo_item("Tags", Some("12"), false))
            .child(Self::repo_item("Stashes", Some("2"), false))
            .child(div().flex_1())
            .child(
                div()
                    .m(px(9.0))
                    .p(px(10.0))
                    .rounded(px(8.0))
                    .border_1()
                    .border_color(rgb(BORDER))
                    .bg(rgb(BG))
                    .flex()
                    .flex_col()
                    .gap(px(5.0))
                    .child(
                        div()
                            .text_size(px(10.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(GREEN))
                            .child("INDEX READY"),
                    )
                    .child(
                        div()
                            .text_size(px(10.0))
                            .text_color(rgb(SUBTLE))
                            .child("12,486 symbols · 1,932 files"),
                    ),
            )
    }

    fn render_changed_files(&self) -> impl IntoElement {
        div()
            .w(px(246.0))
            .h_full()
            .flex_none()
            .flex()
            .flex_col()
            .border_r_1()
            .border_color(rgb(BORDER))
            .bg(rgb(SURFACE))
            .child(
                div()
                    .h(px(42.0))
                    .px(px(12.0))
                    .flex()
                    .items_center()
                    .justify_between()
                    .border_b_1()
                    .border_color(rgb(BORDER))
                    .child(
                        div()
                            .text_size(px(11.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(TEXT))
                            .child("CHANGED FILES"),
                    )
                    .child(
                        div()
                            .px(px(6.0))
                            .py(px(2.0))
                            .rounded(px(10.0))
                            .bg(rgb(SURFACE_RAISED))
                            .text_size(px(10.0))
                            .text_color(rgb(MUTED))
                            .child("4"),
                    ),
            )
            .child(
                div()
                    .p(px(7.0))
                    .flex()
                    .flex_col()
                    .gap(px(2.0))
                    .child(Self::changed_file(
                        "M",
                        "src/ui/components.rs",
                        "+42  -18",
                        true,
                    ))
                    .child(Self::changed_file(
                        "M",
                        "src/chrome_view.rs",
                        "+17  -9",
                        false,
                    ))
                    .child(Self::changed_file(
                        "A",
                        "src/code_index_view.rs",
                        "+128",
                        false,
                    ))
                    .child(Self::changed_file(
                        "D",
                        "src/legacy_panel.rs",
                        "-84",
                        false,
                    )),
            )
            .child(div().flex_1())
            .child(
                div()
                    .p(px(10.0))
                    .border_t_1()
                    .border_color(rgb(BORDER))
                    .flex()
                    .flex_col()
                    .gap(px(7.0))
                    .child(
                        div()
                            .text_size(px(10.0))
                            .text_color(rgb(SUBTLE))
                            .child("COMMIT MESSAGE"),
                    )
                    .child(
                        div()
                            .min_h(px(62.0))
                            .p(px(9.0))
                            .rounded(px(7.0))
                            .border_1()
                            .border_color(rgb(BORDER))
                            .bg(rgb(BG))
                            .text_size(px(11.0))
                            .text_color(rgb(MUTED))
                            .child("feat: refine developer workspace UI"),
                    )
                    .child(
                        Button::new("commit")
                            .primary()
                            .label("Commit 4 files")
                            .on_click(|_, _, _| {}),
                    ),
            )
    }

    fn render_diff(&self) -> impl IntoElement {
        div()
            .flex_1()
            .min_w(px(0.0))
            .h_full()
            .flex()
            .flex_col()
            .bg(rgb(BG))
            .child(
                div()
                    .h(px(42.0))
                    .flex_none()
                    .px(px(13.0))
                    .flex()
                    .items_center()
                    .justify_between()
                    .border_b_1()
                    .border_color(rgb(BORDER))
                    .bg(rgb(SURFACE))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .text_size(px(11.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(TEXT))
                                    .child("src/ui/components.rs"),
                            )
                            .child(
                                div()
                                    .px(px(7.0))
                                    .py(px(2.0))
                                    .rounded(px(10.0))
                                    .bg(rgb(0x342a19))
                                    .text_size(px(10.0))
                                    .text_color(rgb(ORANGE))
                                    .child("MODIFIED"),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            .child(
                                div()
                                    .text_size(px(10.0))
                                    .text_color(rgb(GREEN))
                                    .child("+42"),
                            )
                            .child(
                                div()
                                    .text_size(px(10.0))
                                    .text_color(rgb(RED))
                                    .child("-18"),
                            )
                            .child(Button::new("split").label("Split").on_click(|_, _, _| {})),
                    ),
            )
            .child(
                div()
                    .flex_1()
                    .min_h(px(0.0))
                    .pt(px(8.0))
                    .child(Self::code_line(
                        "118",
                        " ",
                        "pub(crate) fn page_header(title: &'static str) -> Div {",
                        None,
                    ))
                    .child(Self::code_line(
                        "119",
                        " ",
                        "    div()",
                        None,
                    ))
                    .child(Self::code_line(
                        "120",
                        "-",
                        "        .min_h(px(36.0))",
                        Some("remove"),
                    ))
                    .child(Self::code_line(
                        "120",
                        "+",
                        "        .min_h(px(40.0))",
                        Some("add"),
                    ))
                    .child(Self::code_line(
                        "121",
                        "+",
                        "        .px(px(theme::SPACE_4))",
                        Some("add"),
                    ))
                    .child(Self::code_line(
                        "122",
                        " ",
                        "        .border_b_1()",
                        None,
                    ))
                    .child(Self::code_line(
                        "123",
                        " ",
                        "        .border_color(rgb(theme::BORDER_MUTED))",
                        None,
                    ))
                    .child(Self::code_line(
                        "124",
                        "@@",
                        "@@ -124,8 +125,18 @@",
                        Some("hunk"),
                    ))
                    .child(Self::code_line(
                        "125",
                        "+",
                        "        .bg(rgb(theme::SURFACE_BASE))",
                        Some("add"),
                    ))
                    .child(Self::code_line(
                        "126",
                        "+",
                        "        .child(command_group())",
                        Some("add"),
                    ))
                    .child(Self::code_line(
                        "127",
                        " ",
                        "}",
                        None,
                    )),
            )
    }

    fn render_inspector(&self) -> impl IntoElement {
        div()
            .w(px(258.0))
            .h_full()
            .flex_none()
            .flex()
            .flex_col()
            .border_l_1()
            .border_color(rgb(BORDER))
            .bg(rgb(SURFACE))
            .child(
                div()
                    .h(px(42.0))
                    .px(px(12.0))
                    .flex()
                    .items_center()
                    .justify_between()
                    .border_b_1()
                    .border_color(rgb(BORDER))
                    .child(
                        div()
                            .text_size(px(11.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(TEXT))
                            .child("INSPECTOR"),
                    )
                    .child(
                        div()
                            .text_size(px(10.0))
                            .text_color(rgb(SUBTLE))
                            .child("FILE"),
                    ),
            )
            .child(
                div()
                    .p(px(13.0))
                    .flex()
                    .flex_col()
                    .gap(px(5.0))
                    .child(
                        div()
                            .text_size(px(13.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(TEXT))
                            .child("components.rs"),
                    )
                    .child(
                        div()
                            .pb(px(8.0))
                            .text_size(px(10.0))
                            .text_color(rgb(SUBTLE))
                            .child("src/ui/components.rs"),
                    )
                    .child(Self::inspector_row("Status", "Modified"))
                    .child(Self::inspector_row("Encoding", "UTF-8"))
                    .child(Self::inspector_row("Language", "Rust"))
                    .child(Self::inspector_row("Changes", "+42 / -18"))
                    .child(Self::inspector_row("Symbols", "36")),
            )
            .child(
                div()
                    .mx(px(12.0))
                    .h(px(1.0))
                    .bg(rgb(BORDER)),
            )
            .child(
                div()
                    .p(px(13.0))
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .child(
                        div()
                            .text_size(px(10.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(SUBTLE))
                            .child("CODE INTELLIGENCE"),
                    )
                    .child(
                        div()
                            .p(px(10.0))
                            .rounded(px(8.0))
                            .border_1()
                            .border_color(rgb(BORDER))
                            .bg(rgb(BG))
                            .flex()
                            .flex_col()
                            .gap(px(5.0))
                            .child(
                                div()
                                    .text_size(px(11.0))
                                    .text_color(rgb(TEXT))
                                    .child("RepositoryView"),
                            )
                            .child(
                                div()
                                    .text_size(px(10.0))
                                    .text_color(rgb(SUBTLE))
                                    .child("12 references · 4 modules"),
                            ),
                    )
                    .child(
                        div()
                            .p(px(10.0))
                            .rounded(px(8.0))
                            .border_1()
                            .border_color(rgb(BORDER))
                            .bg(rgb(BG))
                            .flex()
                            .flex_col()
                            .gap(px(6.0))
                            .child(
                                div()
                                    .text_size(px(10.0))
                                    .font_weight(FontWeight::SEMIBOLD)
                                    .text_color(rgb(PRIMARY))
                                    .child("AI REVIEW"),
                            )
                            .child(
                                div()
                                    .text_size(px(10.0))
                                    .line_height(px(16.0))
                                    .text_color(rgb(MUTED))
                                    .child("UI token usage is consistent. No blocking issue detected in this demo diff."),
                            ),
                    ),
            )
            .child(div().flex_1())
            .child(
                div()
                    .p(px(10.0))
                    .border_t_1()
                    .border_color(rgb(BORDER))
                    .child(
                        Button::new("ask-ai")
                            .label("Ask AI about this file")
                            .on_click(|_, _, _| {}),
                    ),
            )
    }

    fn render_statusbar(&self) -> impl IntoElement {
        div()
            .h(px(27.0))
            .flex_none()
            .flex()
            .items_center()
            .justify_between()
            .border_t_1()
            .border_color(rgb(BORDER))
            .bg(rgb(SURFACE_RAISED))
            .child(
                div()
                    .h_full()
                    .flex()
                    .items_center()
                    .child(Self::status_item("master  ↑2 ↓0", true))
                    .child(Self::status_item("4 Changes", false))
                    .child(Self::status_item("Index Ready", false))
                    .child(Self::status_item("MCP Running", false)),
            )
            .child(
                div()
                    .h_full()
                    .flex()
                    .items_center()
                    .child(Self::status_item("AI Connected", true))
                    .child(
                        div()
                            .h_full()
                            .px(px(10.0))
                            .flex()
                            .items_center()
                            .text_size(px(10.0))
                            .text_color(rgb(SUBTLE))
                            .child("GPUI Kit 0.6 Demo"),
                    ),
            )
    }
}

impl Render for FlowCodeDemo {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .overflow_hidden()
            .bg(rgb(BG))
            .text_color(rgb(TEXT))
            .child(self.render_titlebar())
            .child(
                div()
                    .flex_1()
                    .min_h(px(0.0))
                    .flex()
                    .overflow_hidden()
                    .child(self.render_sidebar())
                    .child(self.render_changed_files())
                    .child(self.render_diff())
                    .child(self.render_inspector()),
            )
            .child(self.render_statusbar())
    }
}

fn main() {
    let app = gpui_kit::application().with_assets(gpui_kit::assets::Assets);

    app.run(move |cx| {
        gpui_kit::init(cx);

        cx.spawn(async move |cx| {
            cx.open_window(WindowOptions::default(), |window, cx| {
                let view = cx.new(|_| FlowCodeDemo);
                cx.new(|cx| Root::new(view, window, cx).bg(rgb(BG)))
            })
            .expect("failed to open Flow Code demo window");
        })
        .detach();
    });
}
