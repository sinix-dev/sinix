module.exports = {
  Query: {
    hello: (_, { name }) => `Hello ${name || 'World'}`,
  },
}