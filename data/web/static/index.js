const waits = { ".": 300, ":": 500, "\n": 1000 };
let init3DStuff = false;
let running = false;
let speedy = false;
let text =
  "Welcome Astronaut. You have been de-attached from the command pod and are hurtling towards the lunar surface and approximately 2000 m/s.  Connection to mission control has been lost in the aforementioned accident. Finding a better landing spot is imperative.";

document.querySelector("#start").addEventListener("click", startGame);

window.onkeydown = window.onkeyup = (e) => {
  if (e.type === "keydown" && e.keyCode === 82) {
    running = false;
    document.querySelector("#start").style.opacity = "1";
    document.querySelector("#wrap").style.filter = "blur(5px)";
    document.querySelector("#console").innerHTML = "";
  }
  if (e.keyCode !== 32) return;
  if (!running && e.type === "keydown") startGame();
  if (e.type === "keydown") speedy = true;
  if (e.type === "keyup") speedy = false;
};

function updateScreenChar(index, toSpace) {
  index++;
  if (!running) return;
  if (index >= text.length) return;

  let delay = 50;
  if (Object.keys(waits).includes(text[index])) delay = waits[text[index]];
  if (speedy) delay /= 2;

  new Audio("assets/blip2.mp3").play();

  if (text[index] === " ") {
    setTimeout(() => updateScreenChar(index, true), delay);
    return;
  }

  let s = "";
  if (toSpace) s = " ";
  let cons = document.querySelector("#console");
  cons.innerHTML = `${cons.innerHTML.slice(
    0,
    cons.innerHTML.length - 28
  )}${s}${text[index].replace("\n", "<br>")}<span class="blink">█</span>`;

  setTimeout(() => updateScreenChar(index, false), delay);
}

function startGame() {
  running = true;
  if (!init3DStuff) init3D();
  document.querySelector("#start").style.opacity = "0";
  document.querySelector("#wrap").style.filter = "";
  setTimeout(() => updateScreenChar(-1, false), 750);
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

  renderer.setSize(window.innerWidth * 0.5 - 30, window.innerHeight * 0.5 - 30);
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
      window.innerWidth * 0.5 - 30,
      window.innerHeight * 0.5 - 30
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
