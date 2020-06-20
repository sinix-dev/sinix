const express = require('express')
const router = express.Router();

router.post("/command", (req, res) => {
  req.io.emit("message", JSON.parse(req.body.data))
  res.send("bleh")
})

module.exports = router
