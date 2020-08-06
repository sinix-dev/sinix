import {
  Engine,
  Render,
  World,
  Events,
  Bodies,
  Body
} from "matter-js"
import "./socket.js"


const width = 1000
const height = 600
const engine = Engine.create()
const render = Render.create({
  element: document.body,
  engine: engine,
  options: {
    width,
    height,
    pixelRatio: 1,
    background: '#222',
    wireframeBackground: '#222',
    enabled: true,
    wireframes: false,
    showVelocity: false,
    showAngleIndicator: false,
    showCollisions: true
  }
})

const strike_audio = new Audio("assets/sounds/strike-01.mp3")
const soundtrack = new Audio("assets/sounds/Eleven-Petals.mp3")
const offset = 5
const keys = []

let player, opponent

const setup = () => {
  engine.world.gravity.y = 0
  soundtrack.loop = true
  soundtrack.play()

  World.add(engine.world, [
    Bodies.circle(width / 2, height / 2, 50, {
      isStatic: true,
      angle: -Math.PI * 0.1
    })
  ])

  World.add(engine.world, [
    Bodies.rectangle(width / 2, -offset, width + 2 * offset, 50, {
      isStatic: true
    }),
    Bodies.rectangle(width / 2, height + offset, width + 2 * offset, 50, {
      isStatic: true
    }),
    Bodies.rectangle(width + offset, 300, 50, height + 2 * offset, {
      isStatic: true
    }),
    Bodies.rectangle(-offset, 300, 50, width + 2 * offset, {
      isStatic: true
    })
  ])

  const opponent_body = Bodies.polygon(400, 200, 5, 25, {
    friction: 1,
    render: {
      fillStyle: "crimson"
    },
    label: "Bey"
  });

  const opponent_outline = Bodies.polygon(400, 200, 5, 20, {
    friction: 1,
    render: {
      fillStyle: "#000000"
    }
  });

  opponent = Body.create({
    parts: [opponent_body, opponent_outline]
  })

  World.add(engine.world, opponent);

  const player_body = Bodies.polygon(100, 100, 5, 25, {
    friction: 1,
    render: {
      fillStyle: "cyan"
    },
    label: "Bey"
  });

  const player_outline = Bodies.polygon(100, 100, 5, 20, {
    friction: 1,
    render: {
      fillStyle: "navy"
    }
  })

  player = Body.create({
    parts: [player_body, player_outline]
  })

  World.add(engine.world, player)

  /*
   * Keyboard Configurations
   */

  document.body.addEventListener("keydown", function(e) {
    keys[e.keyCode] = true
  })

  document.body.addEventListener("keyup", function(e) {
    keys[e.keyCode] = false
  })

  Engine.run(engine)
  Render.run(render)
}

const collisionListeners = () => {
  const force = 0.002

  Events.on(engine, "beforeTick", function(event) {
    if(keys[87]){
      player.force = { x: 0, y: -force }
    }
    if(keys[83]){
      player.force = { x: 0, y: force }
    }
    if(keys[68]){
      player.force = { x: force, y: 0 }
    }
    if(keys[65]){
      player.force = { x: -force, y: 0 }
    }

    if (keys[37] && player.angularVelocity > -0.55) {
      player.torque = -0.1
      opponent.torque = -0.1
    } else {
      if (keys[39] && player.angularVelocity < 0.55) {
        player.torque = 0.1
        opponent.torque = 0.1
      }
    }
  })

  Events.on(engine, "collisionStart", function(event) {
    const { bodyA, bodyB } = event.pairs[0]

    if(bodyA.label === "Bey" && bodyB.label === "Bey"){
      strike_audio.currentTime = 0
      strike_audio.play()
    }
  })
}

setup()
collisionListeners()
