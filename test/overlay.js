const { init } = require('../index.js')

const client = init(480)

/*
This node script is "headless." It has no graphical context into which Steam can inject an overlay.

Therefore, when you run this script while the Steam app is running, Steam will foreground the Steam app, and display example.com in the Steam app's window.
*/
client.overlay.activateToWebPage('https://www.example.com/');