module.exports = (socket) => {
  socket.on("message", (msg) => {
    console.log(msg)
    socket.emit("message", msg)
  })

  console.log("A user connected")
}
