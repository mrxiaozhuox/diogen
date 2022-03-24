---
title: Dioxus 0.2 版本发布
date: 2022-03-13 13:28:08
tags: Dioxus
category: Rust
---

`Dioxus` 近期发布了 **0.2** 的新版本，贴一份原文地址：[Dioxus 0.2 Release: TUI, Router, Fermi, and Tooling](https://dioxuslabs.com/blog/release-020/)

## 新增特性

- 完成了原生的 `Terminal` 渲染器，类似于 `Ink.JS` 
- 完成了新的 `Router` 路由器程序
- 完成了 `Fermi` 全局状态管理，类似于 `Recoil.JS`
- 桌面平台进行了重大更新，支持了一些 `Window API`
- 完成了 `CLI` 工具的基本开发，它现在可以使用了
- Web 平台的应用效率有了重大提升（比 `React` 快 3 倍）

其实新功能和变更还蛮多的，下面就聊聊我最感兴趣的几个点吧。

## 路由管理

新版本中的 `Router` 其实是满强大的，它同时支持 Desktop & Web 平台。

> 有一个问题：在 Web 端，他需要一个 Server 的支持，这源于它的路由切换方式。

贴一段来自官方的代码：

```rust
fn app(cx: Scope) -> Element {
    cx.render(rsx! {
        Router {
            onchange: move |_| log::info!("Route changed!"),
            ul {
                Link { to: "/",  li { "Go home!" } }
                Link { to: "users",  li { "List all users" } }
                Link { to: "blog", li { "Blog posts" } }
            }
            Route { to: "/", "Home" }
            Route { to: "/users", "User list" }
            Route { to: "/users/:name", User {} }
            Route { to: "/blog", "Blog list" }
            Route { to: "/blog/:post", BlogPost {} }
            Route { to: "", "Err 404 Route Not Found" }
        }
    })
}
```

我们在 `app` 下创建了一个 `Router` 组件，并将我们其他的所有组件都包含在里面。

`Route` 可以创建一个新的路由项，比如 `Route { to: "/users", "User List" }` 这一段代码：

当你访问的路径为 `/users` 时，则它会显示内容：`User List` （这里的 `User List` 可以为任何组件和元素）

而 `Link` 则用于创建一个切换到对应的路径中（在 Web 中它也会跳转到目标的 URL 中）

## 全局状态管理

`Fermi` 本来是一个独立的 crate 包，但在这个版本被引入到 Dioxus 中了（需要通过 features 添加）

它支持我们读写一个全局定义的状态信息：

```rust
// Create a single value in an "Atom"
static TITLE: Atom<&str> = |_| "Hello";

// Read the value from anywhere in the app, subscribing to any changes
fn app(cx: Scope) -> Element {
    let title = use_read(&cx, TITLE);
    cx.render(rsx!{
        h1 { "{title}" }
        Child {}
    })
}

// Set the value from anywhere in the app
fn Child(cx: Scope) -> Element {
    let set_title = use_set(&cx, TITLE);
    cx.render(rsx!{
        button {
            onclick: move |_| set_title("goodbye"),
            "Say goodbye"
        }
    })
}
```

- 设置一个 `static` 且类型为 `Atom<T>` 的闭包函数（返回默认值）
- 通过 `use_read` 可以读取全局状态内容
- 通过 `use_set` 可以设置新的全局状态内容

通过这种方案，我们便不需要将 State 对象不断传递了，仅仅使用全局状态就能完成！



##  异步程序改进

新版本对异步支持也做了一些简单的改进，主要在 `use_future` 和 `use_coroutine` 上：

```rust
fn RenderDog(cx: Scope, breed: String) -> Element {
    let dog_request = use_future(&cx, (breed,), |(breed,)| async move {
        reqwest::get(format!("https://dog.ceo/api/breed/{}/images/random", breed))
            .await
            .unwrap()
            .json::<DogApi>()
            .await
    });

    cx.render(match dog_request.value() {
        Some(Ok(url)) => rsx!{ img { url: "{url}" } },
        Some(Err(url)) => rsx!{ span { "Loading dog failed" }  },
        None => rsx!{ "Loading dog..." }
    })
}
```

`use_future` 新增了一个 `dep` 参数，如果这个值被更新了（可以理解为与上一次调用的值不一样了），则 `Future` 任务会被重新执行。在上面的演示中，当上级传递过来的 `bread` 被更新时，整个 API 也会被重新调用，从而获取新的数据。并且根据 `use_future` 的返回值，我们可以了解到当前任务的状态。

接下来是 `use_oroutine` 方法：

```rust
fn App(cx: Scope) -> Element {
    let sync_task = use_coroutine(&cx, |rx| async move {
        connect_to_server().await;
        let state = MyState::new();

        while let Some(action) = rx.next().await {
            reduce_state_with_action(action).await;
        }
    });

    cx.render(rsx!{
        button {
            onclick: move |_| sync_task.send(SyncAction::Username("Bob")),
            "Click to sync your username to the server"
        }
    })
}
```

它可以开启一个管道，让你在通过发送一些内容到任务内部进行交互处理。
