function commandSubscribe(parent, args, { pubsub }) {
  return pubsub.asyncIterator('COMMAND_CHANNEL')
}

const command = {
  subscribe: commandSubscribe
}

module.exports = {
  command,
}