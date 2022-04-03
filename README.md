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
* It's not context-aware
* You have to build the binaries by yourself.
* Don't have typescript definitions.
* The API it's not trustful.
* The API implement callbacks instead of return flags or promises.
* I hate C++.

## Installation

We need to first build the application. So make sure you have the latest [node.js](https://nodejs.org/en/), [Rust](https://www.rust-lang.org/tools/install) and [Clang](https://rust-lang.github.io/rust-bindgen/requirements.html). We also need [Steam](https://store.steampowered.com/about/) installed and running.

Install dependencies with `npm install` and then run `npm run build:debug` to build the application.

## Usage

With the Steam application running, test it by running `node test`.

```
const { init } = require('./index.js')
const client = init(480)
console.log(client.getName())
```

It's set to run with a test app id: 480. Search for your corresponding app id [here](https://steamdb.info/apps/).

## API

```js
const steamworks = require('steamworks.js')

// You can pass the appId or nothing if you want to use the steam_appid.txt file
const client = steamworks.init()

console.log(client.getName()) // Print user name
// Tries to activate an achievement
if (client.activateAchievement('ACHIEVEMENT')) {
    // ...
}
```

## Electron instructions

Steamworks.js it's a native module and cannot be used by default in the renderer process. To enable the usage of native modules on the renderer process, the following configurations should be made on `main.js`:

```js
const mainWindow = new BrowserWindow({
    // ...
    webPreferences: {
        // ...
        contextIsolation: false,
        nodeIntegration: true
    }
})
```

You also have to enable some flags on chromium to make the steam overlay work. Put this code on the final of `main.js`:

```js
app.commandLine.appendSwitch('in-process-gpu')
app.commandLine.appendSwitch('disable-direct-composition')
```