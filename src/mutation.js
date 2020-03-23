function sendCommand(parent, args, { pubsub }) {
  pubsub.publish('COMMAND_CHANNEL', { command: args })
  return args
}

module.exports = {
  sendCommand
}