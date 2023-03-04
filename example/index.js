const append = (box) => (str) => {
  const para = document.createElement("p");
  const text = document.createTextNode(str);

  para.appendChild(text);
  box.appendChild(para);
};

const container = document.querySelector("#example");

const log = append(container);

const wasm = import("voy");

const input = {
  embeddings: [
    {
      id: "abc9821",
      title: "That is a very happy Person",
      url: "/path/to/one",
      embdeddings: [1.0, 2.0, 3.0],
    },
    {
      id: "def1092",
      title: "That is a Happy Dog",
      url: "/path/to/two",
      embdeddings: [3.0, 1.0, 2.0],
    },
    {
      id: "ghi1234",
      title: "Today is a sunny day",
      url: "/path/to/three",
      embdeddings: [2.0, 3.0, 1.0],
    },
  ],
};
const query = [3.1, 0.9, 2.1];

log("🎉 Welcome to voy...");

log(
  `🖥️ Search for [${query.toString()}]'s the nearest embeddings...`,
  container
);

wasm
  .then((voy) => {
    log(`🕸️ Voy is loaded...`);
    return voy;
  })
  .then((voy) => {
    const index = voy.index(input);

    log(`🕸️ Voy Index 👉 ${index.toString()}`);

    const results = voy.search(index, query, 1);

    results.forEach((result) =>
      log(`🕸️ Voy Result 👉 [${result.embeddings}]: ${result.title}`)
    );
  })
  .then(() => log("✨ Done"));
