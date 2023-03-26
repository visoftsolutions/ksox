import { onMount, Suspense } from "solid-js";
import { Body, ErrorBoundary, FileRoutes, Head, Html, Link, Meta, Routes, Scripts, Title } from "solid-start";
import "./root.css";
import "./api/public/assets";
import { joinPaths } from "solid-start/islands/server-router";
import { get, sse } from "./api/mod";

export const base = import.meta.env.BASE_URL;

export default function Root() {
  onMount(async () => {
    console.log(await get());
    sse().onmessage = (event) => {
      console.log(event.data);
    };
  });

  return (
    <Html lang="en">
      <Head>
        <Title>KsoxExchange</Title>
        <Meta charset="utf-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1" />
        <Link rel="icon" type="image/x-icon" href={joinPaths(base, "/favicon.ico")} />
      </Head>
      <Body>
        <Suspense>
          <ErrorBoundary>
            <Routes>
              <FileRoutes />
            </Routes>
          </ErrorBoundary>
        </Suspense>
        <Scripts />
      </Body>
    </Html>
  );
}
