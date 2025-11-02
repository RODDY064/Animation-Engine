let animCount = 0;
let lastFrameTime = 0;

async function initBig() {
  try {
    const { default: init, Animation } = await import( "./pkg/animation_engine.js");
    await init();

    console.log("it loaded")

    const box = document.querySelector(".box");
    let animationStart = 0;

    function animate(config) {
      box.style.transform = "translate3d(0px, 0px, 0px)";
      setTimeout(() => {
        animationStart = performance.now();
        let anim = new Animation(box);

        anim = anim.cubic(config.x1,config.y1, config.x2,config.y2,config.duration);

        anim = anim.to(null, 200, null, null, null, null, null, null);
        anim.start();
      }, 50);
    }

    animate({
      type: "cubic",
      x1: 0.0,
      y1: 0.0,
      x2: 0.58,
      y2: 1.0,
      duration: 400,
    });
  } catch {
    console.error("Failed to initialize WASM:", err);
  }
}


initBig()
