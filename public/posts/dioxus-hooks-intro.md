---
title: Dioxus Hooks 设计研究
date: 2022-03-06 14:44:46
tags: [Dioxus, Hooks]
category: Rust
---

> 本篇我们将分析 Dioxus 中的 Hooks 设计，Hooks 借鉴自 React，它也是整个 Dioxus 中最重要的部分之一

附一篇 React 的 Hooks 介绍文档：[Hook 规则 – React](https://zh-hans.reactjs.org/docs/hooks-rules.html)，Dioxus 的 Hooks 基本上都是参照 React 开发的。

## Hooks 介绍

在 Dioxus 中，目前提供了多种 Hook 函数：

- `use_state` - 储存状态数据，并自动更新
- `use_ref` - 储存不可 `Clone` 的数据，使用 `RefCell`
- `use_future` - 储存初始化后需要 `Polled` 的任务
- `use_coroutine` - 储存可以被  停止、启动、通讯 的 `Future`
- `use_context_provider` - 暴露 `state` 信息给下级组件
- `use_context` - 使用来自 `use_context_provider` 的内容

在不同的情况下，我们常常会用到不同的 Hooks 函数，它们使我们能很方便的开发应用程序与组件。

## Hooks 规则

鉴于 Hooks 的设计特殊性，我们需要注意：

- 不要在任何 Hooks 的回调中嵌套 Hooks 函数。
- 不要在任何条件判断中使用 Hooks 函数。
- 不要在任何循环体中使用 Hooks 函数。

> 具体的原因我们会在解析源码的部分讲解qwq

### 嵌套使用

```rust
// ❌ 不要在一个 `use_` Hooks 中嵌套 获取\使用 另外一个
cx.use_hook(|_| {
    let name = cx.use_hook(|_| "ads");
})

// ✅ 这样使用就没有任何问题
let name = cx.use_hook(|_| "ads");
cx.use_hook(|_| {
    // 一些相关代码
})
```

### 判断语句内

```rust
// ❌ 不要在任何包含条件判断的代码内使用它
if do_thing {
    let name = use_state(&cx, || 0);
}

// ✅ 先使用 Hooks 再根据情况是否在判断内调用
let name = use_state(&cx, || 0);
if do_thing {
    // 在这里就可以使用 name 内容了
}
```

### 循环体内

```rust
// ❌ 不要在循环中使用 Hooks 函数
let mut nodes = vec![];

for name in names {
    let age = use_state(&cx, |_| 0);
    nodes.push(cx.render(rsx!{
        div { "{age}" }
    }))
}

// ✅ 考虑将重复使用的部分重构为组件
#[inline_props]
fn Child(cx: Scope, name: String) -> Element {
    let age = use_state(&cx, |_| 0);
    cx.render(rsx!{ div { "{age}" } })
}

// ✅ 或者使用 use_ref 保存 HashMap
```rust
let ages = use_ref(&cx, || HashMap::new());

names.iter().map(|name| {
    let age = ages.get(name).unwrap();
    cx.render(rsx!{ div { "{age}" } })
})
```

## Hooks 设计

接下来将进入我们今天的主要内容，研究 Hooks 系统是如何运行的，以及它的代码是怎么样的！

Hooks Package 代码：[dioxus/packages/hooks](https://github.com/DioxusLabs/dioxus/tree/a4ab2d9de0e6d9c5ba7845d9fff78e6b2cdb9fc6/packages/hooks)

`use_hook` 实现代码：[dioxus/scopes.rs](https://github.com/DioxusLabs/dioxus/blob/a4ab2d9de0e6d9c5ba7845d9fff78e6b2cdb9fc6/packages/core/src/scopes.rs#L834)

### Hooks 与 组件刷新

在我们使用 `use_state` 和 `use_ref` 时，我们是可以对内部的数据进行 `Read / Write` 操作的，而如果一个数据被更新，则会被标记 `dirty` （需要更新的），此时我们的页面中的相应部分也会被自动重新渲染。

在重新渲染时，组件内部的代码就会被重复的调用。比方说我们尝试这段组件代码：

```rust
fn app(cx: Scope) -> Element {
    
    let num = use_state(&cx, || 0);

    println!("App 组件被加载了！");

    cx.render(rsx! {
        div {
            "num: {num}",
        }
        button {
            onclick: move |_| num.set(num.get() + 1),
            "num ++"
        }
    })
}
```

我们使用了一个数字类型的 `state`，并在按钮被按下后对它进行 `+1` 的操作。

此时，请注意 `println!` 的内容，每当我们更新一次 state，它都会被打印，这说明我们整个组件都被更新了（因为我们在这个组件函数中使用了 `num` 值），相当于我们的 Hooks 会保存之前的状态，在组件刷新时并不会影响到它的内容。

### Hooks 状态储存

是不是很好奇，既然组件函数被刷新了，那 `use_state` 里的数据被储存到哪里去了呢？

通过查阅 `use_state` 函数的源代码，我们可以发现：它使用了 `cx.use_hook`这一函数。

我们尝试这么一段代码：

```rust
let v = cx.use_hook(|_| 0);
println!("{v}");
*v += 1;
```

发现，它并不会主动的刷新组件函数，但是当每一次组件被其他原因更新后，它依旧会保存之前的内容。

那么我们所看到的所有 `use_` 函数，都是基于 `cx.use_hook` 的。那我们来分析一下它的源代码吧：

```rust
pub fn use_hook<'src, State: 'static>(
    &'src self,
    initializer: impl FnOnce(usize) -> State,
) -> &'src mut State {
    let mut vals = self.hook_vals.borrow_mut();

    let hook_len = vals.len();
    let cur_idx = self.hook_idx.get();

    if cur_idx >= hook_len {
        vals.push(self.hook_arena.alloc(initializer(hook_len)));
    }

    vals
        .get(cur_idx)
        .and_then(|inn| {
            self.hook_idx.set(cur_idx + 1);
            let raw_box = unsafe { &mut **inn };
            raw_box.downcast_mut::<State>()
        })
        .expect("这里是一些错误信息；大概就是顺序出问题了，你可能用在循环判断里了。")
}
```

我们可以看到，`hooks_val`  实际上是以一个 `Vec` 结构被存放在 `Scope` 中的，并且有一个 `hook_idx` 来检查当前处理到哪一个 hook 了，这也是为什么 Hooks 不能用在判断、循环体中的原因；因为每一次调用都是按顺序检查存放的值的，如果顺序错了会导致读取到其他 Hooks 中的值，值类型不一样甚至会导致程序直接崩溃掉。它并没有所谓的键值对，一个 Key 对应一条数据，而是按照顺序存放的数据。

### `use_state` 自动刷新

上面已经聊到过 Hooks 中的自动刷新了，但那仅仅是我们进入进入 Hooks 的引子罢了；接下来我们来了解一下为什么 `use_state` 能做到值被更新时就自动刷新呢？先上源代码：

```rust
pub fn use_state<'a, T: 'static>(
    cx: &'a ScopeState,
    initial_state_fn: impl FnOnce() -> T,
) -> &'a UseState<T> {
    let hook = cx.use_hook(move |_| {
        let current_val = Rc::new(initial_state_fn());
        let update_callback = cx.schedule_update();
        let slot = Rc::new(RefCell::new(current_val.clone()));
        let setter = Rc::new({
            dioxus_core::to_owned![update_callback, slot];
            move |new| {
                {
                    let mut slot = slot.borrow_mut();
                    if let Some(val) = Rc::get_mut(&mut slot) {
                        *val = new;
                    } else {
                        *slot = Rc::new(new);
                    }
                }
                update_callback();
            }
        });

        UseState {
            current_val,
            update_callback,
            setter,
            slot,
        }
    });
    hook.current_val = hook.slot.borrow().clone();
    hook
}
```

其中，在 `UseState` 结构体中，我们主要要使用 `setter` 来进行更新，再看看 setter 完成了一些什么工作呢？它在更新 `slot` 数据后，调用了这个 `update_callback` 函数，而这个函数本质上就是使用 `cx.schedule_update` 方法。

我们再来试试这个呢：

```rust
fn app(cx: Scope) -> Element {
    println!("Update!!");
    cx.render(rsx! {
        button {
            onclick: move |_| {
                cx.schedule_update()()
            },
            "update"
        }
    })
}
```

每当我们点击按钮后，组件都会被重载！这也是为什么 `use_state` 能自动刷新组件的原因了！

## 异步 Hooks

通过使用 `use_future` 和 `use_coroutine` 我们可以启动一个异步任务代码块：

```rust
fn app(cx: Scope) -> Element {
    let ip = use_state(&cx, String::new);
    use_future(&cx, (), |_| {
        let ip_setter = ip.setter();
        async move {
            let resp = reqwest::get("https://httpbin.org/ip")
                .await
                .unwrap()
                .json::<HashMap<String, String>>()
                .await
                .unwrap();
            ip_setter(resp.get("origin").unwrap().to_string());
        }
    });

    cx.render(rsx! {
        div {
            "IP地址：{ip}"
        }
    })
}
```

这里我们使用了 `reqwest` 请求一个API地址，当 reqwest 的异步任务完成后，会更新 `state` IP 的内容，并渲染到页面上。
