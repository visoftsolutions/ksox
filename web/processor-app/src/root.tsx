import { Suspense } from "solid-js";
import { Body, ErrorBoundary, FileRoutes, Head, Html, Link, Meta, Route, Routes, Scripts, Title } from "solid-start";
import "~/root.css";
import { joinPaths } from "solid-start/islands/server-router";
import Index from "~/routes";
import App from "./components/App";
import { Nav, NavProvider, setNav } from "./utils/providers/NavProvider";
import Transfer from "./components/Transfer";

export const base = import.meta.env.BASE_URL;
export const api = joinPaths(base, "/api");

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
                  <Route path={["/"]} element={<App />} preload={() => setNav(Nav.App)} />
                  <Route path="/transfer" element={<Transfer />} preload={() => setNav(Nav.Transfer)} />
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
