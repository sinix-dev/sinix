const path = require('path')
const express = require('express');
const { GraphQLServer, PubSub } = require('graphql-yoga')
const { app, BrowserWindow } = require('electron')

const Query = require('./src/query')
const Mutation = require('./src/mutation')
const Subscription = require('./src/subscription')

const resolvers = {
  Query,
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

let server_pr = server.start(options)
let mainWindow

app.on('ready', () => {
  mainWindow = new BrowserWindow({
    webPreferences: {
      nodeIntegration: true
    },
    fullscreen: true
  })

  server_pr.then(() => {
    mainWindow.loadURL('http://localhost:4143')
  })

  mainWindow.on('closed', function () {
    mainWindow = null
  })
})

app.on('window-all-closed', function () {
  // On macOS it is common for applications and their menu bar
  // to stay active until the user quits explicitly with Cmd + Q
  if (process.platform !== 'darwin') app.quit()
})

app.on('activate', function () {
  // On macOS it's common to re-create a window in the app when the
  // dock icon is clicked and there are no other windows open.
  if (mainWindow === null) createWindow()
})