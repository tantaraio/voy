import { TextModel } from "@visheratin/web-ai";
import { log } from "./log";

const phrases = [
  "The Amazon rainforest,[a] also called Amazon jungle or Amazonia, is a moist broadleaf tropical rainforest in the",
  "in the Amazon biome that covers most of the Amazon basin of South America. This basin encompasses 7,000,000 km2",
  "(2,700,000 sq mi), of which 5,500,000 km2 (2,100,000 sq mi) are covered by the rainforest. This region includes",
  "includes territory belonging to nine nations and 3,344 formally acknowledged indigenous territories.",
  "The majority of the forest, 60%, is in Brazil, followed by Peru with 13%, Colombia with 10%, and with minor amounts in",
  'amounts in Bolivia, Ecuador, French Guiana, Guyana, Suriname, and Venezuela. Four nations have "Amazonas" as the',
];

const query =
  "Which name is also used to describe the Amazon rainforest in English?";

const main = async () => {
  log("🎉 Welcome to Voy");
  log("🕸️ Loading Voy ...");

  // Loading voy WebAssembly module asynchronously
  const voy = await import("voy");

  log(`🕸️ Voy is loaded ✔️ ...`);
  log([
    "🕸️ Voy is indexing [",
    ...phrases.map((p) => `・ "${p},"`),
    "・ ] ...",
  ]);

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
  const resource = { embeddings: data };
  const index = voy.index(resource);

  log(`🕸️ Voy is indexed ✔️ ...`);
  log(`🕸️ Voy is searching for the nearest neighbor for "${query}" ...`);

  // Perform similarity search for a query embeddings
  const q = await model.process(query);
  const result = voy.search(index, q.result, 1);

  // Display search result
  result.neighbors.forEach((result) =>
    log(`🕸️ Voy similarity search result 👉 "${phrases[result.id]}"`)
  );

  log("✨ Done");
};

main();
