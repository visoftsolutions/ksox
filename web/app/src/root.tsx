import { Suspense } from "solid-js";
import { Body, ErrorBoundary, FileRoutes, Head, Html, Link, Meta, Route, Routes, Scripts, Title } from "solid-start";
import "~/root.css";
import { joinPaths } from "solid-start/islands/server-router";
import App from "~/routes";
import Main from "./components/Main";
import Assets from "./components/Assets";
import Account from "./components/Account";
import { Nav, NavProvider, setNav } from "./utils/providers/NavProvider";

export const base = import.meta.env.BASE_URL;
export const api = joinPaths(base, "/api");

export default function Root() {
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
            <NavProvider>
              <Routes>
                <Route path="/" component={App}>
                  <Route path={["/", "/:baseAssetId/:quoteAssetId"]} element={<Main />} preload={() => setNav(Nav.Spot)}/>
                  <Route path="/assets" element={<Assets/>} preload={() => setNav(Nav.Assets)} />
                  <Route path="/account" element={<Account/>} preload={() => setNav(Nav.Account)} />
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
