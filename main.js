const path = require('path')
const express = require('express');
const { app, BrowserWindow } = require('electron')
const router = express()
const http = require("http").createServer(router)
const io = require("socket.io")(http)
const socket = require("./src/socket")

const PORT = 41431

router.use(express.static(path.join(__dirname, 'public')))

let mainWindow

app.on('ready', () => {
  mainWindow = new BrowserWindow({
    webPreferences: {
      nodeIntegration: true
    },
    fullscreen: true
  })

  http.listen(PORT, () => {
    mainWindow.loadURL('http://localhost:41431')
  })

  io.on("connection", socket)

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
