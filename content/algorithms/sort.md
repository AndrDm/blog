---
title: Visualizing Sorting Algorithms in Real Time
date: 2026-07-03
description: Explore how Bubble Sort, Selection Sort, and Insertion Sort work through an interactive live visualization with adjustable speed and operation statistics.
archives:
  - 2026-07
draft: false
---

Sorting algorithms are one of the fundamental building blocks of computer science. While their source code is often only a few lines long, understanding how elements move through a collection can be much easier when the process is visualized.

<!--more-->

This interactive demonstration allows you to watch several classic sorting algorithms operate on a randomized dataset in real time. Use the controls to choose an algorithm, adjust the animation speed, generate a new random sequence, and observe how the algorithm transforms an unsorted array into an ordered one.

During execution, the visualization highlights active comparisons and tracks the number of comparisons and swaps performed. These metrics help illustrate why different algorithms exhibit different performance characteristics and computational complexity.

Available algorithms:

- **Bubble Sort** — repeatedly compares adjacent elements and swaps them when they are out of order.
- **Selection Sort** — repeatedly finds the smallest remaining element and places it in its final position.
- **Insertion Sort** — incrementally builds a sorted section by inserting each new element into the correct location.

Try different speeds, randomize the dataset, and compare how each algorithm behaves as it processes the same input.

<div id="sorting-demo">

<style>
#sorting-demo {
    background: #0d1117;
    color: #e6edf3;
    padding-bottom: 20px;
    font-family: Poppins, system-ui, Segoe UI, sans-serif;
}
#sorting-demo .top {
    display: flex;
    gap: 14px;
    align-items: center;
    padding: 18px 26px;
    background: #161b22;
    flex-wrap: wrap;
}
#sorting-demo h2 {
    margin: 0 18px 0 0;
    font-size: 22px;
    color: #7ee787;
}
#sorting-demo select,
#sorting-demo button {
    font-size: 16px;
    padding: 10px 16px;
    border-radius: 10px;
    border: 1px solid #30363d;
    background: #21262d;
    color: #e6edf3;
    cursor: pointer;
}
#sorting-demo button.go {
    background: #238636;
    border-color: #238636;
    font-weight: 700;
}
#sorting-demo .stat {
    margin-left: auto;
    color: #9fb0c8;
}
#sorting-demo .stat b {
    color: #7ee787;
}
#sorting-demo .bars {
    display: flex;
    align-items: flex-end;
    gap: 3px;
    height: 400px;
    padding: 24px;
    box-sizing: border-box;
}
#sorting-demo .bar {
    flex: 1;
    background: linear-gradient(#3b82f6, #1d4ed8);
    border-radius: 4px 4px 0 0;
    transition: height 0.05s;
}
#sorting-demo .bar.cmp {
    background: linear-gradient(#fbbf24, #d97706);
}
#sorting-demo .bar.swp {
    background: linear-gradient(#f87171, #dc2626);
}
#sorting-demo .bar.done {
    background: linear-gradient(#34d399, #059669);
}
</style>

<!-- No Empty Lines in div section! -->
<div class="top">
    <h2>Sort Algorithms</h2>
    <select id="alg">
        <option value="bubble">Bubble Sort</option>
        <option value="selection">Selection Sort</option>
        <option value="insertion">Insertion Sort</option>
    </select>
    <label style="font-size:15px;color:#9fb0c8">
        Speed
        <input type="range" id="spd" min="1" max="100" value="55" style="vertical-align:middle">
    </label>
    <button id="shuf">🔀 Randomize</button>
    <button class="go" id="run">▶ Sort It!</button>
    <div class="stat">
        Comparisons: <b id="cmp">0</b> · Swaps: <b id="swp">0</b>
    </div>
</div>

<div class="bars" id="bars"></div>

<script>
console.log("sorting demo loaded");

var N = 40;
var arr = [];
var bars = document.getElementById("bars");
var els = [];
var cmp = 0;
var swp = 0;
var running = false;

function build() {
    bars.innerHTML = "";
    els = [];
    for (var i = 0; i < N; i++) {
        var d = document.createElement("div");
        d.className = "bar";
        bars.appendChild(d);
        els.push(d);
    }
}

function paint() {
    for (var i = 0; i < N; i++) {
        els[i].style.height = arr[i] + "%";
    }
}

function updateStats() {
    document.getElementById("cmp").textContent = cmp;
    document.getElementById("swp").textContent = swp;
}

function shuffle() {
    arr = [];
    for (var i = 0; i < N; i++) {
        arr.push(5 + Math.round(Math.random() * 95));
    }
    cmp = 0;
    swp = 0;
    updateStats();
    paint();
}

function delay() {
    var speed = document.getElementById("spd").value;
    return new Promise(function(resolve) {
        setTimeout(resolve, Math.max(2, 105 - speed));
    });
}

async function bubble() {
    for (var i = 0; i < N - 1; i++) {
        for (var j = 0; j < N - 1 - i; j++) {
            cmp++;
            updateStats();
            els[j].className = "bar cmp";
            els[j + 1].className = "bar cmp";
            await delay();
            if (arr[j] > arr[j + 1]) {
                var t = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = t;
                swp++;
                updateStats();
                paint();
            }
            els[j].className = "bar";
            els[j + 1].className = "bar";
        }
        els[N - 1 - i].className = "bar done";
    }
    els[0].className = "bar done";
}

async function selection() {
    for (var i = 0; i < N - 1; i++) {
        var min = i;
        els[i].className = "bar cmp";
        for (var j = i + 1; j < N; j++) {
            els[j].className = "bar cmp";
            cmp++;
            updateStats();
            await delay();
            if (arr[j] < arr[min]) min = j;
            if (j !== min) els[j].className = "bar";
        }
        if (min !== i) {
            var t = arr[i];
            arr[i] = arr[min];
            arr[min] = t;
            swp++;
            updateStats();
            paint();
        }
        els[i].className = "bar done";
    }
    els[N - 1].className = "bar done";
}

async function insertion() {
    els[0].className = "bar done";
    for (var i = 1; i < N; i++) {
        var key = arr[i];
        var j = i - 1;
        els[i].className = "bar swp";
        await delay();
        while (j >= 0 && arr[j] > key) {
            cmp++;
            updateStats();
            els[j].className = "bar cmp";
            arr[j + 1] = arr[j];
            paint();
            await delay();
            els[j].className = "bar";
            j--;
        }
        arr[j + 1] = key;
        swp++;
        updateStats();
        paint();
        for (var k = 0; k <= i; k++) els[k].className = "bar done";
    }
}

async function run() {
    if (running) return;

    running = true;
    document.getElementById("run").disabled = true;
    document.getElementById("shuf").disabled = true;

    try {
        var a = document.getElementById("alg").value;

        if (a === "bubble") {
            await bubble();
        } else if (a === "selection") {
            await selection();
        } else {
            await insertion();
        }

        for (var i = 0; i < N; i++) {
            els[i].className = "bar done";
        }
    } finally {
        running = false;
        document.getElementById("run").disabled = false;
        document.getElementById("shuf").disabled = false;
    }
}

document.getElementById("shuf").onclick = shuffle;
document.getElementById("run").onclick = run;

build();
shuffle();
</script>

</div>
