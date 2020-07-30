const express = require('express')
const router = express.Router()
const colors = [
  "800000",
  "806A00",
  "2A8000",
  "008040",
  "005580",
  "150080",
  "800080",
  "800015"
]

const MAX_USER = 2
const users = []

router.post("/command", (req, res) => {
  req.io.emit("message", JSON.parse(req.body.data))
  res.send("bleh")
})

router.post("/connect", (req, res) => {
  const date = Date.now().valueOf()
  const obj = {
    id: date,
    color: colors[date % colors.length]
  }

  if(users.length < 2){
    users.unshift(obj)
  }
  
  req.io.emit("new", obj)
  res.json(obj)
})
module.exports = router
