---
title: "Interactive Pendulum Wave Simulation"
date: 2026-07-03
description: "Explore the fascinating behavior of multiple pendulums with different lengths oscillating from a common suspension point. Observe synchronization, phase shifts, and periodic realignment using an accurate nonlinear pendulum model and RK4 numerical integration."
archives:
  - 2026-07
draft: false
---

A pendulum wave is a mesmerizing physical phenomenon created by multiple pendulums of different lengths oscillating from the same support. Although all pendulums start in phase, their slightly different natural periods gradually cause them to drift apart, forming beautiful wave-like patterns. After a certain amount of time, the system partially or fully synchronizes again, creating repeating visual structures.

This interactive simulation lets you experiment with pendulum lengths, initial angle, gravity, and time scale while observing how differences in oscillation period influence phase relationships between multiple pendulums.

<!--more-->

## Brief Theory

A mathematical pendulum consists of a point mass suspended from a massless, inextensible string.

For small oscillation angles, the period of a pendulum is approximately:

**T = 2π√(L/g)**

where:

- **T** is the oscillation period,
- **L** is the pendulum length,
- **g** is the gravitational acceleration.

Because the oscillation period depends on the square root of the pendulum length, longer pendulums swing more slowly than shorter ones.

When multiple pendulums of different lengths start from the same initial angle, their motions gradually drift out of phase. This produces evolving geometric patterns known as a **pendulum wave**. At specific moments, the pendulums may align again, creating striking synchronization effects.

This simulation models ten pendulums suspended from a common point. Their lengths are evenly distributed between the selected minimum and maximum values, allowing you to explore how small changes in period lead to complex collective motion.

The motion of each pendulum is calculated using the nonlinear equation:

**θ'' + (g/L)sin(θ) = 0**

Unlike the small-angle approximation, this equation remains accurate for larger oscillation amplitudes.

Numerical integration is performed using the **fourth-order Runge–Kutta (RK4)** method, providing stable and accurate results throughout the simulation.
<div id="m-demo">

<style>
#m-demo *{
    box-sizing:border-box;
}

#m-demo .wrapper{
    display:flex;
    flex-wrap:wrap;
    gap:12px;
    width:100%;
    max-width:100%;
    align-items:flex-start;
}

#m-demo .canvas-wrap{
    position:relative;
    width:100%;
    max-width:400px;
    height:720px;
    background:#ffffff;
    border:1px solid #d1d5db;
    overflow:hidden;
    flex:0 0 400px;
}

#m-demo canvas{
    display:block;
    width:100%;
    height:100%;
}

#m-demo .panel{
    flex:1 1 320px;
    min-width:280px;
    max-width:100%;
    background:#ffffff;
    padding:12px;
    overflow:auto;
    border-left:1px solid #d1d5db;
}

#m-demo h1{
    margin:0 0 8px;
    font-size:1.5rem;
}

#m-demo h2{
    margin:0 0 8px;
    font-size:1.1rem;
}

#m-demo .card{
    background:#ffffff;
    padding:10px;
    border-radius:12px;
    margin-bottom:10px;
}

#m-demo label{
    display:block;
    margin:6px 0 2px;
    font-size:.9rem;
}

#m-demo input[type=range]{
    width:100%;
}

#m-demo .value{
    color:#0284c7;
    font-weight:bold;
}

#m-demo .controls{
    display:flex;
    gap:6px;
    flex-wrap:wrap;
    margin-top:10px;
}

#m-demo button{
    border:none;
    border-radius:8px;
    padding:8px 12px;
    cursor:pointer;
    font-weight:600;
    font-size:.9rem;
}

#m-demo .start{
    background:#16a34a;
    color:#fff;
}

#m-demo .pause{
    background:#ea580c;
    color:#fff;
}

#m-demo .reset{
    background:#2563eb;
    color:#fff;
}

#m-demo .stats{
    display:grid;
    grid-template-columns:1fr 1fr;
    gap:6px;
}

#m-demo .stat{
    background:#0f172a;
    color:#fff;
    padding:8px;
    border-radius:8px;
    font-size:.9rem;
}

#m-demo .small{
    font-size:.85rem;
    line-height:1.4;
}

@media (max-width:1000px){

    #m-demo .wrapper{
        flex-direction:column;
    }

    #m-demo .canvas-wrap{
        width:100%;
        max-width:400px;
        height:720px;
        margin:0 auto;
        flex:none;
    }

    #m-demo .panel{
        width:100%;
        min-width:auto;
        border-left:none;
    }
}
</style>

<!-- No Empty Lines in div section! -->
<div class="wrapper">
    <div class="canvas-wrap">
        <canvas id="canvas"></canvas>
    </div>
    <div class="panel">
        <h1>Mathematical Pendulums</h1>
        <div class="card">
			<label>Minimum Length: <span class="value" id="minLenVal">2.0</span> m</label>
			<input id="minLength" type="range" min="0.5" max="5" step="0.1" value="2">
			<label>Maximum Length: <span class="value" id="lenVal">5.0</span> m</label>
			<input id="length" type="range" min="0.5" max="5" step="0.1" value="5">
            <label>Initial Angle: <span class="value" id="angVal">20</span>°</label>
            <input id="angle" type="range" min="1" max="80" step="1" value="20">
            <label>Gravitational Acceleration: <span class="value" id="gVal">9.81</span> m/s²</label>
            <input id="gravity" type="range" min="1.6" max="24" step="0.01" value="9.81">
            <label>Time Scale: <span class="value" id="speedVal">1.0</span>x</label>
            <input id="speed" type="range" min="0.2" max="4" step="0.1" value="1">
            <div class="controls">
                <button class="start" id="startBtn">Start</button>
                <button class="pause" id="pauseBtn">Pause</button>
                <button class="reset" id="resetBtn">Reset</button>
            </div>
        </div>
        <div class="card">
            <h2>Current Values</h2>
            <div class="stats">
                <div class="stat">Angle<br><b id="curAngle">0°</b></div>
                <div class="stat">Lengths<br><b id="curLength">2.0–5.0 m</b></div>
                <div class="stat">Period<br><b id="period">0 s</b></div>
                <div class="stat">Frequency<br><b id="freq">0 Hz</b></div>
                <div class="stat">Time<br><b id="simTime">0 s</b></div>
                <div class="stat">g<br><b id="curG">9.81</b></div>
            </div>
        </div>
    </div>
</div>

<script>
const canvas = document.getElementById("canvas");
const ctx = canvas.getContext("2d");

const lengthSlider = document.getElementById("length");
const minLengthSlider = document.getElementById("minLength");
const angleSlider = document.getElementById("angle");
const gravitySlider = document.getElementById("gravity");
const speedSlider = document.getElementById("speed");

const lenVal = document.getElementById("lenVal");
const angVal = document.getElementById("angVal");
const gVal = document.getElementById("gVal");
const speedVal = document.getElementById("speedVal");

let running = false;
let time = 0;

const COUNT = 10;

const COLORS = [
    "#ef4444",
    "#f97316",
    "#f59e0b",
    "#84cc16",
    "#22c55e",
    "#10b981",
    "#06b6d4",
    "#3b82f6",
    "#8b5cf6",
    "#ec4899"
];

let pendulums = [];

function hexToRgba(hex, alpha){

    const r = parseInt(hex.slice(1,3),16);
    const g = parseInt(hex.slice(3,5),16);
    const b = parseInt(hex.slice(5,7),16);

    return `rgba(${r},${g},${b},${alpha})`;
}

function buildPendulums(){

    pendulums = [];

	const minL = Math.min(
    Number(minLengthSlider.value),
    Number(lengthSlider.value) - 0.1
);

const maxL = Number(lengthSlider.value);

    for(let i = 0; i < COUNT; i++){

        const length =
            minL +
            (maxL - minL) * i / (COUNT - 1);

        pendulums.push({
            L: length,
            theta: Number(angleSlider.value) * Math.PI / 180,
            omega: 0,
            color: COLORS[i]
        });
    }
}

function resize(){
    canvas.width = canvas.clientWidth;
    canvas.height = canvas.clientHeight;
}

window.addEventListener("resize", resize);
resize();

function updateLabels(){
    lenVal.textContent = lengthSlider.value;
    angVal.textContent = angleSlider.value;
    gVal.textContent = gravitySlider.value;
    speedVal.textContent = Number(speedSlider.value).toFixed(1);

document.getElementById("curLength").textContent =
Number(minLengthSlider.value).toFixed(1) +
"–" +
Number(lengthSlider.value).toFixed(1) +
" m";

    document.getElementById("curG").textContent = gravitySlider.value;

    const L = Number(lengthSlider.value);
    const g = Number(gravitySlider.value);
    const T = 2 * Math.PI * Math.sqrt(L / g);

    document.getElementById("period").textContent = T.toFixed(2) + " s";
    document.getElementById("freq").textContent = (1 / T).toFixed(2) + " Hz";
}

updateLabels();

[
	minLengthSlider,
    lengthSlider,
    angleSlider,
    gravitySlider,
    speedSlider
].forEach(el => {
    el.addEventListener("input", () => {
        updateLabels();
        buildPendulums();
    });
});

function resetSimulation(){
    time = 0;
    buildPendulums();
}

resetSimulation();

document.getElementById("startBtn").onclick = () => running = true;
document.getElementById("pauseBtn").onclick = () => running = false;

document.getElementById("resetBtn").onclick = () => {
    running = false;
    resetSimulation();
};

function derivatives(theta, omega, length){

    const g = Number(gravitySlider.value);

    return {
        dtheta: omega,
        domega: -(g / length) * Math.sin(theta)
    };
}

function rk4(p, dt){

    const k1 = derivatives(
        p.theta,
        p.omega,
        p.L
    );

    const k2 = derivatives(
        p.theta + k1.dtheta * dt / 2,
        p.omega + k1.domega * dt / 2,
        p.L
    );

    const k3 = derivatives(
        p.theta + k2.dtheta * dt / 2,
        p.omega + k2.domega * dt / 2,
        p.L
    );

    const k4 = derivatives(
        p.theta + k3.dtheta * dt,
        p.omega + k3.domega * dt,
        p.L
    );

    p.theta += dt *
        (k1.dtheta + 2*k2.dtheta + 2*k3.dtheta + k4.dtheta) / 6;

    p.omega += dt *
        (k1.domega + 2*k2.domega + 2*k3.domega + k4.domega) / 6;
}

function drawGrid(){
    ctx.strokeStyle = "#d1d5db";
    ctx.lineWidth = 1;

    for(let x = 0; x < canvas.width; x += 40){
        ctx.beginPath();
        ctx.moveTo(x,0);
        ctx.lineTo(x,canvas.height);
        ctx.stroke();
    }

    for(let y = 0; y < canvas.height; y += 40){
        ctx.beginPath();
        ctx.moveTo(0,y);
        ctx.lineTo(canvas.width,y);
        ctx.stroke();
    }
}

let last = performance.now();

function frame(now){

    const realDt = (now - last) / 1000;
    last = now;

    const dt = realDt * Number(speedSlider.value);

    if(running){
        rk4(dt);
        time += dt;
    }

    ctx.clearRect(0,0,canvas.width,canvas.height);
    drawGrid();

	const originX = canvas.width / 2;
	const originY = 40;

	for(let i = 0; i < pendulums.length; i++){

    	const p = pendulums[i];

   		if(running){
        	rk4(p, dt);
    	}

    	const visualLength = p.L * 95;

    	const bobX =
        	originX +
        	visualLength * Math.sin(p.theta);

    	const bobY =
        	originY +
        	visualLength * Math.cos(p.theta);

    	ctx.beginPath();
    	ctx.arc(
        	originX,
        	originY,
        	3,
        	0,
        	Math.PI * 2
    	);
    	ctx.fillStyle = "#64748b";
    	ctx.fill();

    	ctx.beginPath();
    	ctx.moveTo(originX, originY);
    	ctx.lineTo(bobX, bobY);
    	ctx.strokeStyle = hexToRgba(p.color, 0.45);
    	ctx.lineWidth = 1.5;
    	ctx.stroke();

    	ctx.beginPath();
    	ctx.arc(
        	bobX,
        	bobY,
        	8,
        	0,
        	Math.PI * 2
    	);
    	ctx.fillStyle = p.color;
    	ctx.fill();
	}
	document.getElementById("curAngle").textContent =
	COUNT + " pendulums";

    document.getElementById("simTime").textContent =
        time.toFixed(2) + " s";

	requestAnimationFrame(frame);
}

requestAnimationFrame(frame);
</script>

</div>
