import Head from "next/head";

import styles from "@/styles/Home.module.css";

import { useCallback, useEffect, useRef, useState } from "react";

import { NavigationBar } from "@/components/NavigationBar";
import { useVoy } from "@/context/VoyContext";

import { TextModel } from "@visheratin/web-ai";

const phrases = [
  "That is a very happy Person",
  "That is a Happy Dog",
  "Today is a sunny day",
  "Yesterday is a sunny day",
];

export default function ServerSide() {
  const voy = useVoy();
  const [index, setIndex] = useState<string | null>(null);

  const [results, setResults] = useState<any[]>([]);

  const inputRef = useRef<HTMLInputElement>(null);

  const [model, setModel] = useState<any>(null);

  useEffect(() => {
    (async () => {
      if (!model) {
        const modelName = "gtr-t5-quant";

        console.log("Loading model: ", modelName);

        // Create text embeddings
        const model = await (await TextModel.create(modelName)).model;
        setModel(model);
      }
    })();
  }, [model, setModel]);

  useEffect(() => {
    if (!model) return;

    (async () => {
      console.log("Creating embeddings for ", phrases);
      console.log("Processing for model", model);

      const processed = await Promise.all(phrases.map((q) => model.process(q)));

      // Index embeddings with voy
      const data = processed.map(({ result }, i) => ({
        id: String(i),
        title: phrases[i],
        url: `/path/${i}`,
        embeddings: result,
      }));

      const input = { embeddings: data };
      console.log("Indexing", input);

      setIndex(voy.index(input));
    })();
  }, [voy, model]);

  const onSubmit = useCallback(async () => {
    const query = inputRef.current?.value;
    if (!query || !index) return;

    const q = await model.process(query);
    const result = voy.search(index, q.result, 4);

    setResults(result.neighbors);
  }, [index]);

  return (
    <>
      <Head>
        <title>Voy & Next.js Example - Client Side</title>
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <main>
        <NavigationBar />

        <section>
          <code className={styles.example}>
            {model === null ? (
              <>Loading model...</>
            ) : index === null ? (
              <>Waiting for voy index...</>
            ) : (
              <>
                <span>
                  {results.map((result, i) => {
                    return (
                      <p
                        key={i}
                        className={i === 0 ? styles.primary : styles.secondary}
                      >
                        {result.title}
                      </p>
                    );
                  })}
                </span>
                <input type="text" ref={inputRef} />{" "}
                <button onClick={onSubmit}>Submit</button>
                <p>Try searching for:</p>
                <ul>
                  <li>
                    That is a happy <b>p</b>erson
                  </li>
                  <li>
                    That is a happy <b>P</b>erson
                  </li>
                  <li>sunny</li>
                  <li>sunny day</li>
                </ul>
              </>
            )}
          </code>
        </section>
      </main>
    </>
  );
}
