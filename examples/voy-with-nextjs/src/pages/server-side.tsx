import Head from "next/head";

import styles from "@/styles/Home.module.css";

import { useCallback, useEffect, useRef, useState } from "react";

import { NavigationBar } from "@/components/NavigationBar";

export default function ServerSide() {
  const inputRef = useRef<HTMLInputElement>(null);
  const [embeddings, setEmbeddings] = useState<Record<string, any>[] | null>(
    null
  );

  const [results, setResults] = useState<any[]>([]);

  useEffect(() => {
    fetch("/api/embeddings")
      .then((res) => res.json())
      .then((result) => {
        setEmbeddings(result.embeddings);
      });
  }, []);

  const onSubmit = useCallback(() => {
    if (inputRef.current?.value) {
      setResults([]);
      fetch("/api/process", {
        method: "POST",
        body: JSON.stringify({
          embeddings,
          searchQuery: inputRef.current?.value,
        }),
      })
        .then((res) => res.json())
        .then((result) => {
          setResults(result.neighbors);
        });
    }
  }, [embeddings]);

  return (
    <>
      <Head>
        <title>Voy & Next.js Example - Server Side</title>
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <main>
        <NavigationBar />

        <section>
          <code className={styles.example}>
            {embeddings === null ? (
              <>Waiting for embeddings...</>
            ) : (
              <>
                <span>
                  {results.map((result, i) => {
                    return (
                      <p
                        key={result.title}
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
