const ws = new WebSocket("ws://lisa.com:4143/subscriptions", "graphql-ws")

ws.onopen = function () {
  console.log("Socket Open")
  ws.send(`{"type":"connection_init"}`)
  ws.send('{"type": "start", "payload":{"query":"subscription { command { type value }}"}}')
};

ws.onmessage = function (e) {
  let response = JSON.parse(e.data)
  if (response.type == "error") {
    console.log(data.payload.message)
  }
  else if (response.type == "data") {
    PLAYER_COMMANDS.push(response.payload.data.command)
  }
}

const PLAYER_COMMANDS = []

const DIRECTION_MAP = [
  { Y: 1, X: 0 }, // DOWN
  { Y: 0, X: -1 }, // LEFT
  { Y: -1, X: 0 }, // UP
  { Y: 0, X: 1 }, // RIGHT
]

const WORLD_MAP = {
  width: 18,
  height: 10,
  tile_width: Math.floor(window.screen.width / 18),
  tile_height: Math.floor(window.screen.height / 10),
  tiles: [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0, 0,
    0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 1, 0, 1, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0,
    0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0,
    0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
  ],
  graphics: []
}

class Keyboard {
  constructor() {
    this.pressed = {}
    this.latest = undefined
  }

  watch(el) {
    el.addEventListener('keydown', (e) => {
      this.pressed[e.key.toLowerCase()] = true
      this.latest = e.key.toLowerCase()
    })
    el.addEventListener('keyup', (e) => {
      this.pressed[e.key.toLowerCase()] = false
    })
  }
}

class Player {
  size = 50
  speed = 12
  color = 'crimson'
  direction = 2

  constructor() {
    this.position = {
      x: 500,
      y: 500
    }
  }

  render() {
    this.handler()
    this.updatePosition()

    canvas.context.fillStyle = this.color
    canvas.context.fillRect(this.position.x, this.position.y, this.size, this.size)
  }

  updatePosition() {
    var new_x = this.position.x + DIRECTION_MAP[this.direction].X * this.speed
    var new_y = this.position.y + DIRECTION_MAP[this.direction].Y * this.speed

    if (new_x < 0) {
      new_x = 0
    } else if (new_x > canvas.element.width - this.size) {
      new_x = canvas.element.width - this.size
    }

    if (new_y < 0) {
      new_y = 0
    } else if (new_y > canvas.element.height - this.size) {
      new_y = canvas.element.height - this.size
    }

    var tile_1, tile_2
    if (this.direction == 0) {
      // DOWN
      tile_1 = Math.floor((new_y + this.size) / WORLD_MAP.tile_height) * WORLD_MAP.width + Math.floor(new_x / WORLD_MAP.tile_width)
      tile_2 = Math.floor((new_y + this.size) / WORLD_MAP.tile_height) * WORLD_MAP.width + Math.floor((new_x + this.size - 2) / WORLD_MAP.tile_width)

      if (WORLD_MAP.tiles[tile_1] || WORLD_MAP.tiles[tile_2]) {
        new_y = Math.floor(tile_1 / WORLD_MAP.width) * WORLD_MAP.tile_height - this.size
      }
    } else if (this.direction == 1) {
      // LEFT
      tile_1 = Math.floor((new_y) / WORLD_MAP.tile_height) * WORLD_MAP.width + Math.floor(new_x / WORLD_MAP.tile_width)
      tile_2 = Math.floor((new_y + this.size - 2) / WORLD_MAP.tile_height) * WORLD_MAP.width + Math.floor((new_x) / WORLD_MAP.tile_width)

      if (WORLD_MAP.tiles[tile_1] || WORLD_MAP.tiles[tile_2]) {
        new_x = (tile_1 % WORLD_MAP.width) * WORLD_MAP.tile_width + WORLD_MAP.tile_width
      }
    } else if (this.direction == 2) {
      // UP
      tile_1 = Math.floor((new_y) / WORLD_MAP.tile_height) * WORLD_MAP.width + Math.floor(new_x / WORLD_MAP.tile_width)
      tile_2 = Math.floor((new_y) / WORLD_MAP.tile_height) * WORLD_MAP.width + Math.floor((new_x + this.size - 2) / WORLD_MAP.tile_width)

      if (WORLD_MAP.tiles[tile_1] || WORLD_MAP.tiles[tile_2]) {
        new_y = Math.floor(tile_1 / WORLD_MAP.width) * WORLD_MAP.tile_height + WORLD_MAP.tile_height
      }
    } else if (this.direction == 3) {
      // RIGHT
      tile_1 = Math.floor((new_y) / WORLD_MAP.tile_height) * WORLD_MAP.width + Math.floor((new_x + this.size) / WORLD_MAP.tile_width)
      tile_2 = Math.floor((new_y + this.size - 2) / WORLD_MAP.tile_height) * WORLD_MAP.width + Math.floor((new_x + this.size) / WORLD_MAP.tile_width)

      if (WORLD_MAP.tiles[tile_1] || WORLD_MAP.tiles[tile_2]) {
        new_x = (tile_1 % WORLD_MAP.width) * WORLD_MAP.tile_width - this.size
      }
    }
    this.position.x = new_x
    this.position.y = new_y
  }

  turn(_direction) {

    this.direction = (this.direction + _direction + 4) % 4
  }

  handler() {
    this.keypressHandler()
    this.commandHandler()
  }

  keypressHandler() {

    if (keyboard.latest == 'f') {
      this.turn(-1)
    } else if (keyboard.latest == 'j') {
      this.turn(1)
    }

    keyboard.latest = undefined
  }

  commandHandler() {

    if (PLAYER_COMMANDS.length > 0) {
      let command = PLAYER_COMMANDS.pop()
      if (command.type == "turn") {
        let turnMap = {
          "left": -1,
          "right": 1
        }

        this.turn(turnMap[command.value])
      }
    }
  }
}

class Canvas {
  constructor(_id) {
    this.element = document.getElementById(_id)
    this.context = this.element.getContext('2d')

    this.element.width = window.innerWidth
    this.element.height = window.innerHeight
  }
}

function drawGame() {

  canvas.context.fillStyle = 'black';
  canvas.context.fillRect(0, 0, canvas.element.width, canvas.element.height);
  canvas.context.fillStyle = 'white'

  for (var i = 0; i < WORLD_MAP.graphics.length; i++) {
    canvas.context.fillRect(WORLD_MAP.graphics[i].x, WORLD_MAP.graphics[i].y, WORLD_MAP.tile_width, WORLD_MAP.tile_height)
  }

  player.render()
  window.requestAnimationFrame(drawGame);
}

const canvas = new Canvas('gameCanvas')
const player = new Player()
const keyboard = new Keyboard()

while (true) {
  extra_tile = Math.floor(Math.random() * 100) % WORLD_MAP.tiles.length
  if (WORLD_MAP.tiles[extra_tile] == 0) {
    WORLD_MAP.tiles[extra_tile] = 1
    break;
  }
}

for (var i = 0; i < WORLD_MAP.tiles.length; i++) {
  if (WORLD_MAP.tiles[i] == 1) {
    x = (i % WORLD_MAP.width) * WORLD_MAP.tile_width
    y = Math.floor(i / WORLD_MAP.width) * WORLD_MAP.tile_height
    WORLD_MAP.graphics.push({
      x: x,
      y: y,
    })
  }
}

keyboard.watch(document)
window.requestAnimationFrame(drawGame);

window.addEventListener('resize', function(){
  canvas.element.width = window.innerWidth
  canvas.element.height = window.innerHeight
})