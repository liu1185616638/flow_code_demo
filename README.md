# Flow Code Demo

基于 [GPUI Kit](https://gpui-kit.com/) 的 Khaslana / CodeFlow 风格界面演示项目。

这个仓库只用于验证 UI 方向，不包含真实 Git、AI、代码索引、MCP 或工作流逻辑。

## Demo 展示内容

- 顶部应用标题栏与仓库/分支上下文
- 左侧 Workspace / Code / Repository 导航
- Changed Files 列表
- 中央 Diff / Code Review 画布
- 右侧 Inspector
- 底部 Status Bar
- GPUI Kit Button / Root / Theme 基础组件
- Calm Technical 风格的紧凑开发者工作台布局

## 运行

需要 Rust 2024 edition 环境。

```powershell
cargo run
```

首次编译 GPUI Kit 依赖较多，后续增量编译会明显更快。

## 定位

此项目是纯视觉 Demo，目标是用于对比 Khaslana 当前 `gpui-ce + yororen_ui + 自有组件` UI 与 GPUI Kit 方案的整体展示效果，不作为业务代码迁移分支。
