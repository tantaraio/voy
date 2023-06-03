import { TextModel } from "@visheratin/web-ai";
import { logIndex, logIntro, logResource } from "./log";
import { phrases } from "./phrases";
import { perf } from "./performance";

const query =
  "Which name is also used to describe the Amazon rainforest in English?";

const main = async () => {
  const timer = perf();

  logIntro("🎉 Welcome to Voy");
  logIntro("🕸️ Loading Voy ...");

  // Loading voy WebAssembly module asynchronously
  const voy = await import("voy");

  logIntro(`🕸️ Voy is loaded ✔️ ...`);
  logIntro("🕸️ Voy is indexing [");

  logResource([...phrases.map((p) => `・ "${p}",`)]);

  logIndex(`・ ] (${phrases.length} sentences) ...`);

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

  logIndex(`🕸️ Voy is indexed ✔️ ...`);
  logIndex(`🕸️ Voy is searching for the nearest neighbors of "${query}" ...`);

  // Perform similarity search for a query embeddings
  const q = await model.process(query);
  const result = voy.search(index, q.result, 3);

  // Display search result
  logIndex("🕸️ --- Voy similarity search result ---");

  result.neighbors.forEach((result, i) => {
    if (i === 0) {
      logIndex(`🥇  "${phrases[result.id]}"`);
    } else if (i === 1) {
      logIndex(`🥈  "${phrases[result.id]}"`);
    } else if (i === 2) {
      logIndex(`🥉  "${phrases[result.id]}"`);
    } else {
      logIndex(`🕸️  "${phrases[result.id]}"`);
    }
  });

  logIndex(`✨ Done in ${timer.stop()}s`);
};

main();
