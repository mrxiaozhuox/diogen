---
title: Dioxus 桌面应用 杂谈
date: 2022-03-06 11:22:44
tags: [Dioxus, Desktop]
categories: Rust
---

> 做为一名 Dioxus 项目的贡献者，我也在不断的学习中。所以在本篇中来简单聊聊 Dioxus 对于桌面平台的开发设计。



## 桌面应用原理

**Dioxus-Desktop** 主要使用 **wry** 生成窗口，WRY 是一款跨平台的 Webview 渲染库，它支持所有主流平台（Windows Linux MacOS）。其本质上就是在一个原生的窗口中嵌入 Webview 渲染器，从而使得我们可以将一些 HTML 内容嵌入到桌面程序之中，而 Dioxus 则是运用了这一点。

在这里贴一下 wry 的仓库地址：[Tauri-Apps/wry](https://github.com/tauri-apps/wry)

在 Dioxus 中，它会根据你传递的配置生成相应的文件，并加入事件的循环监听：

```rust
dioxus::desktop::launch_cfg(app, |config| {
    config.with_window(|win| {
        win.with_title("mrxzx.info")
    })
});
```

在启动一个 Desktop 应用时，我们可以通过闭包函数对窗口进行相关配置，这部分配置取自于 `tao` 包，目前 Dioxus 还没有进行这方面的封装。关于 `WindowBuilder` 的开发文档：[WindowBuilder in tao::window - Rust (docs.rs)](https://docs.rs/tao/0.6.3/tao/window/struct.WindowBuilder.html)

```rust
event_loop.run(move |window_event, event_loop, control_flow| {
        *control_flow = ControlFlow::Wait;

        match window_event {
            Event::NewEvents(StartCause::Init) => {
				// 窗口创建时的一些初始化工作
            Event::WindowEvent {
                event, window_id, ..
            } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Destroyed { .. } => desktop.close_window(window_id, control_flow),
                WindowEvent::Resized(_) | WindowEvent::Moved(_) => {
                    if let Some(view) = desktop.webviews.get_mut(&window_id) {
                        let _ = view.resize();
                    }
                }
                _ => {}
            },
            Event::UserEvent(user_event) => {
                desktop_context::handler(user_event, &mut desktop, control_flow)
            }
            Event::MainEventsCleared => {}
            Event::Resumed => {}
            Event::Suspended => {}
            Event::LoopDestroyed => {}
            Event::RedrawRequested(_id) => {}
            _ => {}
        }
    })
```

当窗口被创建后，程序会进入一个 loop 循环，并一直等待事件的进入，并根据不同的事件进行不同的处理。



## 窗口交互

这一部分的原型主要是我提交的，因为在尝试制作无边框应用时遇到了 `Window Drag` 的问题，当隐藏窗口本身的头部栏后，无法对窗口进行托动，以及自定义关闭、最小化、最大化按钮。所以说一直在和 JK 讨论这方面的实现方案，最终决定制作一个 `use_window` 并封装一些函数在程序运行时可以被调用。（在此之前我们无法在运行时对窗口进行原生的交互，如 更新 Title 内容、隐藏窗口、关闭窗口等。）目前只需要在相应的 Element & Function 中使用 `use_window` 即可完成窗口的交互。

```rust
fn app(cx: Scope) -> Element {
    let win = use_window(&cx);
    cx.render(rsx! {
        div { "hello world" }
        button {
            onclick: move |_| {
                win.set_title("Dioxus YYDS!")
            },
            "更换窗口标题"
        }
    })
}
```

目前支持的函数：

```rust
/// trigger the drag-window event
pub fn drag(&self) {}

/// set window minimize state
pub fn set_minimized(&self, minimized: bool) {}

/// set window maximize state
pub fn set_maximized(&self, maximized: bool) {}

/// toggle window maximize state
pub fn toggle_maximized(&self) {}

/// set window visible or not
pub fn set_visible(&self, visible: bool) {}

/// close window
pub fn close(&self) {}

/// set window to focus
pub fn focus(&self) {}

/// change window to fullscreen
pub fn set_fullscreen(&self, fullscreen: bool) {}

/// set resizable state
pub fn set_resizable(&self, resizable: bool) {}

/// set the window always on top
pub fn set_always_on_top(&self, top: bool) {}

/// set cursor visible or not
pub fn set_cursor_visible(&self, visible: bool) {}

/// set cursor grab
pub fn set_cursor_grab(&self, grab: bool) {}

/// set window title
pub fn set_title(&self, title: &str) {}

/// change window to borderless
pub fn set_decorations(&self, decoration: bool) {}

/// opens DevTool window
pub fn devtool(&self) {}
```

这使得我们可以在 Dioxus 组件中与窗口进行交互。



### 实现方案

这里简单聊一聊这部分的实现方法；我们定义了一种 Custom 事件，并在 Desktop Context 被调用时，将对应的事件类型 Send 到 Tao 的事件循环之中，使得它触发相应的事件并进行响应。

```rust
Event::UserEvent(user_event) => {
    desktop_context::handler(user_event, &mut desktop, control_flow)
}
```

这里的 `UserEvent` 便是在 Desktop Context 时触发的，它交给 `desktop_contex::handler` 进行处理。



### 更多交互

虽然目前我们实现大部分常用的 Window 交互接口，但它并不完整，所以说目前我们还在想办法完成更多交互功能。



## 多窗口

目前的 **Dioxus-Desktop** 仅支持单个窗口，不过我们正在尝试设计多窗口，这也是下一个大版本的主要任务。
