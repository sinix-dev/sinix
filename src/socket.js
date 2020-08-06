// Create WebSocket connection.
import axios from "axios"

const params = new URLSearchParams()
params.append("username", "sanket143")

axios.post("http://localhost:41431/register", params).then((res) => {
  const socket = new WebSocket("ws://localhost:41431/ws/sanket143")

  socket.onopen = () => {
    console.log("connected")
  }
}).catch((err) => {
  console.log(err)
})

