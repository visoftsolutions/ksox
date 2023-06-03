import { Suspense, lazy } from "solid-js";
import { Body, ErrorBoundary, FileRoutes, Head, Html, Link, Meta, Route, Routes, Scripts, Title } from "solid-start";
import "~/root.css";
import { joinPaths } from "solid-start/islands/server-router";
import { Nav, NavProvider, setNav } from "~/components/providers/NavProvider";

export const base = import.meta.env.BASE_URL;
export const api = joinPaths(base, "/api");
export const projectId = import.meta.env.VITE_WALLET_CONNECT_PROJECT_ID;

const Index = lazy(() => import("~/routes"));
const App = lazy(() => import("./components/App"));
const Account = lazy(() => import("./components/Account"));
const Asset = lazy(() => import("./components/Asset"));

export default function Root() {
  return (
    <Html lang="en">
      <Head>
        <Title>KsoxPay</Title>
        <Meta charset="utf-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1" />
        <Meta name="theme-color" content="#0F0D12" />
        <Meta name="description" content="Ksox Payment Processor" />
        <Link rel="icon" href="/gfx/logo.svg" />
      </Head>
      <Body>
        <Suspense>
          <ErrorBoundary>
            <NavProvider>
              <Routes>
                <Route path="/" component={Index}>
                  <Route path="/" element={<App />} preload={() => setNav(Nav.App)} />
                  <Route path="/account" element={<Account />} preload={() => setNav(Nav.Account)} />
                  <Route path="/asset/:assetId" element={<Asset />} preload={() => setNav(Nav.Asset)} />
                </Route>
                <FileRoutes />
              </Routes>
            </NavProvider>
          </ErrorBoundary>
        </Suspense>
        <Scripts />
      </Body>
    </Html>
  );
}
