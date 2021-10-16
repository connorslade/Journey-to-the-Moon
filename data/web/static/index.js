const waits = { ".": 300, ":": 500, "\n": 1000 };
let text =
  "Welcome Astronaut. You have been de-attached from the command pod and are hurtling towards the lunar surface and approximately 2000 m/s.  Connection to mission control has been lost in the aforementioned accident. Finding a better landing spot is imperative.";

document.querySelector("#start").addEventListener("click", () => {
  document.querySelector("#start").style.display = "none";
  document.querySelector("#wrap").style.filter = "";
  updateScreenChar(-1, false);
});

function updateScreenChar(index, toSpace) {
  index++;
  if (index >= text.length) return;

  let delay = 50;
  if (Object.keys(waits).includes(text[index])) delay = waits[text[index]];

  if (text[index] == " ") {
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
