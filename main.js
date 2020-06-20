const path = require('path')
const express = require('express');
const bodyParser = require('body-parser')
const { app, BrowserWindow } = require('electron')
const router = express()
const http = require("http").createServer(router)
const io = require("socket.io")(http)
const socket = require("./src/socket")
const routes = require("./src/routes")

const PORT = 41431

router.use(bodyParser.urlencoded({ extended: false }))
router.use(bodyParser.json())

router.use(express.static(path.join(__dirname, 'public')))
router.use((req, res, next) => {
  req.io = io
  next()
})

router.use("/", routes)

let mainWindow

app.on('ready', () => {
  mainWindow = new BrowserWindow({
    webPreferences: {
      nodeIntegration: true
    },
    fullscreen: true
  })

  mainWindow.on('closed', function () {
    mainWindow = null
  })

  http.listen(PORT, () => {
    mainWindow.loadURL('http://localhost:41431')
  })

  io.on("connection", socket)
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
