import { NextApiRequest, NextApiResponse } from "next";

import { load } from "@tensorflow-models/universal-sentence-encoder";

require("@tensorflow/tfjs-node");

const model = await load();

export default async function handler(
  _req: NextApiRequest,
  res: NextApiResponse
) {
  const phrases = [
    "That is a very happy Person",
    "That is a Happy Dog",
    "Today is a sunny day",
    "Yesterday is a sunny day",
  ];

  // Create text embeddings
  const processed = await model.embed(phrases);
  const data = processed.arraySync().map((result: number[], i: number) => ({
    id: String(i),
    title: phrases[i],
    url: `/path/${i}`,
    embeddings: result,
  }));

  res.status(200).json({ embeddings: data });
}
