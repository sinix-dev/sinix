const path = require('path')
const express = require('express');
const { GraphQLServer } = require('graphql-yoga')

const typeDefs = require('./src/typedefs')
const resolvers = require('./src/resolvers')

const server = new GraphQLServer({
  typeDefs,
  resolvers
})

const options = {
  port: 4143,
  endpoint: '/graphql',
  playground: '/playground',
  subscriptions: '/subscriptions',
}

server.express.use("/", express.static(path.join(__dirname, 'public')))
server.start(options, () => console.log('Server started.'))