import { TextModel } from "@visheratin/web-ai";
import { log } from "./log";

const phrases = [
  "That is a very happy Person",
  "That is a Happy Dog",
  "Today is a sunny day",
];

const query = "That is a happy person";

const main = async () => {
  log("🎉 Welcome to voy");
  log("🕸️ Loading voy ...");

  // Loading voy WebAssembly module asynchronously
  const voy = await import("voy");

  log(`🕸️ voy is loaded ✔️ ...`);
  log(`🕸️ voy is indexing [ ${phrases.map((p) => `"${p}"`).join(", ")} ] ...`);

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

  log(`🕸️ voy is indexed ✔️ ...`);
  log(`🕸️ voy is searching for the nearest neighbor for "${query}" ...`);

  // Perform similarity search for a query embeddings
  const q = await model.process(query);
  const nearests = voy.search(index, q.result, 1);

  // Display search result
  nearests.forEach((result) =>
    log(`🕸️ voy similarity search result 👉 "${result.title}"`)
  );

  log("✨ Done");
};

main();
