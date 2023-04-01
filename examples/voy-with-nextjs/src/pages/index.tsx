import Head from "next/head";

import ReactMarkdown from "react-markdown";
import rehypeRaw from "rehype-raw";

import { NavigationBar } from "@/components/NavigationBar";

const readme = require("./../../../../README.md").default;

export default function Home() {
  return (
    <>
      <Head>
        <title>Voy & Next.js Example</title>
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <main>
        <NavigationBar />
        <section>
          <p>
            This example project includes a client side project using
            @visheratin/web-ai for the embeddings, and a server side project
            using @tensorflow-models/universal-sentence-encoder for the
            embeddings.
          </p>
          <hr />
          <ReactMarkdown rehypePlugins={[rehypeRaw]}>{readme}</ReactMarkdown>
        </section>
      </main>
    </>
  );
}
