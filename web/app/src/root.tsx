import { Suspense } from "solid-js";
import { Body, ErrorBoundary, FileRoutes, Head, Html, Meta, Routes, Scripts, Title, Route } from "solid-start";
import "~/root.css";
import { joinPaths } from "solid-start/islands/server-router";
import Main from "./components/Main";

export let base = import.meta.env.BASE_URL;
if (import.meta.env.MODE === "development") {
  base = import.meta.env.VITE_DEV_BACKEND_URL;
}
export const api = joinPaths(base, "/api");

export default function Root() {
  return (
    <Html lang="en">
      <Head>
        <Title>KsoxExchange</Title>
        <Meta charset="utf-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1" />
      </Head>
      <Body>
        <Suspense>
          <ErrorBoundary>
            <Routes>
              <Route path="/:baseAssetId/:quoteAssetId" component={Main} />
              <FileRoutes />
            </Routes>
          </ErrorBoundary>
        </Suspense>
        <Scripts />
      </Body>
    </Html>
  );
}
