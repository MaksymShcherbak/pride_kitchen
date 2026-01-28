const svg = document.getElementById("result");
if (!svg) {
  alert("SVG element not found!");
} else {
  const images = svg.querySelectorAll("image");
  const promises = [];

  images.forEach((img) => {
    const href = img.getAttribute("href") || img.getAttribute("xlink:href");
    if (!href || href.startsWith("data:")) return;

    const p = fetch(href)
      .then((res) => res.blob())
      .then((blob) => {
        return new Promise((resolve) => {
          const reader = new FileReader();
          reader.onloadend = () => {
            img.setAttribute("href", reader.result);
            resolve();
          };
          reader.readAsDataURL(blob);
        });
      });

    promises.push(p);
  });

  Promise.all(promises).then(() => {
    const serializer = new XMLSerializer();
    const source = serializer.serializeToString(svg);
    const blob = new Blob([source], { type: "image/svg+xml;charset=utf-8" });
    const url = URL.createObjectURL(blob);
    const a = document.createElement("a");
    a.href = url;
    a.download = "flag.svg";
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    URL.revokeObjectURL(url);
  });
}
