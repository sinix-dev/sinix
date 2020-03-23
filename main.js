const path = require('path')
const express = require('express');
const { GraphQLServer, PubSub } = require('graphql-yoga')

const Mutation = require('./src/mutation')
const Subscription = require('./src/subscription')

const resolvers = {
 Mutation,
 Subscription
}


const pubsub = new PubSub()

const server = new GraphQLServer({
  typeDefs: './schema.graphql',
  resolvers,
  context: { pubsub }
})

const options = {
  port: 4143,
  endpoint: '/graphql',
  playground: '/playground',
  subscriptions: '/subscriptions',
}

server.express.use("/", express.static(path.join(__dirname, 'public')))
server.start(options, () => console.log('Server started.'))