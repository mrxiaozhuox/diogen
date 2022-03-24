---
title: Diogen 静态博客包
date: 2022-03-18 23:04:29
tags: [Dioxus, Diogen]
category: Rust
---

最近 `Dioxus` 0.2 发布了，我寻思着也使用这个版本做点啥小项目玩玩：[Diogen | static blog generator](https://github.com/mrxiaozhuox/diogen)

与其说是一个博客生成器，倒不如说是一个博客程序，因为只要你在 `HTML` 中引入了这个 `WASM` 文件，就可以直接渲染这个静态博客网站了！你甚至不需要配置服务器的伪静态，因为它会使用 `/#/XXX` 的方式做路由。换句话说，它就是一个但页面程序。每当你写完一篇博客，你需要做的仅仅是在 `index.json` 中加入新博客的名字，其他的 `WASM` 都会自动解析。



## 路由的实现

在这个程序中，我并没有直接套用 `dioxus-router` 工具包，而是自己写了一个简单的路由：

- 当 `Link` 组件被点击时，更新 `ROUTER` 全局状态到新的地址。
- 程序检测到 `ROUTER` 更新，对子组件（页面内容）进行切换。
- 更新 `/#/` 标记为新的路径，方便下一次复制路径时直接打开。

简单说，用一个 `State` 保存更新路由状态，然后每次打开页面检查 `/#/` 并恢复相应的组件。



## 内容加载

内容加载我使用了 `reqwasm` 通过 `GET` 请求访问现有资源（基于静态服务器的情况）

比如说这是 Index.json 的内容：

```json
[
    "dioxus.md",
    "hello.md",
]
```

那么我们便知总共有两篇文章了，加载相应内容或 `META` 数据则可以这样：

```
GET 请求： http://127.0.0.1/posts/dioxus.md
GET 请求： http://127.0.0.1/posts/hello.md
```

它并不需要任何其他帮助，这一切都是建立在 `静态` 服务之上的！



## Meta 解析

我特地实现了一个包用于解析 Markdown Meta 数据：[Markdown Meta Parser](https://crates.io/crates/markdown-meta-parser)

它可以解析：

```markdown
---
title: Hello World
date: 2022-03-18
tags: [hello, test]
---

接下来是 Markdown 的文本内容。
```

这样一来，配置单篇文章信息也变得非常简单了！
