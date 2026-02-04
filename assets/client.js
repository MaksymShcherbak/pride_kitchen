const downloadSvgBtn = document.getElementById("download-svg-btn");
const downloadPngBtn = document.getElementById("download-png-btn");

downloadSvgBtn.addEventListener("click", downloadSvg);
downloadPngBtn.addEventListener("click", downloadPng);

function downloadSvg() {
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
}

function downloadPng() {
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
      const svgBlob = new Blob([source], {
        type: "image/svg+xml;charset=utf-8",
      });
      const url = URL.createObjectURL(svgBlob);

      const img = new Image();
      img.style.imageRendering = "pixelated";
      img.onload = function () {
        const scale = 5;
        const canvas = document.createElement("canvas");
        canvas.width = (img.width || 250) * scale;
        canvas.height = (img.height || 150) * scale;
        const ctx = canvas.getContext("2d");

        ctx.imageSmoothingEnabled = false;
        ctx.scale(scale, scale);
        ctx.drawImage(img, 0, 0);
        URL.revokeObjectURL(url);
        canvas.toBlob(function (blob) {
          const a = document.createElement("a");
          a.href = URL.createObjectURL(blob);
          a.download = "flag.png";
          document.body.appendChild(a);
          a.click();
          document.body.removeChild(a);
        }, "image/png");
      };
      img.src = url;
    });
  }
}

function scrollTop() {
  window.scrollTo({
    top: 0,
    behavior: "smooth",
  });
}
