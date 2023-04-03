import { Suspense } from "solid-js";
import { Body, ErrorBoundary, FileRoutes, Head, Html, Link, Meta, Route, Routes, Scripts, Title } from "solid-start";
import "~/root.css";
import { joinPaths } from "solid-start/islands/server-router";
import App from "~/routes";
import { SessionProvider } from "./components/Buttons/WalletButton";
import { MarketProvider } from "./utils/providers/MarketProvider";
import { AssetsProvider } from "./utils/providers/AssetsProvider";
import Main from "./components/Main";
import Assets from "./components/Assets";
import Account from "./components/Account";

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
            <Routes>
              <Route path="/" component={App}>
                <Route path={["/", "/:baseAssetId/:quoteAssetId"]} component={Main} />
                <Route path="/assets" component={Assets} />
                <Route path="/account" component={Account} />
              </Route>
              <FileRoutes />
            </Routes>
          </ErrorBoundary>
        </Suspense>
        <Scripts />
      </Body>
    </Html>
  );
}
