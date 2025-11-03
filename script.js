let animCount = 0;
let lastFrameTime = 0;

async function initBig() {
  try {
    const {
      default: init,
      Animation,
      GestureController,
    } = await import("./pkg/animation_engine.js");
    await init();

    console.log("it loaded");

    const box = document.querySelector(".box");
    const button = document.querySelector(".box2");
    const gesture = new GestureController();

    button.addEventListener("mousedown", () => {
      const scale = gesture.onPress(true); 
      new Animation(button)
      .spring_smooth(2)
      .animate({ scale})
      .start()
  
    });

    function animate() {
      box.style.transform = "translate3d(0px, 0px, 0px)";
      let anim = new Animation(box)
        .spring_smooth(Infinity)
        .set_delay(1000)
        .animate({ rotate: 360 })
        .start();
    }

    animate();
  } catch {
    console.error("Failed to initialize WASM:", err);
  }
}

initBig();
