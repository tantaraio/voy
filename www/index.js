import * as wasm from "voy";

const input = [
  {
    id: "abd",
    title: "That is a very happy Person",
    url: "/path/to/one",
    embdeddings: [1.0, 2.0, 3.0],
  },
  {
    id: "abd",
    title: "That is a Happy Dog",
    url: "/path/to/two",
    embdeddings: [3.0, 1.0, 2.0],
  },
  {
    id: "abd",
    title: "Today is a sunny day",
    url: "/path/to/three",
    embdeddings: [2.0, 3.0, 1.0],
  },
];

wasm.index(input);
wasm.search();
