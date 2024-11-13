---
title: Markdown — Math Functions
date: 2024-11-11
authorbox: false
sidebar: false
description: Notes about Math Equations in Markdown
categories:
  - Knowledge Base
archives:
  - 2024-11
tags:
  - Markdown
draft: false
---
Collection of the Math Examples
<!--more-->

Refer to https://support.typora.io/Math/

https://gohugo.io/content-management/mathematics/

### Base Math

Example:

````
$$
\begin{align*}
y = y(x,t) &= A e^{i\theta} \\
&= A (\cos \theta + i \sin \theta) \\
&= A (\cos(kx - \omega t) + i \sin(kx - \omega t)) \\
&= A\cos(kx - \omega t) + i A\sin(kx - \omega t)  \\
&= A\cos \Big(\frac{2\pi}{\lambda}x - \frac{2\pi v}{\lambda} t \Big) + i A\sin \Big(\frac{2\pi}{\lambda}x - \frac{2\pi v}{\lambda} t \Big)  \\
&= A\cos \frac{2\pi}{\lambda} (x - v t) + i A\sin \frac{2\pi}{\lambda} (x - v t)
\end{align*}
$$
````

$$
\begin{align*}
y = y(x,t) &= A e^{i\theta} \\
&= A (\cos \theta + i \sin \theta) \\
&= A (\cos(kx - \omega t) + i \sin(kx - \omega t)) \\
&= A\cos(kx - \omega t) + i A\sin(kx - \omega t)  \\
&= A\cos \Big(\frac{2\pi}{\lambda}x - \frac{2\pi v}{\lambda} t \Big) + i A\sin \Big(\frac{2\pi}{\lambda}x - \frac{2\pi v}{\lambda} t \Big)  \\
&= A\cos \frac{2\pi}{\lambda} (x - v t) + i A\sin \frac{2\pi}{\lambda} (x - v t)
\end{align*}
$$

### Chemistry

```
$$
$\ce{CH4 + 2 $\left( \ce{O2 + 79/21 N2} \right)$}$
$$
```

$$
$\ce{CH4 + 2 $\left( \ce{O2 + 79/21 N2} \right)$}$
$$

### Cross Reference

```
$$
x+1\over\sqrt{1-x^2}\label{ref1}\tag{1}
$$
```


$$
x+1\over\sqrt{1-x^2}\label{ref1}\tag{1}
$$

### Line Break

Line break within math mode.
```
$$ 
x + y = 2\\x - y = 4 
$$
```

$$
x + y = 2\\x - y = 4
$$



TeX Command not supported in Hugo

