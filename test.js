const { init, runCallbacks } = require('./index.js')

init(480)
setInterval(runCallbacks, 50)