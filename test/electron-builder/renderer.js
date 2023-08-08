/** @type {import('steamworks.js')} */
const steamworks = require('steamworks.js');
const client = steamworks.init(480);

const playerName = client.localplayer.getName()
document.getElementById('name').innerText = playerName

document.getElementById('activateOverlay').addEventListener('click', function() {
    client.overlay.activateToWebPage('https://www.example.com/')
})
