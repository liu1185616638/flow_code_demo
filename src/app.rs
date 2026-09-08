use gpui_kit::component::button::*;
use gpui_kit::component::*;
use gpui_kit::prelude::FluentBuilder;
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
const PURPLE: u32 = 0xa78bfa;
const CYAN: u32 = 0x67d5e8;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Page {
    Workspace,
    History,
    Graph,
    Workflows,
    Intelligence,
    AiReview,
}

impl Page {
    fn title(self) -> &'static str {
        match self {
            Self::Workspace => "工作区",
            Self::History => "提交记录",
            Self::Graph => "提交图谱",
            Self::Workflows => "工作流",
            Self::Intelligence => "代码理解",
            Self::AiReview => "AI Review",
        }
    }

    fn description(self) -> &'static str {
        match self {
            Self::Workspace => "Review Canvas · 变更、Diff 与提交",
            Self::History => "History Inspector · 提交、文件与历史差异",
            Self::Graph => "Commit Graph · 分支拓扑与提交关系",
            Self::Workflows => "Runbook Studio · 自动化工作流编排",
            Self::Intelligence => "Code Intelligence · 符号、引用与依赖",
            Self::AiReview => "AI Workspace · Review、解释与建议",
        }
    }
}

struct FlowCodeDemo {
    page: Page,
    command_open: bool,
}

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

    fn badge(text: &'static str, color: u32) -> impl IntoElement {
        div()
            .px(px(7.0))
            .py(px(2.0))
            .rounded(px(10.0))
            .bg(rgb(SURFACE_RAISED))
            .text_size(px(10.0))
            .text_color(rgb(color))
            .child(text)
    }

    fn panel_header(title: &'static str, trailing: &'static str) -> impl IntoElement {
        div()
            .h(px(42.0))
            .flex_none()
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
                    .child(title),
            )
            .child(
                div()
                    .text_size(px(10.0))
                    .text_color(rgb(SUBTLE))
                    .child(trailing),
            )
    }

    fn nav_button(
        &self,
        id: &'static str,
        label: &'static str,
        page: Page,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let active = self.page == page;
        Button::new(id)
            .label(label)
            .small()
            .selected(active)
            .on_click(cx.listener(move |this, _, _, cx| {
                this.page = page;
                this.command_open = false;
                cx.notify();
            }))
    }

    fn repo_row(label: &'static str, badge: Option<&'static str>, active: bool) -> impl IntoElement {
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
                            .child(if active { "◆" } else { "◇" }),
                    )
                    .child(label),
            )
            .when_some(badge, |this, badge| this.child(Self::badge(badge, SUBTLE)))
    }

    fn status_item(text: &'static str, accent: u32) -> impl IntoElement {
        div()
            .h_full()
            .px(px(10.0))
            .flex()
            .items_center()
            .border_r_1()
            .border_color(rgb(BORDER))
            .text_size(px(10.0))
            .text_color(rgb(accent))
            .child(text)
    }

    fn metric_card(
        title: &'static str,
        value: &'static str,
        detail: &'static str,
        accent: u32,
    ) -> impl IntoElement {
        div()
            .flex_1()
            .min_w(px(120.0))
            .p(px(12.0))
            .rounded(px(9.0))
            .border_1()
            .border_color(rgb(BORDER))
            .bg(rgb(SURFACE))
            .flex()
            .flex_col()
            .gap(px(5.0))
            .child(
                div()
                    .text_size(px(10.0))
                    .text_color(rgb(SUBTLE))
                    .child(title),
            )
            .child(
                div()
                    .text_size(px(19.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(accent))
                    .child(value),
            )
            .child(
                div()
                    .text_size(px(10.0))
                    .text_color(rgb(MUTED))
                    .child(detail),
            )
    }

    fn render_titlebar(&self, cx: &mut Context<Self>) -> impl IntoElement {
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
                    .child(div().h(px(22.0)).w(px(1.0)).bg(rgb(BORDER)))
                    .child(
                        div()
                            .px(px(9.0))
                            .py(px(5.0))
                            .rounded(px(6.0))
                            .bg(rgb(SURFACE_RAISED))
                            .text_size(px(11.0))
                            .text_color(rgb(MUTED))
                            .child("khaslana  /  master"),
                    )
                    .child(
                        Button::new("command")
                            .label("⌘K  Command")
                            .small()
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.command_open = !this.command_open;
                                cx.notify();
                            })),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(6.0))
                    .child(Button::new("fetch").label("Fetch"))
                    .child(Button::new("pull").label("Pull"))
                    .child(Button::new("push").primary().label("Push"))
                    .child(Button::new("settings").label("Settings")),
            )
    }

    fn render_sidebar(&self, cx: &mut Context<Self>) -> impl IntoElement {
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
            .child(
                div()
                    .px(px(7.0))
                    .flex()
                    .flex_col()
                    .gap(px(3.0))
                    .child(self.nav_button("nav-workspace", "工作区", Page::Workspace, cx))
                    .child(self.nav_button("nav-history", "提交记录", Page::History, cx))
                    .child(self.nav_button("nav-graph", "提交图谱", Page::Graph, cx))
                    .child(self.nav_button("nav-workflows", "工作流", Page::Workflows, cx)),
            )
            .child(Self::section_label("CODE"))
            .child(
                div()
                    .px(px(7.0))
                    .flex()
                    .flex_col()
                    .gap(px(3.0))
                    .child(self.nav_button("nav-intelligence", "代码理解", Page::Intelligence, cx))
                    .child(self.nav_button("nav-ai", "AI Review", Page::AiReview, cx)),
            )
            .child(Self::section_label("REPOSITORY"))
            .child(Self::repo_row("master", Some("HEAD"), true))
            .child(Self::repo_row("dev_lcc", Some("+2"), false))
            .child(Self::repo_row("feature/ui-kit", None, false))
            .child(Self::repo_row("Remotes", Some("3"), false))
            .child(Self::repo_row("Tags", Some("12"), false))
            .child(Self::repo_row("Stashes", Some("2"), false))
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

    fn render_page_header(&self) -> impl IntoElement {
        div()
            .h(px(52.0))
            .flex_none()
            .px(px(16.0))
            .flex()
            .items_center()
            .justify_between()
            .border_b_1()
            .border_color(rgb(BORDER))
            .bg(rgb(SURFACE))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap(px(3.0))
                    .child(
                        div()
                            .text_size(px(15.0))
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(TEXT))
                            .child(self.page.title()),
                    )
                    .child(
                        div()
                            .text_size(px(10.0))
                            .text_color(rgb(SUBTLE))
                            .child(self.page.description()),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(6.0))
                    .child(Self::badge("master", PRIMARY))
                    .child(Self::badge("↑2 ↓0", GREEN)),
            )
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
                    .child(div().text_size(px(11.0)).text_color(rgb(if selected { TEXT } else { MUTED })).child(path))
                    .child(div().text_size(px(10.0)).text_color(rgb(SUBTLE)).child(detail)),
            )
    }

    fn code_line(number: &'static str, prefix: &'static str, text: &'static str, tone: &'static str) -> impl IntoElement {
        let (bg, fg) = match tone {
            "add" => (0x14251c, 0x9ee6b8),
            "remove" => (0x2a171c, 0xf0a0aa),
            "hunk" => (0x171e31, 0x93a9f6),
            _ => (BG, 0xb6bdc9),
        };
        div()
            .min_h(px(23.0))
            .flex()
            .items_center()
            .bg(rgb(bg))
            .text_size(px(11.0))
            .font_family("monospace")
            .child(div().w(px(42.0)).px(px(8.0)).text_align(TextAlign::Right).text_color(rgb(0x555e6e)).child(number))
            .child(div().w(px(20.0)).text_align(TextAlign::Center).text_color(rgb(fg)).child(prefix))
            .child(div().flex_1().min_w(px(0.0)).pr(px(12.0)).text_color(rgb(fg)).child(text))
    }

    fn render_workspace(&self) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .child(
                div()
                    .w(px(250.0))
                    .h_full()
                    .flex_none()
                    .flex()
                    .flex_col()
                    .border_r_1()
                    .border_color(rgb(BORDER))
                    .bg(rgb(SURFACE))
                    .child(Self::panel_header("CHANGED FILES", "4 files"))
                    .child(
                        div()
                            .p(px(7.0))
                            .flex()
                            .flex_col()
                            .gap(px(2.0))
                            .child(Self::changed_file("M", "src/ui/components.rs", "+42  -18", true))
                            .child(Self::changed_file("M", "src/chrome_view.rs", "+17  -9", false))
                            .child(Self::changed_file("A", "src/code_index_view.rs", "+128", false))
                            .child(Self::changed_file("D", "src/legacy_panel.rs", "-84", false)),
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
                            .child(div().text_size(px(10.0)).text_color(rgb(SUBTLE)).child("COMMIT MESSAGE"))
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
                            .child(Button::new("commit-demo").primary().label("Commit 4 files")),
                    ),
            )
            .child(
                div()
                    .flex_1()
                    .min_w(px(0.0))
                    .h_full()
                    .flex()
                    .flex_col()
                    .bg(rgb(BG))
                    .child(Self::panel_header("src/ui/components.rs", "MODIFIED   +42  -18"))
                    .child(
                        div()
                            .flex_1()
                            .pt(px(8.0))
                            .child(Self::code_line("118", " ", "pub(crate) fn page_header(title: &'static str) -> Div {", "normal"))
                            .child(Self::code_line("119", " ", "    div()", "normal"))
                            .child(Self::code_line("120", "-", "        .min_h(px(36.0))", "remove"))
                            .child(Self::code_line("120", "+", "        .min_h(px(40.0))", "add"))
                            .child(Self::code_line("121", "+", "        .px(px(theme::SPACE_4))", "add"))
                            .child(Self::code_line("122", " ", "        .border_b_1()", "normal"))
                            .child(Self::code_line("123", "@@", "@@ -124,8 +125,18 @@", "hunk"))
                            .child(Self::code_line("124", "+", "        .bg(rgb(theme::SURFACE_BASE))", "add"))
                            .child(Self::code_line("125", "+", "        .child(command_group())", "add"))
                            .child(Self::code_line("126", " ", "}", "normal")),
                    ),
            )
            .child(self.render_workspace_inspector())
    }

    fn render_workspace_inspector(&self) -> impl IntoElement {
        div()
            .w(px(260.0))
            .h_full()
            .flex_none()
            .flex()
            .flex_col()
            .border_l_1()
            .border_color(rgb(BORDER))
            .bg(rgb(SURFACE))
            .child(Self::panel_header("INSPECTOR", "FILE"))
            .child(
                div()
                    .p(px(13.0))
                    .flex()
                    .flex_col()
                    .gap(px(8.0))
                    .child(div().text_size(px(13.0)).font_weight(FontWeight::SEMIBOLD).text_color(rgb(TEXT)).child("components.rs"))
                    .child(div().text_size(px(10.0)).text_color(rgb(SUBTLE)).child("src/ui/components.rs"))
                    .child(Self::badge("Modified", ORANGE))
                    .child(div().h(px(1.0)).bg(rgb(BORDER)))
                    .child(div().text_size(px(10.0)).text_color(rgb(MUTED)).child("Rust · UTF-8 · 36 symbols"))
                    .child(div().text_size(px(10.0)).text_color(rgb(MUTED)).child("12 references · 4 modules")),
            )
            .child(
                div()
                    .mx(px(12.0))
                    .p(px(10.0))
                    .rounded(px(8.0))
                    .border_1()
                    .border_color(rgb(BORDER))
                    .bg(rgb(BG))
                    .flex()
                    .flex_col()
                    .gap(px(6.0))
                    .child(div().text_size(px(10.0)).font_weight(FontWeight::SEMIBOLD).text_color(rgb(PRIMARY)).child("AI REVIEW"))
                    .child(div().text_size(px(10.0)).line_height(px(16.0)).text_color(rgb(MUTED)).child("Token usage is consistent. The layout change keeps the existing design-system contract.")),
            )
            .child(div().flex_1())
            .child(div().p(px(10.0)).border_t_1().border_color(rgb(BORDER)).child(Button::new("ask-ai-demo").label("Ask AI about this file")))
    }

    fn commit_row(hash: &'static str, title: &'static str, meta: &'static str, active: bool) -> impl IntoElement {
        div()
            .mx(px(7.0))
            .p(px(9.0))
            .rounded(px(7.0))
            .bg(rgb(if active { PRIMARY_SOFT } else { SURFACE }))
            .flex()
            .flex_col()
            .gap(px(4.0))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .child(div().text_size(px(11.0)).text_color(rgb(if active { TEXT } else { MUTED })).child(title))
                    .child(div().text_size(px(9.0)).font_family("monospace").text_color(rgb(PRIMARY)).child(hash)),
            )
            .child(div().text_size(px(10.0)).text_color(rgb(SUBTLE)).child(meta))
    }

    fn render_history(&self) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .child(
                div()
                    .w(px(340.0))
                    .h_full()
                    .flex_none()
                    .flex()
                    .flex_col()
                    .border_r_1()
                    .border_color(rgb(BORDER))
                    .bg(rgb(SURFACE))
                    .child(Self::panel_header("COMMITS", "master · 248"))
                    .child(
                        div()
                            .p(px(7.0))
                            .flex()
                            .flex_col()
                            .gap(px(3.0))
                            .child(Self::commit_row("b4ec891", "Merge branch 'master' into dev_lcc", "lcc · 8 min ago · HEAD", true))
                            .child(Self::commit_row("e297342", "重构代码索引设置页并接入 MCP 多仓库管理", "lcc · 12 min ago", false))
                            .child(Self::commit_row("b5cb2cb", "feat: 支持 MCP 多仓库查询与按需索引", "lcc · 2 hours ago", false))
                            .child(Self::commit_row("830a2a4", "refactor: simplify workspace chrome", "lcc · yesterday", false))
                            .child(Self::commit_row("3aa9b21", "feat: add AI review workspace", "lcc · 2 days ago", false)),
                    ),
            )
            .child(
                div()
                    .flex_1()
                    .min_w(px(0.0))
                    .h_full()
                    .flex()
                    .flex_col()
                    .bg(rgb(BG))
                    .child(Self::panel_header("COMMIT INSPECTOR", "b4ec891"))
                    .child(
                        div()
                            .p(px(16.0))
                            .flex()
                            .flex_col()
                            .gap(px(9.0))
                            .child(div().text_size(px(16.0)).font_weight(FontWeight::SEMIBOLD).text_color(rgb(TEXT)).child("Merge branch 'master' into dev_lcc"))
                            .child(div().text_size(px(10.0)).text_color(rgb(SUBTLE)).child("lcc · master · 8 minutes ago"))
                            .child(
                                div()
                                    .flex()
                                    .gap(px(8.0))
                                    .child(Self::badge("12 files", MUTED))
                                    .child(Self::badge("+420", GREEN))
                                    .child(Self::badge("-83", RED)),
                            )
                            .child(div().h(px(1.0)).bg(rgb(BORDER)))
                            .child(div().text_size(px(11.0)).text_color(rgb(MUTED)).child("Changed files"))
                            .child(Self::changed_file("M", "src/code_index_view.rs", "+128  -21", true))
                            .child(Self::changed_file("M", "src/ui/components.rs", "+42  -18", false))
                            .child(Self::changed_file("A", "docs/code-intelligence.md", "+91", false)),
                    )
                    .child(div().flex_1())
                    .child(
                        div()
                            .h(px(150.0))
                            .border_t_1()
                            .border_color(rgb(BORDER))
                            .bg(rgb(SURFACE))
                            .child(Self::panel_header("HISTORY DIFF", "Preview"))
                            .child(Self::code_line("64", "+", "pub fn ensure_repo_index(repo: &Path) -> Result<()> {", "add"))
                            .child(Self::code_line("65", "+", "    indexer.ensure(repo)?;", "add"))
                            .child(Self::code_line("66", " ", "}", "normal")),
                    ),
            )
    }

    fn graph_row(label: &'static str, refs: &'static str, color: u32, merge: bool) -> impl IntoElement {
        div()
            .h(px(48.0))
            .px(px(14.0))
            .flex()
            .items_center()
            .border_b_1()
            .border_color(rgb(0x202630))
            .child(
                div()
                    .w(px(90.0))
                    .flex()
                    .items_center()
                    .gap(px(8.0))
                    .child(div().size(px(if merge { 11.0 } else { 9.0 })).rounded(px(8.0)).bg(rgb(color)))
                    .child(div().h(px(1.0)).w(px(48.0)).bg(rgb(color))),
            )
            .child(
                div()
                    .flex_1()
                    .min_w(px(0.0))
                    .flex()
                    .flex_col()
                    .gap(px(3.0))
                    .child(div().text_size(px(11.0)).text_color(rgb(TEXT)).child(label))
                    .child(div().text_size(px(10.0)).text_color(rgb(SUBTLE)).child(refs)),
            )
    }

    fn render_graph(&self) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .flex_col()
            .bg(rgb(BG))
            .child(
                div()
                    .h(px(42.0))
                    .px(px(12.0))
                    .flex()
                    .items_center()
                    .justify_between()
                    .border_b_1()
                    .border_color(rgb(BORDER))
                    .bg(rgb(SURFACE))
                    .child(div().flex().gap(px(6.0)).child(Self::badge("Branch: master", PRIMARY)).child(Self::badge("All authors", MUTED)).child(Self::badge("Merge commits", PURPLE)))
                    .child(div().text_size(px(10.0)).text_color(rgb(SUBTLE)).child("248 commits · topology mode")),
            )
            .child(
                div()
                    .flex_1()
                    .min_h(px(0.0))
                    .child(Self::graph_row("Merge branch 'master' into dev_lcc", "b4ec891 · HEAD · master", PRIMARY, true))
                    .child(Self::graph_row("重构代码索引设置页并接入 MCP 多仓库管理", "e297342 · dev_lcc", GREEN, false))
                    .child(Self::graph_row("feat: 支持 MCP 多仓库查询与按需索引", "b5cb2cb", CYAN, false))
                    .child(Self::graph_row("refactor: simplify workspace chrome", "830a2a4", PRIMARY, true))
                    .child(Self::graph_row("feat: add AI review workspace", "3aa9b21 · feature/ai", PURPLE, false))
                    .child(Self::graph_row("fix: preserve navigator layout preference", "12c09ef", ORANGE, false)),
            )
            .child(
                div()
                    .h(px(118.0))
                    .flex_none()
                    .border_t_1()
                    .border_color(rgb(BORDER))
                    .bg(rgb(SURFACE))
                    .p(px(14.0))
                    .flex()
                    .justify_between()
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap(px(5.0))
                            .child(div().text_size(px(13.0)).font_weight(FontWeight::SEMIBOLD).text_color(rgb(TEXT)).child("Selected commit · b4ec891"))
                            .child(div().text_size(px(10.0)).text_color(rgb(MUTED)).child("Merge branch 'master' into dev_lcc"))
                            .child(div().flex().gap(px(6.0)).child(Self::badge("12 files", MUTED)).child(Self::badge("+420", GREEN)).child(Self::badge("-83", RED))),
                    )
                    .child(div().flex().gap(px(6.0)).child(Button::new("graph-history").label("在提交记录中查看")).child(Button::new("graph-checkout").label("Checkout"))),
            )
    }

    fn workflow_step(index: &'static str, title: &'static str, detail: &'static str, state: &'static str, color: u32) -> impl IntoElement {
        div()
            .p(px(11.0))
            .rounded(px(8.0))
            .border_1()
            .border_color(rgb(BORDER))
            .bg(rgb(SURFACE))
            .flex()
            .items_center()
            .gap(px(10.0))
            .child(div().size(px(24.0)).rounded(px(12.0)).flex().items_center().justify_center().bg(rgb(SURFACE_RAISED)).text_size(px(10.0)).text_color(rgb(color)).child(index))
            .child(div().flex_1().flex().flex_col().gap(px(3.0)).child(div().text_size(px(11.0)).text_color(rgb(TEXT)).child(title)).child(div().text_size(px(10.0)).text_color(rgb(SUBTLE)).child(detail)))
            .child(Self::badge(state, color))
    }

    fn render_workflows(&self) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .child(
                div()
                    .w(px(260.0))
                    .h_full()
                    .flex_none()
                    .border_r_1()
                    .border_color(rgb(BORDER))
                    .bg(rgb(SURFACE))
                    .child(Self::panel_header("RUNBOOKS", "6 templates"))
                    .child(div().p(px(8.0)).flex().flex_col().gap(px(5.0)).child(Self::commit_row("AUTO", "Feature → Dev", "Merge · Push · Browser QA", true)).child(Self::commit_row("REL", "Release", "Tag · Push · CNB publish", false)).child(Self::commit_row("SYNC", "Sync upstream", "Fetch · Rebase · Push", false)).child(Self::commit_row("QA", "UI smoke test", "Skills · Chrome MCP", false))),
            )
            .child(
                div()
                    .flex_1()
                    .min_w(px(0.0))
                    .h_full()
                    .flex()
                    .flex_col()
                    .bg(rgb(BG))
                    .child(Self::panel_header("FEATURE → DEV", "Ready"))
                    .child(
                        div()
                            .p(px(16.0))
                            .flex()
                            .flex_col()
                            .gap(px(9.0))
                            .child(Self::workflow_step("1", "Preflight", "检查工作区与目标分支", "DONE", GREEN))
                            .child(Self::workflow_step("2", "Commit", "生成并提交当前变更", "DONE", GREEN))
                            .child(Self::workflow_step("3", "Merge to dev_lcc", "合并当前 feature 分支", "RUNNING", PRIMARY))
                            .child(Self::workflow_step("4", "Push", "推送 dev_lcc 到 origin", "WAITING", SUBTLE))
                            .child(Self::workflow_step("5", "Browser QA", "加载 Skill + Chrome DevTools MCP", "WAITING", SUBTLE)),
                    )
                    .child(div().flex_1())
                    .child(
                        div()
                            .h(px(126.0))
                            .border_t_1()
                            .border_color(rgb(BORDER))
                            .bg(rgb(0x0c0e12))
                            .p(px(12.0))
                            .font_family("monospace")
                            .text_size(px(10.0))
                            .text_color(rgb(MUTED))
                            .child("14:31:08  ✓ preflight complete\n14:31:09  ✓ commit 4 files\n14:31:10  → merging feature/ui-kit into dev_lcc\n14:31:11    no conflicts detected"),
                    ),
            )
    }

    fn symbol_row(kind: &'static str, name: &'static str, path: &'static str, refs: &'static str, active: bool) -> impl IntoElement {
        div()
            .mx(px(7.0))
            .p(px(9.0))
            .rounded(px(7.0))
            .bg(rgb(if active { PRIMARY_SOFT } else { SURFACE }))
            .flex()
            .items_center()
            .gap(px(9.0))
            .child(Self::badge(kind, if active { PRIMARY } else { SUBTLE }))
            .child(div().flex_1().min_w(px(0.0)).flex().flex_col().gap(px(3.0)).child(div().text_size(px(11.0)).text_color(rgb(TEXT)).child(name)).child(div().text_size(px(9.0)).text_color(rgb(SUBTLE)).child(path)))
            .child(div().text_size(px(10.0)).text_color(rgb(MUTED)).child(refs))
    }

    fn render_intelligence(&self) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .child(
                div()
                    .w(px(330.0))
                    .h_full()
                    .flex_none()
                    .flex()
                    .flex_col()
                    .border_r_1()
                    .border_color(rgb(BORDER))
                    .bg(rgb(SURFACE))
                    .child(Self::panel_header("SYMBOL EXPLORER", "12,486 symbols"))
                    .child(div().p(px(7.0)).flex().flex_col().gap(px(3.0)).child(Self::symbol_row("struct", "RepositoryView", "src/main.rs", "27 refs", true)).child(Self::symbol_row("fn", "render_workspace", "src/workspace_view.rs", "8 refs", false)).child(Self::symbol_row("fn", "render_history", "src/history_view.rs", "6 refs", false)).child(Self::symbol_row("mod", "code_index", "src/code_index/mod.rs", "14 refs", false)).child(Self::symbol_row("trait", "GitService", "src/git/service.rs", "31 refs", false))),
            )
            .child(
                div()
                    .flex_1()
                    .min_w(px(0.0))
                    .h_full()
                    .flex()
                    .flex_col()
                    .bg(rgb(BG))
                    .child(Self::panel_header("SYMBOL INSPECTOR", "RepositoryView"))
                    .child(
                        div()
                            .p(px(16.0))
                            .flex()
                            .gap(px(10.0))
                            .child(Self::metric_card("REFERENCES", "27", "across 9 modules", PRIMARY))
                            .child(Self::metric_card("CALLERS", "14", "direct call sites", GREEN))
                            .child(Self::metric_card("DEPENDENCIES", "8", "domain modules", CYAN))
                            .child(Self::metric_card("CHANGE RISK", "MED", "UI shell hotspot", ORANGE)),
                    )
                    .child(
                        div()
                            .mx(px(16.0))
                            .flex_1()
                            .min_h(px(0.0))
                            .rounded(px(9.0))
                            .border_1()
                            .border_color(rgb(BORDER))
                            .bg(rgb(SURFACE))
                            .flex()
                            .flex_col()
                            .child(Self::panel_header("DEPENDENCY MAP", "static preview"))
                            .child(
                                div()
                                    .flex_1()
                                    .p(px(18.0))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .gap(px(22.0))
                                    .child(Self::metric_card("RepositoryView", "CORE", "main application state", PRIMARY))
                                    .child(div().text_size(px(20.0)).text_color(rgb(SUBTLE)).child("→"))
                                    .child(Self::metric_card("UI Views", "12", "workspace · history · graph", CYAN))
                                    .child(div().text_size(px(20.0)).text_color(rgb(SUBTLE)).child("→"))
                                    .child(Self::metric_card("Git + Index", "9", "services and stores", GREEN)),
                            ),
                    )
                    .child(div().h(px(16.0))),
            )
    }

    fn render_ai_review(&self) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .child(
                div()
                    .flex_1()
                    .min_w(px(0.0))
                    .h_full()
                    .flex()
                    .flex_col()
                    .bg(rgb(BG))
                    .child(Self::panel_header("AI REVIEW", "4 files · 1,284 tokens"))
                    .child(
                        div()
                            .p(px(16.0))
                            .flex()
                            .flex_col()
                            .gap(px(10.0))
                            .child(
                                div()
                                    .p(px(12.0))
                                    .rounded(px(9.0))
                                    .border_1()
                                    .border_color(rgb(BORDER))
                                    .bg(rgb(SURFACE))
                                    .flex()
                                    .flex_col()
                                    .gap(px(7.0))
                                    .child(div().flex().justify_between().child(div().text_size(px(11.0)).font_weight(FontWeight::SEMIBOLD).text_color(rgb(TEXT)).child("Review summary")).child(Self::badge("PASS", GREEN)))
                                    .child(div().text_size(px(11.0)).line_height(px(18.0)).text_color(rgb(MUTED)).child("The change consistently uses the design tokens and keeps the workspace hierarchy clear. No blocking issue found.")),
                            )
                            .child(
                                div()
                                    .p(px(12.0))
                                    .rounded(px(9.0))
                                    .border_1()
                                    .border_color(rgb(BORDER))
                                    .bg(rgb(SURFACE))
                                    .flex()
                                    .flex_col()
                                    .gap(px(7.0))
                                    .child(div().flex().justify_between().child(div().text_size(px(11.0)).font_weight(FontWeight::SEMIBOLD).text_color(rgb(TEXT)).child("Suggestion · components.rs:120")).child(Self::badge("STYLE", PRIMARY)))
                                    .child(div().text_size(px(11.0)).line_height(px(18.0)).text_color(rgb(MUTED)).child("Consider extracting the repeated 40px page-header height into a semantic layout token before the next UI iteration.")),
                            )
                            .child(
                                div()
                                    .p(px(12.0))
                                    .rounded(px(9.0))
                                    .border_1()
                                    .border_color(rgb(BORDER))
                                    .bg(rgb(SURFACE))
                                    .flex()
                                    .flex_col()
                                    .gap(px(7.0))
                                    .child(div().flex().justify_between().child(div().text_size(px(11.0)).font_weight(FontWeight::SEMIBOLD).text_color(rgb(TEXT)).child("Architecture note")).child(Self::badge("INFO", CYAN)))
                                    .child(div().text_size(px(11.0)).line_height(px(18.0)).text_color(rgb(MUTED)).child("The new inspector layout is a good candidate for reuse across commit, symbol, workflow and conflict pages.")),
                            ),
                    )
                    .child(div().flex_1()),
            )
            .child(
                div()
                    .w(px(320.0))
                    .h_full()
                    .flex_none()
                    .border_l_1()
                    .border_color(rgb(BORDER))
                    .bg(rgb(SURFACE))
                    .child(Self::panel_header("AI CONTEXT", "connected"))
                    .child(
                        div()
                            .p(px(13.0))
                            .flex()
                            .flex_col()
                            .gap(px(10.0))
                            .child(Self::metric_card("MODEL", "GPT", "review profile · high", PRIMARY))
                            .child(Self::metric_card("CODE INDEX", "READY", "12,486 symbols", GREEN))
                            .child(Self::metric_card("DIFF", "4", "+187 / -111 lines", ORANGE)),
                    ),
            )
    }

    fn render_command_palette(&self) -> impl IntoElement {
        div()
            .absolute()
            .top(px(58.0))
            .left(px(320.0))
            .right(px(320.0))
            .max_w(px(620.0))
            .mx_auto()
            .rounded(px(10.0))
            .border_1()
            .border_color(rgb(0x3a4351))
            .bg(rgb(0x171b22))
            .shadow_lg()
            .overflow_hidden()
            .child(
                div()
                    .h(px(44.0))
                    .px(px(13.0))
                    .flex()
                    .items_center()
                    .border_b_1()
                    .border_color(rgb(BORDER))
                    .text_size(px(12.0))
                    .text_color(rgb(MUTED))
                    .child("⌕  Search commands, branches, commits, symbols..."),
            )
            .child(
                div()
                    .p(px(7.0))
                    .flex()
                    .flex_col()
                    .gap(px(3.0))
                    .child(Self::command_row("Git", "Push current branch", "⌘⇧P", true))
                    .child(Self::command_row("Navigate", "Open Commit Graph", "", false))
                    .child(Self::command_row("Code", "Find symbol RepositoryView", "", false))
                    .child(Self::command_row("AI", "Review current changes", "", false))
                    .child(Self::command_row("Workflow", "Run Feature → Dev", "", false)),
            )
    }

    fn command_row(group: &'static str, title: &'static str, shortcut: &'static str, active: bool) -> impl IntoElement {
        div()
            .h(px(38.0))
            .px(px(9.0))
            .rounded(px(7.0))
            .bg(rgb(if active { PRIMARY_SOFT } else { 0x171b22 }))
            .flex()
            .items_center()
            .justify_between()
            .child(div().flex().items_center().gap(px(10.0)).child(Self::badge(group, if active { PRIMARY } else { SUBTLE })).child(div().text_size(px(11.0)).text_color(rgb(if active { TEXT } else { MUTED })).child(title)))
            .child(div().text_size(px(10.0)).text_color(rgb(SUBTLE)).child(shortcut))
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
                    .child(Self::status_item("master  ↑2 ↓0", PRIMARY))
                    .child(Self::status_item("4 Changes", MUTED))
                    .child(Self::status_item("Index Ready", GREEN))
                    .child(Self::status_item("MCP Running", CYAN)),
            )
            .child(
                div()
                    .h_full()
                    .flex()
                    .items_center()
                    .child(Self::status_item("AI Connected", PRIMARY))
                    .child(div().h_full().px(px(10.0)).flex().items_center().text_size(px(10.0)).text_color(rgb(SUBTLE)).child("GPUI Kit 0.6 Showcase")),
            )
    }

    fn render_page(&self) -> impl IntoElement {
        match self.page {
            Page::Workspace => self.render_workspace().into_any_element(),
            Page::History => self.render_history().into_any_element(),
            Page::Graph => self.render_graph().into_any_element(),
            Page::Workflows => self.render_workflows().into_any_element(),
            Page::Intelligence => self.render_intelligence().into_any_element(),
            Page::AiReview => self.render_ai_review().into_any_element(),
        }
    }
}

impl Render for FlowCodeDemo {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .relative()
            .size_full()
            .flex()
            .flex_col()
            .overflow_hidden()
            .bg(rgb(BG))
            .text_color(rgb(TEXT))
            .child(self.render_titlebar(cx))
            .child(
                div()
                    .flex_1()
                    .min_h(px(0.0))
                    .flex()
                    .overflow_hidden()
                    .child(self.render_sidebar(cx))
                    .child(
                        div()
                            .flex_1()
                            .min_w(px(0.0))
                            .h_full()
                            .flex()
                            .flex_col()
                            .overflow_hidden()
                            .child(self.render_page_header())
                            .child(div().flex_1().min_h(px(0.0)).overflow_hidden().child(self.render_page())),
                    ),
            )
            .child(self.render_statusbar())
            .when(self.command_open, |this| this.child(self.render_command_palette()))
    }
}

fn main() {
    let app = gpui_kit::application().with_assets(gpui_kit::assets::Assets);

    app.run(move |cx| {
        gpui_kit::init(cx);

        cx.spawn(async move |cx| {
            cx.open_window(WindowOptions::default(), |window, cx| {
                let view = cx.new(|_| FlowCodeDemo {
                    page: Page::Workspace,
                    command_open: false,
                });
                cx.new(|cx| Root::new(view, window, cx).bg(rgb(BG)))
            })
            .expect("failed to open Flow Code demo window");
        })
        .detach();
    });
}
