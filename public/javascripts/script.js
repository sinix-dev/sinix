const DIRECTION_MAP = [
  { Y: 1, X: 0 }, // DOWN
  { Y: 0, X: -1 }, // LEFT
  { Y: -1, X: 0 }, // UP
  { Y: 0, X: 1 }, // RIGHT
]

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
  size = 30
  speed = 3
  color = 0xDC143C
  direction = 0

  constructor() {

    this.pixi = new PIXI.Graphics()
    this.position = {
      x: 100,
      y: 100
    }
  }

  render() {
    this.handler()
    this.updatePosition()

    this.pixi.clear()
    this.pixi.beginFill(this.color)
    this.pixi.drawRect(this.position.x, this.position.y, this.size, this.size)
    this.pixi.endFill()
  }

  updatePosition() {
    var new_x = this.position.x + DIRECTION_MAP[this.direction].X * this.speed
    var new_y = this.position.y + DIRECTION_MAP[this.direction].Y * this.speed

    if (new_x < 0) {
      new_x = 0
    } else if (new_x > app.screen.width - this.size) {
      new_x = app.renderer.screen.width - this.size
    }

    if (new_y < 0) {
      new_y = 0
    } else if (new_y > app.screen.height - this.size) {
      new_y = app.renderer.screen.height - this.size
    }

    this.position.x = new_x
    this.position.y = new_y
  }

  turn(_direction) {

    this.direction = (this.direction + _direction + 4) % 4
  }

  handler() {
    if (keyboard.latest == 'f') {
      this.turn(-1)
    } else if (keyboard.latest == 'j') {
      this.turn(1)
    }

    keyboard.latest = undefined
  }
}

const keyboard = new Keyboard()
const app = new PIXI.Application({ antialias: true });

keyboard.watch(document)
document.body.appendChild(app.view)

const player = new Player()
app.stage.addChild(player.pixi)

app.ticker.add(() => {
  player.render()
});


// Listen for window resize events
window.addEventListener('resize', resize)

// Resize function window
function resize() {
  // Resize the renderer
  app.renderer.resize(window.innerWidth, window.innerHeight)
}

resize()