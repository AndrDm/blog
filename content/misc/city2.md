---
title: city2
date: 2024-06-03
authorbox: false
sidebar: false
description: city2
categories:
  - Template
archives:
  - Jan-2023
tags:
  - template
draft: true
---
Text before Cut.
<!--more-->
TEST

<canvas id="animationCanvas" width="600" height="360" style="border:1px"></canvas>
<script>
var canvas
var canvasContext
let t = 0
window.onload = function() {
  canvas = document.getElementById('animationCanvas');
  ctx = canvas.getContext('2d');
  setInterval(drawCity, 33);
}

function drawCity(){
	canvas.width = 600;
	canvas.height = 360;
	ctx.scale(6, 6);
	for(w=99,++t,i=6e3;
		i--;
		ctx.fillRect(i%w,i/w|0,1-d*Z/w+s,1))
		for(a=i%w/50-1,s=b=1-i/4e3,X=t,Y=Z=d=1;
			++Z<w&(Y<6-(32<Z&27<X%w&&X/9^Z/8)*8%46||d|(s=(X&Y&Z)%3/Z,a=b=1,d=Z/w));
			Y-=b)X+=a;
ctx.rect(0, 0, 100, 60);
ctx.stroke();
}

</script>


END

