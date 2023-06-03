import { Suspense, lazy } from "solid-js";
import { Body, ErrorBoundary, FileRoutes, Head, Html, Link, Meta, Route, Routes, Scripts, Title } from "solid-start";
import "~/root.css";
import { joinPaths } from "solid-start/islands/server-router";
import { Nav, NavProvider, setNav } from "./components/providers/NavProvider";
import { WalletProvider } from "@web/components/providers/WalletProvider";

export const base = import.meta.env.BASE_URL;
export const api = joinPaths(base, "/api");
export const projectId = import.meta.env.VITE_WALLET_CONNECT_PROJECT_ID;

const Index = lazy(() => import("~/routes"));
const App = lazy(() => import("./components/App"));
const Assets = lazy(() => import("./components/Assets"));

export default function Root() {
  return (
    <Html lang="en">
      <Head>
        <Title>KsoxExchange</Title>
        <Meta charset="utf-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1" />
        <Meta name="theme-color" content="#0F0D12" />
        <Meta name="description" content="Ksox Cryptocurrency Exchange" />
        <Link rel="icon" href="/gfx/logo.svg" />
      </Head>
      <Body>
        <Suspense>
          <ErrorBoundary>
            <NavProvider>
              <WalletProvider>
                <Routes>
                  <Route path="/" component={Index}>
                    <Route path={["/", "/:baseAssetId/:quoteAssetId"]} element={<App />} preload={() => setNav(Nav.App)} />
                    <Route path="/assets" element={<Assets />} preload={() => setNav(Nav.Assets)} />
                  </Route>
                  <FileRoutes />
                </Routes>
              </WalletProvider>
            </NavProvider>
          </ErrorBoundary>
        </Suspense>
        <Scripts />
      </Body>
    </Html>
  );
}
