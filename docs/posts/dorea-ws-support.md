---
title: DoreaDB WebSocket 通道支持
date: 2022-03-10 00:01:34
tags: [Dorea, WebSocket]
categories: Rust
---

之前在 `DoreaDB` 设计笔录中就聊到过，目前完成的 `Service` 性能并不好，仅仅因为它方便。

我一直在寻找一个好的解决方案；所以近日我尝试了为 `Service` 添加一个 `websocket` 管道：

## 连接演示

`WebSocket` 管道位于：`ws://127.0.0.1:3451/_ws/` 路径，通过 WS 就可以直接与数据库进行交互了：

```javascript
// 这里我们使用 JavaScript 做一个简单的连接演示：
let ws = new WebSocket("ws://127.0.0.1:3451/_ws/");

ws.onmessage = function(evt) {
	let result = evt.data;
    // dorea-ws 的回应信息都是 JSON 结构
    // { "alpha": "OK", "data": ... , "message": "" }
	console.log(result);
};

// 我们需要先使用 Service 的账号密码进行登陆（本次连接内
ws.send("login master DOREA@SERVICE");
// [LOG]: {"alpha":"OK","data":"...","message":""}
// 注：这里会返回账号的具体信息

ws.send("set number 3.14");
// [LOG]: {"alpha":"OK","data":"","message":""}

ws.send("get number");
// [LOG]: {"alpha":"OK","data":"3.14","message":""}

```

## 相比优势

`WebSocket` 连接内部同样是开启了一个原生的 `TCP + Dorea Protocol` 的客户端，但是它不同于普通的 `HTTP\HTTPS` 请求，它不会在每一次交互、执行时都开启新的连接到数据库，而是一次 `ws` 连接仅开启一个客户端到服务器的连接。

它的交互体验更接近于原本的 `TCP` 客户端，你不用担心连接内的一些状态（如 Style 配置）丢失。

目前，我会将 `ws` 作为我的主用连接方案（最近我正在使用 **Dioxus** 为 Dorea 开发一款管理工具）



## 实现方法

通过 **Axum** 的 `ws` *Feature* 实现：[axum/examples/websockets · tokio-rs/axum](https://github.com/tokio-rs/axum/tree/main/examples/websockets)
