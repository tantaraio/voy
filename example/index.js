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

wasm.then((voy) => {
  console.log("🦀 voy", voy);
  const index = voy.index(input);
  console.log("🦀 Index", index);
  const result = voy.search(index, query, 1);
  console.log("🦀 Result", result);
});
