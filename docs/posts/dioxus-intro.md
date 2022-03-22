---
title: Dioxus 项目介绍
date: 2022-01-16 18:10:18
tags: [Dioxus]
categories: Rust
---

很久没写博客了，最近在参加一个比较有意思的项目开发：[`Dioxus`](https://dioxuslabs.com/)

`Dioxus` 是一款类似于 React 的 UI 用户界面构建工具（生态系统）。

使用 Dioxus 可以快速构建高性能、跨平台的应用程序。

## 开发方式

Dioxus 的风格几乎等于 React ，所以如果你会使用 React 框架，那么 Dioxus 对你来说将会非常简单。它在设计模式是都参考（~~抄~~）了 React 框架。同时官方文档也一直在强调着：如果文档在这方面阐述的不清楚，可以去 React 文档看看 qwq

我们先来看一段演示代码吧：

```rust
fn app(cx: Scope) -> Element {

    let mut count = use_state(&cx, || 0);

    cx.render(rsx!(
        h1 { "Counter: {count}" }
        button { onclick: move |_| count += 1, "Up high!" }
        button { onclick: move |_| count -= 1, "Down low!" }
    ))
}
```

这个代码实现的功能很简单：

- 页面上有一个 `h1` 标签，两个加减数值的 `button` ，且都绑定了点击事件。
- 创建了一个可变的 `state` 变量，并且被绑定在了 `h1` 之中。
- 当按钮被点击，`state` 值会改变，注意：`h1` 也会动态的被改变！

也就是说：点击 增加/减少 按钮会同步更新 `h1` 中的数值！

当然，你也可以很轻易的实现双向绑定：

```rust
fn app(cx: Scope) -> Element {

    let mut text = use_state(&cx, || String::new());

    cx.render(rsx!(
        h1 { "Text: {text}" }
        input {
            value: "{text}",
            oninput: |evt| text.set(evt.value.clone())
        }
    ))
}
```

当输入框被输入时，将输入内容更新到 `text` 中，同时更新 `h1` 从而达到 `双向绑定` 。

## 跨平台性

我很喜欢尝试开发一些桌面程序，但我又喜欢跨平台的技术（这就是为什么我对 C#、Object-C 啥的提不起兴趣），所以说一个支持跨平台的开发框架对我来说真的超酷好吗！而 Dioxus 的核心代码就可以在任何平台下被使用。

- 网页端使用了 `WebAssembly` 技术（简单理解就是将 Rust 这种语言编译到浏览器运行 ）
- 桌面端则是使用了 Tauri 系统下的 `Tao` 包，它支持各平台的桌面端程序开发
- 移动端似乎也是 `Tao` 包，但是现在支持的不好，感觉没啥用qwq
- TUI 使用了 Rust 的 `Rink` 包（Dioxus 官方发布）

## 组件封装

Dioxus 对于组件的封装也很简单📦：

```rust

#[derive(Props)]
pub struct PostProps<'a> {
    title: &'a str,
    content: &'a str,
    author: u32,
    visible: bool
}

pub fn Post<'a>(cx: Scope<'a, PostProps<'a>>) -> Element {

    if !cx.props.visible {
        return None; 
    }

    let author = GetAuthorInfo(cx.props.author);

    cx.render(rsx!(
        div {
            class: "post",
            h1 { "{cx.props.title}" },
            div {
                p { "作者：{author.name}" }
                p { "cx.props.content" }
            }
        }
    ))
}
```

在上面的代码中，我尝试封装了一个 `Post` 组件，它包含：

- title 文章标题
- content 文章内容
- author 作者ID
- visible 是否可见

它根据不同的 4 个参数，渲染出来的页面也不一样，这对代码复用很方便，因为你不再需要关注一堆标签的结构了。而仅仅只需要考虑具体的数据内容：

```rust
// .... 组件代码 ....
fn app(cx: Scope) -> Element {
    cx.render(rsx!(
        div {
            class: "post-list",
            Post { /* .. 参数内容 .. */ }
            Post { /* .. 参数内容 .. */ }
            Post { /* .. 参数内容 .. */ }
            Post { /* .. 参数内容 .. */ }
        }
    ))
}
```

## 一些资源

目前我正在尝试帮助 `Dioxus` 构造其生态，同时也在完成一些周边的项目：

- [Golde](https://github.com/mrxiaozhuox/golde) - 用于在 Dioxus 中直接调用 JS 代码并获取其返回值。
- [Dioxus-Boostrap](https://github.com/mrxiaozhuox/dioxus-bootstrap) - 为 Dioxus 封装的 Bootstrap UI 框架。
- [Dioxus-Guide-CN](https://dioxus.mrxzx.info) - 对 Dioxus 文档的中文翻译。

目前 Dioxus 在国内还没有社区网站，如果后期项目发展的较好我也会搭建相关的网站。