const waits = { ".": 300, ":": 500 };
let text =
  "Welcome Astronaut. You have been de-attached from the command pod and are hurtling towards the lunar surface and approximately 2000 m/s.  Connection to mission control has been lost in the aforementioned accident. Finding a better landing spot is imperative.";

updateScreenChar(-1, false);

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
  document.querySelector("#console").innerText += `${s}${text[index]}`;

  setTimeout(() => updateScreenChar(index, false), delay);
}
