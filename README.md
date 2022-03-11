[![Build Status](https://github.com/ceifa/steamworks.js/actions/workflows/publish.yml/badge.svg)](https://github.com/ceifa/steamworks.js/actions/workflows/publish.yml)
[![npm](https://img.shields.io/npm/v/steamworks.js.svg)](https://npmjs.com/package/steamworks.js)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Chat](https://img.shields.io/discord/663831597690257431?label=chat&logo=discord)](https://discord.gg/H6B7UE7fMY)

# Steamworks.js

A modern implementation of the Steamworks SDK for HTML/JS based applications.

| Feature | Supported |
|----------|------------ |
| Windows | ✔ |
| Linux | ✔ |
| MacOS | ✔ |
| Electron 12+ | ✔ |
| NW.js 0.29+ | ✔ |
| Node.js 14+ | ✔ |
| Pre-built binaries | ✔ |
| Easy to install | ✔ |
| Open Source | ✔ |
| MIT license | ✔ |

## Why

I used [greenworks](https://github.com/greenheartgames/greenworks) for a long time and it's great, but I gave up for the following reasons.

* It's not being maintained anymore.
* It's not up to date.
* You have to build the binaries by yourself.
* Don't have typescript definitions.
* The API it's not trustful.
* The API implement callbacks instead of return flags or promises.
* I hate C++.

## API

```js
const steamworks = require('steamworks.js')

// You can pass the appId or nothing if you want to use the steam_appid.txt file
const client = steamworks.init()

console.log(client.getName()) // Print user name
// Activate activateAchievement
if (client.activateAchievement('ACHIEVEMENT')) {
    // ...
}
```

## Electron instructions

Because steamworks.js is written using some Node.JS features, it should be initialized on [preload](https://www.electronjs.org/pt/docs/latest/tutorial/context-isolation) and proxied to the renderer:

```js
// preload.js
const steamworks = require('steamworks.js')
const client = steamworks.init()

contextBridge.exposeInMainWorld('steamworksClient', client)
```

```js
// renderer.js
console.log(window.steamworksClient.getName())
```

You have to enable some flags on chromium to make the steam overlay work, put this code on the final of `main.js`:

```js
app.commandLine.appendSwitch('in-process-gpu')
app.commandLine.appendSwitch('disable-direct-composition')
```