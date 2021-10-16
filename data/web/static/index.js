const waits = { ".": 300, ":": 500, "\n": 1000 };
let speedy = false;
let running = false;
let text =
  "Welcome Astronaut. You have been de-attached from the command pod and are hurtling towards the lunar surface and approximately 2000 m/s.  Connection to mission control has been lost in the aforementioned accident. Finding a better landing spot is imperative.";

document.querySelector("#start").addEventListener("click", startGame);

window.onkeydown = window.onkeyup = (e) => {
  if (e.keyCode !== 32) return;
  if (!running && e.type === "keydown") startGame();
  if (e.type === "keydown") speedy = true;
  if (e.type === "keyup") speedy = false;
};

function updateScreenChar(index, toSpace) {
  index++;
  if (index >= text.length) return;

  let delay = 50;
  if (Object.keys(waits).includes(text[index])) delay = waits[text[index]];
  if (speedy) delay /= 2;

  if (text[index] === " ") {
    setTimeout(() => updateScreenChar(index, true), delay);
    return;
  }

  let s = "";
  if (toSpace) s = " ";
  new Audio("assets/blip2.mp3").play();
  let cons = document.querySelector("#console");
  cons.innerHTML = `${cons.innerHTML.slice(
    0,
    cons.innerHTML.length - 28
  )}${s}${text[index].replace("\n", "<br>")}<span class="blink">█</span>`;

  setTimeout(() => updateScreenChar(index, false), delay);
}

function startGame() {
  running = true;
  document.querySelector("#start").style.display = "none";
  document.querySelector("#wrap").style.filter = "";
  updateScreenChar(-1, false);
}
