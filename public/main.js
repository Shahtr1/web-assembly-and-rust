async function init() {
  let rustApp = null;

  try {
    // index.js in pkg directory is responsible for exporting web assembly
    rustApp = await import("../pkg");
  } catch (e) {
    console.error(e);
    return;
  }

  console.log(rustApp);

  const input = document.getElementById("upload");

  // sends file as a string using base64 encoding using fileReader.result
  const fileReader = new FileReader();

  // will get called whenever the file is loaded
  fileReader.onloadend = () => {
    const base64 = fileReader.result.replace(
      /^data:image\/(png|jpeg|jpg);base64,/,
      ""
    );
    const image_data_url = rustApp.grayscale(base64);
    document.getElementById("new-img").setAttribute("src", image_data_url);
  };

  input.addEventListener("change", () => {
    fileReader.readAsDataURL(input.files[0]);
  });
}

init();
