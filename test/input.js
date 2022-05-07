const { init } = require('../index.js')

const client = init(1694450)
client.input.init()

const actionset = client.input.getActionSet('GameControls')

const affirm = client.input.getDigitalAction('Affirm')
const cancel = client.input.getDigitalAction('Cancel')
const control = client.input.getAnalogAction('Control')

setInterval(() => {
    console.clear()

    const controllers = client.input.getControllers()
    console.log('Controllers: ' + controllers.length)

    controllers.forEach(controller => {
        controller.activateActionSet(actionset)

        console.log('============')
        console.log('Affirm: ' + controller.isDigitalActionPressed(affirm))
        console.log('Cancel: ' + controller.isDigitalActionPressed(cancel))
        console.log('Control: ' + JSON.stringify(controller.getAnalogActionVector(control)))
    })
}, 66)