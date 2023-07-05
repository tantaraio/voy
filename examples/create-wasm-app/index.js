import { TextModel } from "@visheratin/web-ai";
import { logIndex, logIntro, logResource } from "./log";
import { phrases } from "./phrases";
import { perf } from "./performance";

const initialQuery =
  "Which name is also used to describe the Amazon rainforest in English?";

const initialPhrases = phrases.slice(0, 6);

const main = async () => {
  const timer = perf();

  logIntro("🎉 Welcome to Voy");
  logIntro("🕸️ Loading Voy ...");

  // Loading voy WebAssembly module asynchronously
  const { Voy } = await import("voy-search");

  logIntro(`🕸️ Voy is loaded ✔️ ...`);
  logIntro("🕸️ Voy is indexing [");

  logResource([...initialPhrases.map((p) => `・ "${p}",`)]);

  logIndex(`・ ] (${initialPhrases.length} phrases) ...`);

  // Create text embeddings
  const model = await (await TextModel.create("gtr-t5-quant")).model;
  const processed = await Promise.all(
    initialPhrases.map((q) => model.process(q))
  );

  // Index embeddings with voy
  const data = processed.map(({ result }, i) => ({
    id: String(i),
    title: initialPhrases[i],
    url: `/path/${i}`,
    embeddings: result,
  }));
  const resource = { embeddings: data };

  const voy = new Voy(resource);

  logIndex(`🕸️ Voy is indexed ✔️ ...`);
  logIndex(
    `🕸️ Voy is searching for the nearest neighbors of "${initialQuery}" ...`
  );

  // Perform similarity search for the query embeddings
  const q = await model.process(initialQuery);
  const result = voy.search(q.result, 3);

  // Display search result
  logIndex("🕸️ --- Voy similarity search result ---");

  result.neighbors.forEach((result, i) => {
    if (i === 0) {
      logIndex(`🥇  "${result.title}"`);
    } else if (i === 1) {
      logIndex(`🥈  "${result.title}"`);
    } else if (i === 2) {
      logIndex(`🥉  "${result.title}"`);
    } else {
      logIndex(`🕸️  "${result.title}"`);
    }
  });

  logIndex("⮐");

  const newPhrase = phrases.slice(6, 7);

  logIndex(`🕸️ Voy is adding a new phrase "${newPhrase[0]}" to the index ...`);

  const newEmbeddings = await Promise.all(
    newPhrase.map((q) => model.process(q))
  );

  const addition = newEmbeddings.map(({ result }, i) => ({
    id: String(6),
    title: newPhrase[i],
    url: `/path/${6}`,
    embeddings: result,
  }));

  index = voy.add({ embeddings: addition });

  logIndex(`🕸️ Voy is indexed ✔️ ...`);
  logIndex(
    `🕸️ Voy is searching for the nearest neighbors of "${initialQuery}" ...`
  );
  logIndex("🕸️ --- Voy similarity search result ---");

  voy.search(q.result, 3).neighbors.forEach((result, i) => {
    if (i === 0) {
      logIndex(`🥇  "${result.title}"`);
    } else if (i === 1) {
      logIndex(`🥈  "${result.title}"`);
    } else if (i === 2) {
      logIndex(`🥉  "${result.title}"`);
    } else {
      logIndex(`🕸️  "${result.title}"`);
    }
  });

  logIndex("⮐");
  logIndex(
    `🕸️ Voy is removing the new phrase "${newPhrase[0]}" from the index ...`
  );

  index = voy.remove({ embeddings: addition });
  logIndex(
    `🕸️ Voy is searching for the nearest neighbors of "${initialQuery}" ...`
  );

  logIndex(`🕸️ Voy is indexed ✔️ ...`);
  logIndex("🕸️ --- Voy similarity search result ---");

  voy.search(q.result, 3).neighbors.forEach((result, i) => {
    if (i === 0) {
      logIndex(`🥇  "${result.title}"`);
    } else if (i === 1) {
      logIndex(`🥈  "${result.title}"`);
    } else if (i === 2) {
      logIndex(`🥉  "${result.title}"`);
    } else {
      logIndex(`🕸️  "${result.title}"`);
    }
  });

  logIndex("⮐");
  logIndex(`🕸️ Voy is serializing ...`);

  const serialized = voy.serialize();
  logIndex(`🕸️ Voy is serialized ✔️ ...`);

  logIndex(`🕸️ Voy is deserializing ...`);

  const deserializedVoy = Voy.deserialize(serialized);
  logIndex(`🕸️ Voy is deserialized ✔️ ...`);

  logIndex("🕸️ --- Deserialized Voy similarity search result ---");
  deserializedVoy.search(q.result, 3).neighbors.forEach((result, i) => {
    if (i === 0) {
      logIndex(`🥇  "${result.title}"`);
    } else if (i === 1) {
      logIndex(`🥈  "${result.title}"`);
    } else if (i === 2) {
      logIndex(`🥉  "${result.title}"`);
    } else {
      logIndex(`🕸️  "${result.title}"`);
    }
  });

  logIndex("⮐");
  logIndex(`🕸️ Voy is clearing the index ...`);

  voy.clear();
  deserializedVoy.clear();

  logIndex(`🕸️ Voy is cleared ✔️ ...`);
  logIndex(`✨ Done in ${timer.stop()}s`);
};

main();
