(() => {
  if (!main) var main = document.getElementById("main");
  const div = document.createElement("div");
  div.innerHTML = rustLogoSvg;
  div.innerHTML += `<span>Not Found =>&nbsp;${notFoundPath}</span>`;
  main.appendChild(div);
})();
