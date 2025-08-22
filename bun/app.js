// Simple log generator
setInterval(() => {
  const rand = Math.random();
  
  if (rand < 0.5) {
    console.log("Info: Everything is working fine");
  } else if (rand < 0.8) {
    console.warn("Warning: Something might be wrong");
  } else {
    console.error("Error: Something went wrong!");
  }
}, 1000);
