---
title: "Interactive Mathematical Pendulum Simulation"
date: 2026-07-03
description: "Explore the motion of a mathematical pendulum using a real-time interactive simulation. Adjust pendulum length, initial angle, gravity, and time scale while observing period, frequency, and angular displacement calculated using a nonlinear physical model."
archives:
  - 2026-07
draft: false
---

Mathematical pendulums are among the most fundamental oscillating systems studied in physics. This interactive simulation allows you to explore how pendulum motion depends on length, gravity, and initial displacement. The model uses the full nonlinear pendulum equation and integrates the motion using a fourth-order Runge–Kutta method for improved numerical accuracy.

<!--more-->

## Brief Theory

A mathematical pendulum consists of a point mass suspended from a massless, inextensible string.

For small oscillation angles, the period is given by:

**T = 2π√(L/g)**

where:

- **T** is the oscillation period,
- **L** is the pendulum length,
- **g** is the gravitational acceleration.

Increasing the pendulum length increases the oscillation period.

Increasing gravitational acceleration decreases the oscillation period.

This simulation uses the full nonlinear equation of motion:

**θ'' + (g/L)sin(θ) = 0**

The equation is numerically integrated using the **fourth-order Runge–Kutta (RK4)** method, providing accurate results even for larger oscillation amplitudes.
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
        <h1>Mathematical Pendulum</h1>
        <div class="card">
            <label>Pendulum Length: <span class="value" id="lenVal">2.0</span> m</label>
            <input id="length" type="range" min="0.5" max="5" step="0.1" value="2">
            <label>Initial Angle: <span class="value" id="angVal">20</span>°</label>
            <input id="angle" type="range" min="1" max="80" step="1" value="20">
            <label>Gravitational Acceleration: <span class="value" id="gVal">9.81</span> m/s²</label>
            <input id="gravity" type="range" min="1.6" max="24" step="0.01" value="9.81">
            <label>Time Scale: <span class="value" id="speedVal">1.0</span>x</label>
            <input id="speed" type="range" min="0.2" max="4" step="0.1" value="1">
            <label><input type="checkbox" id="trail" checked> Show Motion Trail</label>
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
                <div class="stat">Length<br><b id="curLength">2 m</b></div>
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
const angleSlider = document.getElementById("angle");
const gravitySlider = document.getElementById("gravity");
const speedSlider = document.getElementById("speed");
const trailBox = document.getElementById("trail");

const lenVal = document.getElementById("lenVal");
const angVal = document.getElementById("angVal");
const gVal = document.getElementById("gVal");
const speedVal = document.getElementById("speedVal");

let running = false;
let theta = 20 * Math.PI / 180;
let omega = 0;
let time = 0;
let trail = [];

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

    document.getElementById("curLength").textContent = lengthSlider.value + " m";
    document.getElementById("curG").textContent = gravitySlider.value;

    const L = Number(lengthSlider.value);
    const g = Number(gravitySlider.value);
    const T = 2 * Math.PI * Math.sqrt(L / g);

    document.getElementById("period").textContent = T.toFixed(2) + " s";
    document.getElementById("freq").textContent = (1 / T).toFixed(2) + " Hz";
}

updateLabels();

[lengthSlider, angleSlider, gravitySlider, speedSlider].forEach(el => {
    el.addEventListener("input", updateLabels);
});

function resetSimulation(){
    theta = Number(angleSlider.value) * Math.PI / 180;
    omega = 0;
    time = 0;
    trail = [];
}

resetSimulation();

document.getElementById("startBtn").onclick = () => running = true;
document.getElementById("pauseBtn").onclick = () => running = false;

document.getElementById("resetBtn").onclick = () => {
    running = false;
    resetSimulation();
};

function derivatives(th, om){
    const L = Number(lengthSlider.value);
    const g = Number(gravitySlider.value);

    return {
        dtheta: om,
        domega: -(g / L) * Math.sin(th)
    };
}

function rk4(dt){
    const k1 = derivatives(theta, omega);

    const k2 = derivatives(
        theta + k1.dtheta * dt / 2,
        omega + k1.domega * dt / 2
    );

    const k3 = derivatives(
        theta + k2.dtheta * dt / 2,
        omega + k2.domega * dt / 2
    );

    const k4 = derivatives(
        theta + k3.dtheta * dt,
        omega + k3.domega * dt
    );

    theta += dt * (k1.dtheta + 2*k2.dtheta + 2*k3.dtheta + k4.dtheta) / 6;
    omega += dt * (k1.domega + 2*k2.domega + 2*k3.domega + k4.domega) / 6;
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
    const originY = 60;
    const visualLength = Number(lengthSlider.value) * 95;

    const bobX = originX + visualLength * Math.sin(theta);
    const bobY = originY + visualLength * Math.cos(theta);

    if(trailBox.checked){
        trail.push({x:bobX,y:bobY});

        if(trail.length > 800){
            trail.shift();
        }

        ctx.beginPath();

        for(let i=0;i<trail.length;i++){
            const p = trail[i];

            if(i === 0){
                ctx.moveTo(p.x,p.y);
            }else{
                ctx.lineTo(p.x,p.y);
            }
        }

        ctx.strokeStyle = "#38bdf8";
        ctx.lineWidth = 2;
        ctx.stroke();
    }

    ctx.beginPath();
    ctx.arc(originX, originY, 6, 0, Math.PI*2);
    ctx.fillStyle = "#64748b";
    ctx.fill();

    ctx.beginPath();
    ctx.moveTo(originX, originY);
    ctx.lineTo(bobX, bobY);
    ctx.strokeStyle = "#94a3b8";
    ctx.lineWidth = 3;
    ctx.stroke();

    ctx.beginPath();
    ctx.arc(bobX, bobY, 18, 0, Math.PI*2);
    ctx.fillStyle = "#22c55e";
    ctx.fill();

    document.getElementById("curAngle").textContent =
        (theta * 180 / Math.PI).toFixed(1) + "°";

    document.getElementById("simTime").textContent =
        time.toFixed(2) + " s";

    requestAnimationFrame(frame);
}

requestAnimationFrame(frame);
</script>

</div>
