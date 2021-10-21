const waits = { ".": 300, ":": 500, "\n": 1000 };
let init3DStuff = false;
let validOptions = [];
let running = false;
let typeing = false;
let this_path = [];
let downKeys = {};
let input = "";
let ask = "";

const garble = [
  "          Orbital Resonance",
  "          Retrograde Apathy",
  "         Delta Hamming Code",
  "         Periapsis altitude",
  "  Circualization Completion",
  "       Ballistic trajectory",
  "Specific Impulse Commitment",
  "        Tangential Velocity",
];

const art = [
  "           ,:",
  "  OK_____,'?|",
  "        /   :    OK",
  "  ALL SYSTEMS____/",
  "   OK / />/",
  "   /_/  ????",
  "  __/?  /        OK",
  "  )'-. /_________/",
  " ./*o :\\ ",
  " /.'*'",
  " '/'",
  "*+",
  ".",
  "",
];

const printTextDone = new Event("printTextDone");
const cons = document.querySelector("#console");
const info = document.querySelector("#info");
const ascii = document.querySelector("#ascii");

// Make start button... start the game
document.querySelector("#start").addEventListener("click", startGame);

window.addEventListener("keyup", (e) => {
  downKeys[e.key] = false;
});

// Speedup and Reset keyboard stuff
window.addEventListener("keydown", (e) => {
  if (downKeys[e.key]) return;
  downKeys[e.key] = true;
  new Audio("assets/keyPress.mp3").play();

  if (validOptions.includes(e.key) && typeing) {
    input += e.key;
    cons.innerHTML = `${cons.innerHTML.slice(0, cons.innerHTML.length - 28)}${
      e.key
    }<span class="blink">█</span>`;
  }

  if (e.key === "Backspace") {
    let cons = document.querySelector("#console");
    cons.innerHTML = `${cons.innerHTML.slice(
      0,
      cons.innerHTML.length - 29
    )}<span class="blink">█</span>`;
  }

  if (e.key === "Enter" && typeing && running) {
    typeing = false;

    cons.innerHTML = `${cons.innerHTML.slice(
      0,
      cons.innerHTML.length - 28
    )}${"<br><br>"}<span class="blink">█</span>`;

    textPath(`${this_path.join("-")}-${parseInt(input) - 1}`);
  }

  if (e.key === "r") {
    validOptions = [];
    running = false;
    typeing = false;
    this_path = [];
    downKeys = {};
    input = "";
    ask = "";
    document.querySelector("#start").style.opacity = "1";
    document.querySelector("#title").style.opacity = "1";
    document.querySelector("#credits").style.opacity = "1";
    document.querySelector("#wrap").style.filter = "blur(5px)";
    document.querySelector("#console").innerHTML =
      '<span class="blink">█</span>';
  }
  if (e.key !== " " && e.key !== "Enter") return;
  if (!running) startGame();
});

// Called when a Print Text Job Finishes
window.addEventListener("printTextDone", () => {
  input = "";
  typeing = true;
});

// For drawing to the console
function updateScreenChar(text, index, toSpace) {
  typeing = false;
  keysRuning = true;
  index++;
  if (!running) return;
  if (index >= text.length) {
    window.dispatchEvent(printTextDone);
    return;
  }

  let delay = 50;
  if (Object.keys(waits).includes(text[index])) delay = waits[text[index]];
  if (downKeys[" "]) delay /= 2;

  if (downKeys["s"]) {
    // running = false;
    cons.innerHTML = `${cons.innerHTML.slice(0, cons.innerHTML.length - 28)}${
      toSpace ? " " : ""
    }${text
      .slice(index, text.length)
      .replace(/\n/g, "<br>")}<span class="blink">█</span>`;
    document.getElementsByClassName("blink")[0].scrollIntoView();
    window.dispatchEvent(printTextDone);
    return;
  }
  new Audio("assets/blip.mp3").play();

  if (text[index] === " ") {
    setTimeout(() => updateScreenChar(text, index, true), delay);
    return;
  }

  cons.innerHTML = `${cons.innerHTML.slice(0, cons.innerHTML.length - 28)}${
    toSpace ? " " : ""
  }${text[index].replace("\n", "<br>")}<span class="blink">█</span>`;
  document
    .getElementsByClassName("blink")[0]
    .scrollIntoView({ behavior: "smooth" });

  setTimeout(() => updateScreenChar(text, index, false), delay);
}

function textPath(path) {
  let parts = path.split("-");
  this_path.push(parts[parts.length - 1]);
  fetch("/api/option?q=" + path).then((r) => {
    if (r.status !== 200) {
      this_path.pop();
      input = "";
      updateScreenChar(
        ` Invalid Option... Lets try that again${ask}`,
        0,
        false
      );
      return;
    }

    r.json().then((data) => {
      let options = "";
      ask = "";

      if ("answer" in data) {
        data.answer.forEach((item, i) => {
          i++;
          validOptions = validOptions.concat(i.toString().split(""));
          options += `${i}) ${item.option}\n`;
        });
        ask = `\n\n${options.trim()}\n\n>`;
      }

      if ("end" in data) {
        ask = "\n\n";
        ask += data.end ? "You Win! Woo!" : "You Lose... oop";
        document.getElementsByClassName("blink")[0].scrollIntoView();
        // running = typeing = false;
      }

      updateScreenChar(
        `${data.text ? `${data.text}\n\n` : ""}${data.question}${ask}`,
        -1,
        false
      );
    });
  });
}

// Start the game
// ... if the name wasent clear
function startGame() {
  running = true;
  if (!init3DStuff) init3D();
  controlPanel();
  document.querySelector("#start").style.opacity = "0";
  document.querySelector("#title").style.opacity = "0";
  document.querySelector("#credits").style.opacity = "0";
  document.querySelector("#wrap").style.filter = "";

  setTimeout(() => textPath("0"), 750);
}

// 3D Garbage
function init3D() {
  init3DStuff = true;
  const scene = new THREE.Scene();
  const loader = new THREE.STLLoader();
  const renderer = new THREE.WebGLRenderer();
  const material = new THREE.MeshBasicMaterial({
    color: 0x00ff00,
    wireframe: true,
  });

  renderer.setSize(
    window.innerWidth * 0.5 - window.innerHeight * 0.025 - 30,
    window.innerHeight * 0.5 - window.innerHeight * 0.005 - 30
  );
  document.querySelector("#random").appendChild(renderer.domElement);

  let camera = new THREE.PerspectiveCamera(
    45,
    window.innerWidth / window.innerHeight,
    1,
    10
  );
  camera.position.z = 5;

  loader.load("assets/lander.stl", (geometry) => {
    const mesh = new THREE.Mesh(geometry, material);

    mesh.position.set(0, -1.5, 0);
    mesh.rotation.set(180, 0, 0);
    mesh.scale.set(1, 1, 1);

    scene.add(mesh);

    const animate = () => {
      requestAnimationFrame(animate);

      mesh.rotation.z += 0.01;
      renderer.render(scene, camera);
    };

    animate();
  });

  window.addEventListener("resize", () => {
    renderer.setSize(
      window.innerWidth * 0.5 - window.innerHeight * 0.025 - 30,
      window.innerHeight * 0.5 - window.innerHeight * 0.005 - 30
    );
    camera = new THREE.PerspectiveCamera(
      45,
      window.innerWidth / window.innerHeight,
      1,
      10
    );
    camera.position.z = 5;
  });
}

// display random crap on the side of the screen that does literally nothing and is completely useless
// nice comment ↗
function controlPanel() {
  ascii.innerHTML = info.innerHTML = "<pre>";
  for (let i = 0; i < garble.length - 1; i++)
    info.innerHTML += `${garble[i].replace(RegExp(/ /g), "&nbsp;")}: ${
      Math.random() >= 0.5 ? "+" : "-"
    }${("000" + Math.floor(Math.random() * 1000)).slice(-3)}<br>`;

  for (let i = 0; i < art.length - 1; i++)
    ascii.innerHTML += art[i].replace(RegExp(/ /g), "&nbsp;") + "<br>";
  ascii.innerHTML += "<br>Dampening code: ";

  for (let i = 0; i < 8; i++)
    ascii.innerHTML += String.fromCharCode(65 + Math.floor(Math.random() * 26));
  ascii.innerHTML += "</pre>";
  info.innerHTML += "<br><br>";

  for (let i = 0; i < 8; i++) {
    info.innerHTML += `${"&nbsp;".repeat(8)}Orbital pd ${i}: ${"█".repeat(
      1 + Math.floor(Math.random() * 14)
    )}<br>`;
  }
  info.innerHTML += "</pre>";

  setTimeout(() => controlPanel(), 5000);
}
