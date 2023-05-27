import { NextApiRequest, NextApiResponse } from "next";

import { load } from "@tensorflow-models/universal-sentence-encoder";

require("@tensorflow/tfjs-node");

import { index, search } from "voy-search";

const model = await load();

export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse
) {
  const { embeddings = [], searchQuery = "" } = JSON.parse(req.body);

  const indexed = index({ embeddings });

  const q = await model.embed([searchQuery.toString()]);
  const result = search(indexed, new Float32Array(q.arraySync()[0]), 5);

  res.status(200).json(result);
}
