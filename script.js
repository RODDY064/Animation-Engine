let animCount = 0;
let lastFrameTime = 0;

async function initBig() {
  try {
    const { default: init, Animation } = await import( "./pkg/animation_engine.js");
    await init();

    console.log("it loaded")

    const box = document.querySelector(".box");
    let animationStart = 0;

    function animate() {
      box.style.transform = "translate3d(0px, 0px, 0px)";
       let anim = new Animation(box)
        .spring_smooth(400)
        .with_velocity('y',25.0)
        .set_delay(1000)
        .animate({ x:300 })
        .start();

    }

    animate();
  } catch {
    console.error("Failed to initialize WASM:", err);
  }
}


initBig()
