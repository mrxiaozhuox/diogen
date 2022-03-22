---
title: 高数学习笔记 - 函数
date: 2022-03-08 07:54:30
tags: 高数笔记
categories: Math
---

函数就是数集到数集的映射： $D \subset R$ （$R$ 为实数集）

$X$ 自变量，$Y$ 因变量；$y = f(x) \quad x \in D$ 

$D$ 为定义域，可记作 $D_f$；值域记作 $R_f$

$f$ 表示一个具体的规则，$f(x)$ 则表示一个具体的函数值。



### 构成函数的两要素

定义域 $D_f$ 以及对应的规则 $f$ 是构成一个函数的两个要素。

 

### 例：绝对值函数

$$
y = |x| = 
\begin{cases}
-x & x < 0 \\
x  & x \ge 0
\end{cases}
$$

$D = (-\infty, +\infty) \quad R_f = [0, +\infty)$



### 例：符号函数

$$
y = \operatorname{sgn}x = 
\begin{cases}
1 & x > 0 \\
0 & x = 0 \\
-1 & x < 0
\end{cases}
$$

符号函数表示了 $x$ 的正负性，当值大于零则为 1，小于 0 则为 -1，而等于 0 则就是零。一般缩写为 $\operatorname{sgn}x$

$x = \operatorname{sgn}x \cdot |x|$



### 例：取整函数

$$
[x]: \text{不超过} x \text{的最大整数}
$$

$$
[3.1] = 3 \quad [5] = 5 \quad [0] = 0 \quad [-1.6] = -2
$$

### 函数的特性

#### 有界性

- 上界：$\exists k_1 \quad f(x) \le k_1$
- 下界：$\exists k_2 \quad f(x) \ge k_2$
- 有界：$\exists \text{正数} M \quad |f(x)| \le M$
- 无界：$\forall \text{正数}M \quad \exists x_1 \in x \quad |f(x_1)| > M$



#### 单调性

单调增： $x_1 < x_2 \quad f(x_1) < f(x_2)$

单调减：$x_1 < x_2 \quad f(x_1) > f(x_2)$



#### 奇偶性

$D$ 必须关于原点对称

$f(-x) = f(x)$ 偶

$f(-x) = -f(x)$ 奇



#### 周期性

周期函数：$\exists 正数 l \quad f(x + l) = f(x)$ ；$l$ 即为它的周期

最小周期：$y = \sin x$ ；周期（最小周期）为 $2 \pi$

---

并非每个 **周期函数** 都有最小周期。
$$
D(x) = 
\begin{cases}
1 & x \in Q \\
0 & x \in Q^c
\end{cases}
$$
当 $x$ 为有理数时是 1，无理数则是 0

**不存在最小的正有理数作为周期**



### 反函数

设：$f: D \to f(D)$ 为单射

$f^{-1}: f(D) \to D$ 为反函数



如果 $f$ 为单调函数，且为单射：则 $f^{-1}$ 必然存在，且 $f^{-1}$ 单调。（$f^{-1}$与 $f$ 的单调增减一样）



### 复合函数

$y = f(t) \quad t = g(x)$

$y = f(g(x))$

$t$ 为中间变量，起过渡作用。

$R_g$ 必须存在于 $D_f$ 里



### 视频地址

[《高等数学》同济版 全程教学视频（宋浩老师）](https://www.bilibili.com/video/BV1Eb411u7Fw?p=3)
