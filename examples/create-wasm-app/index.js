import { TextModel } from "@visheratin/web-ai";
import { log } from "./log";
import { phrases } from "./phrases";
import { perf } from "./performance";

const query =
  "Which name is also used to describe the Amazon rainforest in English?";

const main = async () => {
  const timer = perf();

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
  log(`🕸️ Voy is searching for the nearest neighbors for "${query}" ...`);

  // Perform similarity search for a query embeddings
  const q = await model.process(query);
  const result = voy.search(index, q.result, 3);

  // Display search result
  log("🕸️ --- Voy similarity search result ---");
  result.neighbors.forEach((result, i) => {
    if (i === 0) {
      log(`🥇  "${phrases[result.id]}"`);
    } else if (i === 1) {
      log(`🥈  "${phrases[result.id]}"`);
    } else if (i === 2) {
      log(`🥉  "${phrases[result.id]}"`);
    } else {
      log(`🕸️  "${phrases[result.id]}"`);
    }
  });

  log(`✨ Done in ${timer.stop()}s`);
};

main();
