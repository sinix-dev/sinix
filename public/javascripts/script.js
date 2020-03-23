// Listen for animate update
document.onkeydown = (eve) => {
  if (eve.key == "a") {
    direction = "left"
  }
  else if (eve.key == "s") {
    direction = "down"
  }
  else if (eve.key == "d") {
    direction = "right"
  }
  else if (eve.key == "w") {
    direction = "up"
  }
}

const app = new PIXI.Application({ antialias: true });
document.body.appendChild(app.view);

const graphics = new PIXI.Graphics();

// let's create a moving shape
const thing = new PIXI.Graphics();
app.stage.addChild(thing);
thing.x = 0;
thing.y = 0;

let right = 0;
let down = 0;
let up = 0;
let left = 0;
let size = 30;
let direction = "right"


app.ticker.add((delta) => {

  if (direction == "down") {
    down += 2
  }
  else if (direction == "right") {
    right += 2
  }
  else if (direction == "left") {
    right -= 2
  }
  else if (direction == "up") {
    down -= 2
  }

  if (app.renderer.screen.width - size <= right) {
    right = app.renderer.screen.width - size
  }
  else if (right <= 0) {
    right = 0
  }

  if (app.renderer.screen.height - size <= down) {
    down = app.renderer.screen.height - size
  }
  else if (down <= 0) {
    down = 0
  }

  thing.clear();
  thing.beginFill(0xDE3249);
  thing.drawRect(right, down, size, size);
  thing.endFill();
});


// Listen for window resize events
window.addEventListener('resize', resize);

// Resize function window
function resize() {
	// Resize the renderer
	app.renderer.resize(window.innerWidth, window.innerHeight);
  
  // You can use the 'screen' property as the renderer visible
  // area, this is more useful than view.width/height because
  // it handles resolution
  rect.position.set(app.screen.width, app.screen.height);
}

resize();