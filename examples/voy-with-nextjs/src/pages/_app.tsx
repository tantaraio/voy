import { VoyProvider } from "@/context/VoyContext";
import "@/styles/globals.css";
import type { AppProps } from "next/app";

export default function App({ Component, pageProps }: AppProps) {
  return (
    <VoyProvider>
      <Component {...pageProps} />
    </VoyProvider>
  );
}
