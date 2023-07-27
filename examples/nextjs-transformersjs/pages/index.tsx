import { Pipeline, pipeline } from "@xenova/transformers";
import { Inter } from "next/font/google";
import { useCallback, useEffect, useState } from "react";
import { SearchResult, Voy } from "voy-search";

const inter = Inter({ subsets: ["latin"] });

const phrases = [
  "That is a very happy Person",
  "That is a Happy Dog",
  "Today is a sunny day",
];

const query = "Is it summer yet?";

async function getExtractor() {
  const extractor = await pipeline(
    "feature-extraction",
    "Xenova/all-MiniLM-L6-v2"
  );
  return extractor;
}

async function extract(extractor: Pipeline, text: string) {
  const result = await extractor(text, { pooling: "mean", normalize: true });
  return result.data;
}

export default function Home() {
  const [result, setResult] = useState<SearchResult>();

  const run = useCallback(async (extractor: Pipeline) => {
    const embeddings = await Promise.all(
      phrases.map((phrase) => extract(extractor, phrase))
    );

    const data = embeddings.map((embeddings, i) => ({
      id: String(i),
      title: phrases[i],
      url: `/path/${i}`,
      embeddings: Array.from(embeddings) as number[],
    }));

    const index = new Voy({ embeddings: data });

    const q = await extract(extractor, query);

    const result = index.search(q, 1);

    setResult(result);
  }, []);

  useEffect(() => {
    getExtractor().then(run);
  }, []);

  return (
    <main
      className={`flex min-h-screen flex-col justify-center p-4 md:p-24 ${inter.className}`}
    >
      <div className="my-4">
        <h5 className="font-bold">📚 Index:</h5>
        {phrases.map((phrases) => (
          <p key={phrases}>{phrases}</p>
        ))}
      </div>
      <div className="my-4">
        <h5 className="font-bold">❓ Query: </h5>
        <p>{query}</p>
      </div>
      <div className="my-4">
        <h5 className="font-bold">✨ Search Result</h5>
        {!result && <p>...</p>}
        {result?.neighbors.map((n) => (
          <p key={n.id}>{n.title}</p>
        ))}
      </div>
    </main>
  );
}
