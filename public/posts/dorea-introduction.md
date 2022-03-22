---
title: DoreaDB 预览版发布
date: 2021-09-27 23:40:33
tags: [Dorea]
categories: Rust
---

在昨天晚上，我发布了 `DoreaDB` 数据库的最新预览版本：`v0.3.0-alpha`

这个版本与前面两个版本 `v0.2.1` 和 `v0.1.0` 最大的区别就是：**它是一个完全重写的系统！**

在 `v0.2.1` 发布后，我发现了一些代码问题以及设计思路问题，这些东西已经不单单是能通过修改代码的方式能弥补的了，所以说我决定重写这个项目！这次重写的周期大概在两三个月时间，项目的类型系统、TCP通讯、存储模型都使用了更加优秀的设计方案。 

## 数据类型

DoreaDB v0.3.0 支持以下数据类型

- **Null** 空值（仅在复合中有意义）
- **String** 字符串
- **Number** 含符号和小数位的数字
- **Boolean** 真假值（布尔值）
- **List** 列表（数组）
- **Dict** 字典（键值对）
- **Tuple** 元组（Pair）

前四种为基本数据类型，后三种为复合类型（复合类型中可包含七种类型的任意值）

> Null 如果作为根 Value 则不会被识别为正常值，而是作为一个已被删除的数据存在的。

### 类型识别

`DoreaDB` 为了方便与其他语言对接，支持从 `字符串` 中自动根据结构转换数据类型。

#### String

使用双引号包含的值为字符串值：

```
"hello world" ✅
zhuoer_liu    ❌
"123456"      ✅
123456        ❌
```

#### Number

任何正负数字（可带小数点）
```
50   ✅
32   ✅
3.14 ✅
-32  ✅
```

#### Boolean 

布尔值只有两个可能性：

```
True  ✅
False ✅
```

#### List

列表是一个复合类型，它可以存放上面的任何一种数据。

```
[
    "这是字符串",
    3.1415,
    True,
    "又是一个字符串",
    [
        "可以再套数组的",
        18,
        False
    ]
]
```

#### Dict

字典就是一个键值对的复合类型：

```
{
    "name": "ZhuoEr Liu"
    "age": 18,
    "bool": True,
    "tags": ["tag-1", "tag-2"]
}
```

#### Tuple

元组就是两个数据的组合：

```
("PI", 3.14)
```

## 存储系统

使用 `Bitcask` 存储方案，即 日志型 操作：只存在插入操作，增删改查都是插入的操作。

### 增操作

直接在数据库文件后续写入一条信息。

```
{"key": "foo", "value": "String(\"bar\")", other: {...}}
```

### 删操作

将一条数据覆盖为 Null 值。

```
{"key": "foo", value: "Null", other: {...}}
```

### 改操作

与 `增` 同理，后面的会覆盖前面的。

```
{"key": "foo", "value": "String(\"hello world\")", other: {...}}
```

### 查操作

通过内存中加载的索引位置直接读取到相应行的数据内容。

```
{
    "key": "foo",
    "index": 3,
    "group": "default",
    "fileid": 1
}
```

### 存储格式

`DoreaDB` 在目前版本：(v0.3.0-alpha) 下采用的是 JSON 格式的存储（后续可能会更新）

以下为从个人测试集中抽取的数据项：

```
{
    "crc":3114172235,
    "key":"plugins",
    "value":{
        "List":[
            {
                "String":"example"
            }
        ]
    },
    "time_stamp":[1632404275,0]
}
```

## 下载地址

[Github Release](https://github.com/mrxiaozhuox/dorea/releases "mrxiaozhuox/doreadb")

## 截图

![](https://upc.cloud.wwsg18.com/uploads%2F2021%2F08%2F26%2F1_5PJTELnd_%E6%B7%B1%E5%BA%A6%E6%88%AA%E5%9B%BE_20210826191636.png)