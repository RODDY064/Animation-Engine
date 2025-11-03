let currentIndex = 0;
let animationEngine = null;
let autoResetTimer = null;
const AUTO_RESET_DELAY = 5000;
let isAnimating = false;
let thumbPosition = 0;
let thumbWidth = 0;
let sliderContainer = null;

async function initAnimation() {
  try {
    const { default: init, Animation } = await import(
      "./pkg/animation_engine.js"
    );
    await init();

    animationEngine = Animation;
    console.log("Animation engine loaded");

    // Wait for DOM to be fully ready
    if (document.readyState === 'loading') {
      document.addEventListener('DOMContentLoaded', () => {
        preloadImages();
        setupSliderAnimation();
      });
    } else {
      preloadImages();
      setupSliderAnimation();
    }
    
  } catch (err) {
    console.error("Failed to initialize WASM:", err);
  }
}

function preloadImages() {
  const pictures = document.querySelectorAll(".image-wrapper picture");
  pictures.forEach((picture) => {
    const img = picture.querySelector("img");
    if (img && img.src) {
      const tempImg = new Image();
      tempImg.src = img.src;
      console.log("Preloading:", img.src);
    }
  });
}

function setupSliderAnimation() {
  const listItems = document.querySelectorAll(".terms-words li");
  const sliderThumb = document.querySelector(".slider-thumb");
  const sliderCon = document.querySelector(".slider-con");
  const pictures = document.querySelectorAll(".image-wrapper picture");

  sliderContainer = sliderCon;

  // Give DOM time to calculate sizes
  setTimeout(() => {
    // Initial setup - set thumb width to first item and show first picture
    updateSliderThumb(0, sliderThumb, listItems, sliderCon);
    pictures[0].style.opacity = "1";

    console.log("Setup complete with", listItems.length, "items");
    console.log("Initial thumb position:", thumbPosition, "width:", thumbWidth);

    // Add click listeners to each list item
    listItems.forEach((item, index) => {
      item.addEventListener("click", (e) => {
        e.preventDefault();
        e.stopPropagation();
        
        console.log("Clicked item", index, "current:", currentIndex);
        
        // If clicking the same item, do nothing
        if (currentIndex === index) {
          console.log("Already on index", index, "- ignoring click");
          return;
        }
        
        // If animation is in progress, ignore clicks
        if (isAnimating) {
          console.log("Animation in progress - ignoring click");
          return;
        }

        clearAutoResetTimer();
        navigateToIndex(index, pictures, sliderThumb, listItems, sliderCon);
        startAutoResetTimer(pictures, sliderThumb, listItems, sliderCon);
      });
    });

    // Keyboard navigation
    document.addEventListener("keydown", (e) => {
      if (isAnimating) return;
      
      if (e.key === "ArrowRight") {
        const nextIndex = (currentIndex + 1) % listItems.length;
        if (nextIndex !== currentIndex) {
          clearAutoResetTimer();
          navigateToIndex(nextIndex, pictures, sliderThumb, listItems, sliderCon);
          startAutoResetTimer(pictures, sliderThumb, listItems, sliderCon);
        }
      } else if (e.key === "ArrowLeft") {
        const prevIndex = (currentIndex - 1 + listItems.length) % listItems.length;
        if (prevIndex !== currentIndex) {
          clearAutoResetTimer();
          navigateToIndex(prevIndex, pictures, sliderThumb, listItems, sliderCon);
          startAutoResetTimer(pictures, sliderThumb, listItems, sliderCon);
        }
      }
    });

    // Start auto-reset timer
    startAutoResetTimer(pictures, sliderThumb, listItems, sliderCon);
  }, 100);
}

function navigateToIndex(index, pictures, sliderThumb, listItems, sliderCon) {
  if (currentIndex === index) {
    console.log("Already at index", index, "- cancelling navigation");
    return;
  }

  const previousIndex = currentIndex;
  currentIndex = index;
  isAnimating = true;
  

  new animationEngine(pictures[previousIndex])
    .cubic(0.25, 0.1, 0.25, 1.0, 1000)
    .animate({ opacity: 0 })
    .set_delay(600)
    .start();

  new animationEngine(pictures[index])
    .cubic(0.42, 0.0, 0.58, 1.0, 600) 
    .animate({ opacity: 1 })
    .on_complete(() => {
      isAnimating = false;
    })
    .start();

  // Animate slider thumb
  animateSliderThumbJS(index, sliderThumb, listItems, sliderCon);
}


function updateSliderThumb(index, sliderThumb, listItems, sliderCon) {
  const selectedItem = listItems[index];
  
  if (!selectedItem) {
    console.error("Selected item not found for index:", index);
    return;
  }

  // Get positions RELATIVE to the slider container
  const selectedRect = selectedItem.getBoundingClientRect();
  const containerRect = sliderCon.getBoundingClientRect();
  
  const itemWidth = selectedItem.offsetWidth;
  const itemOffsetLeft = selectedRect.left - containerRect.left;

  // Store both values for use in animations
  thumbPosition = itemOffsetLeft;
  thumbWidth = itemWidth;

  sliderThumb.style.width = itemWidth + "px";
  sliderThumb.style.left = itemOffsetLeft + "px";
  sliderThumb.style.transform = "none";

  console.log(`Updated thumb for index ${index} - position: ${thumbPosition}px, width: ${thumbWidth}px`);
}

// ✨ NORMAL JS ANIMATION for thumb with simple ease-out
function animateSliderThumbJS(index, sliderThumb, listItems, sliderCon) {
  const selectedItem = listItems[index];
  
  if (!selectedItem) {
    console.error("Selected item not found for index:", index);
    return;
  }

  // Get positions RELATIVE to the slider container
  const selectedRect = selectedItem.getBoundingClientRect();
  const containerRect = sliderCon.getBoundingClientRect();

  const newWidth = selectedItem.offsetWidth;
  const newLeft = selectedRect.left - containerRect.left;

  const currentLeft = thumbPosition;
  const currentWidth = thumbWidth;
  const slideDistance = newLeft - currentLeft;
  const widthDistance = newWidth - currentWidth;

  console.log(`JS Animation for index ${index}:`);
  console.log(`  From: left=${currentLeft}px, width=${currentWidth}px`);
  console.log(`  To: left=${newLeft}px, width=${newWidth}px`);

  // Animation duration
  const duration = 800;
  const startTime = performance.now();
  let animationFrameId = null;

  function animate(currentTime) {
    const elapsed = currentTime - startTime;
    const progress = Math.min(elapsed / duration, 1);
    
    // ✨ Simple ease-out easing (no cubic bezier)
    const easeProgress = easeOut(progress);
    
    // Animate position
    const currentPosition = currentLeft + (slideDistance * easeProgress);
    sliderThumb.style.left = currentPosition + "px";
    
    // Animate width
    const currentWidthValue = currentWidth + (widthDistance * easeProgress);
    sliderThumb.style.width = currentWidthValue + "px";
    
    if (progress < 1) {
      animationFrameId = requestAnimationFrame(animate);
    } else {
      // Final position
      thumbPosition = newLeft;
      thumbWidth = newWidth;
      sliderThumb.style.left = newLeft + "px";
      sliderThumb.style.width = newWidth + "px";
      console.log(`Animation complete for index ${index}`);
    }
  }

  animationFrameId = requestAnimationFrame(animate);
}

// ✨ Simple ease-out easing function
function easeOut(t) {
  return 1 - Math.pow(1 - t, 3);
}

function startAutoResetTimer(pictures, sliderThumb, listItems, sliderCon) {
  autoResetTimer = setTimeout(() => {
    if (currentIndex !== 0 && !isAnimating) {
      console.log("Auto-resetting to index 0 (Photo editing)");
      navigateToIndex(0, pictures, sliderThumb, listItems, sliderCon);
    }
  }, AUTO_RESET_DELAY);
}

function clearAutoResetTimer() {
  if (autoResetTimer) {
    clearTimeout(autoResetTimer);
    autoResetTimer = null;
    console.log("Auto-reset timer cleared");
  }
}

// Initialize on load
initAnimation();