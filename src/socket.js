import axios from "axios"

const params = new URLSearchParams()
params.append("username", "sinix")

export default (player) => {
  axios.post("http://localhost:41431/register", params).then((res) => {
    const socket = new WebSocket("ws://localhost:41431/ws/sinix")

    socket.onopen = () => {
      console.log("connected")
    }

    socket.onmessage = (resp) => {
      let { data } = resp
      data = JSON.parse(data)

      if(data.event_type === "STICK1"){
        data.payload.x *= 1/60 * 0.003
        data.payload.y *= 1/60 * 0.003

        player.force = data.payload
      } else if (data.event_type === "BUTTON"){

        if(data.payload.val === "A"){
          player.torque = -0.5
        } else if(data.payload.val === "B"){
          player.torque = 0.5
        }
      }
    }
  }).catch((err) => {
    console.log(err)
  })
}
