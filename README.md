<div align="center">
  <h1>Voy</h1>
  <strong>A WASM vector similarity search engine written in Rust</strong>
</div>

![voy: a vector similarity search engine in WebAssembly][demo]

[![npm version](https://badge.fury.io/js/voy-search.svg)](https://badge.fury.io/js/voy-search)

- Tiny: 75KB gzipped, 69KB brotli
- Fast: voy uses [k-d tree][k-d-tree] to index and provide fast search
- Tree Shakable: built with performance optimization in mind

> **🚜 Work in Progress**
>
> voy is still in development. We are working on:
>
> - Built-in text transformation in WebAssembly: voy relies on JavaScript libraries like [`web-ai`][web-ai] to generate text embeddings.
> - Index update: Currently it's required to re-build the index when a resource update occurs.
> - TypeScript support: Due to the limitation of WASM tooling, complex data types are not auto-generated.

## Installation

```bash
# with npm
npm i voy-search

# with Yarn
yarn add voy-search

# with pnpm
pnpm add voy-search
```

## APIs

#### `index(input: Resource): SerializedIndex`

**Parameters**

```ts
interface Resource {
  embeddings: Array<{
    id: string; // id of the resource
    title: string; // title of the resource
    url: string; // path to the resource
    embeddings: number[]; // embeddings of the resource
  }>;
}
```

**Return**

```ts
type SerializedIndex = string; // serialized k-d tree
```

#### `search(index: SerializedIndex, query: Query, k: NumberOfResult): Nearests`

**Parameter**

```ts
type SerializedIndex = string; // serialized k-d tree

type Query = number[]; // embeddings of the search query

type NumberOfResult = number; // K top results to return
```

**Return**

```ts
type Nearests = Array<{
  id: string; // id of the nearest resource
  title: string; // title of the nearest resource
  url: string; // path of the nearest resource
  body: string; // body of the nearest resource
  embeddings: number[]; // embeddings of the nearest resource
}>;
```

## Usage

As of now, voy relies on libraries like [`web-ai`][web-ai] to generate embeddings for text:

```js
import { TextModel } from "@visheratin/web-ai";

const voy = await import("voy");

const phrases = [
  "That is a very happy Person",
  "That is a Happy Dog",
  "Today is a sunny day",
];
const query = "That is a happy person";

// Create text embeddings
const model = await (await TextModel.create("gtr-t5-quant")).model;
const processed = await Promise.all(phrases.map((q) => model.process(q)));

// Index embeddings with voy
const data = processed.map(({ result }, i) => ({
  id: String(i),
  title: phrases[i],
  url: `/path/${i}`,
  embeddings: result,
}));
const input = { embeddings: data };
const index = voy.index(input);

// Perform similarity search for a query embeddings
const q = await model.process(query);
const nearests = voy.search(index, q.result, 1);

// Display search result
nearests.forEach((result) =>
  console.log(`✨ voy similarity search result: "${result.title}"`)
);
```

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Sponsor

<a href="https://reflect.app" target="_blank"><img src="https://avatars.githubusercontent.com/u/73365487?s=64&v=4"></a>

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

[demo]: ./voy.gif "voy demo"
[web-ai]: https://github.com/visheratin/web-ai
[k-d-tree]: https://en.wikipedia.org/wiki/K-d_tree
