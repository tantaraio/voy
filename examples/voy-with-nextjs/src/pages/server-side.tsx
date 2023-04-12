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
    fetch(`${process.env.NEXT_PUBLIC_API_BASE_URL}/api/embeddings`).then(
      async (res) => {
        const { embeddings } = await res.json();
        setEmbeddings(embeddings);
      }
    );
  }, []);

  const onSubmit = useCallback(() => {
    if (inputRef.current?.value) {
      setResults([]);
      fetch(`${process.env.NEXT_PUBLIC_API_BASE_URL}/api/process`, {
        method: "POST",
        body: JSON.stringify({
          embeddings,
          searchQuery: inputRef.current?.value,
        }),
      }).then(async (res) => {
        const { nearests } = await res.json();
        setResults(nearests);
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
