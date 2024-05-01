---
title: Search
authorbox: false
sidebar: false
menu: main
---

<link href="/pagefind/pagefind-ui.css" rel="stylesheet">
<script src="/pagefind/pagefind-ui.js"></script>
<div id="search"></div>
<script>
    window.addEventListener('DOMContentLoaded', (event) => {
        new PagefindUI({
            element: "#search",
            showSubResults: true,
            showImages: true,
            processResult: function (result) {
                if (result?.meta?.image) {
                    let resultBase = new URL(result.url, window.location);
                    let remappedImage = new URL(result.meta.image, resultBase);
                    if (remappedImage.hostname !== window.location.hostname) {
                        result.meta.image = remappedImage.toString();
                    } else {
                        result.meta.image = remappedImage.pathname;
                    }
                }
            }
        });    
    });
</script>
