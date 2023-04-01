import Link from "next/link";

import classes from "./NavigationBar.module.css";

export const NavigationBar = () => {
  return (
    <>
      <h1>Voy</h1>
      <h3> 🕸️🦀 A WASM vector similarity search written in Rust</h3>
      <ul className={classes.ul}>
        <li>
          <Link href="/">⭐ Start</Link>
        </li>

        <li>
          <Link href="/server-side">🌐 Server Side Example</Link>
        </li>

        <li>
          <Link href="/client-side">🧑🏽‍💻 Client Side Example</Link>
        </li>
      </ul>
    </>
  );
};
