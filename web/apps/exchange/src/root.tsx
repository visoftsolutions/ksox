import { Suspense, lazy } from "solid-js";
import {
  Body,
  ErrorBoundary,
  FileRoutes,
  Head,
  Html,
  Link,
  Meta,
  Route,
  Routes,
  Scripts,
  Title,
} from "solid-start";
import "~/root.css";
import { joinPaths } from "solid-start/islands/server-router";
import { Nav, NavProvider, setNav } from "~/components/providers/NavProvider";
import { WalletProvider } from "@packages/components/providers/WalletProvider";

export const base = import.meta.env.BASE_URL;
export const api = joinPaths(base, "/api");
export const projectId = import.meta.env
  .VITE_KSOX_WEB_WALLET_CONNECT_PROJECT_ID;

const Index = lazy(() => import("~/routes"));
const App = lazy(() => import("~/components/App"));
const Assets = lazy(() => import("~/components/Assets"));

export default function Root() {
  return (
    <Html lang="en">
      <Head>
        <Title>KsoxExchange</Title>
        <Meta charset="utf-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1" />
        <Meta name="theme-color" content="#0F0D12" />
        <Meta
          name="description"
          content="KSOX is a decentralized exchange powered by zk-STARKs technology. Buy, sell, and trade cryptocurrencies securely and seamlessly. Accept crypto payments through NFC-enabled mobile phones."
        />
        <Meta
          name="keywords"
          content="KSOX, Crypto, decentralized exchange, zk-STARKs, cryptocurrency exchange, buy, sell, trade, transaction fees, arbitrage, transaction processing, Token Crowdsale, KSXT, KSX, investor, crowdfunding, secure, seamless, crypto payments"
        />
        <Link rel="icon" href="/gfx/logo.svg" />
        {import.meta.env.PROD == true ? (
          <>
            <Link
              rel="manifest"
              href={joinPaths(base, "/manifest.webmanifest")}
            />
            <script src={joinPaths(base, "/registerSW.js")} />
            <script
              async
              src="https://www.googletagmanager.com/gtag/js?id=G-HS7VCPVSGW"
            />
            <script async src="/google-analytics.js" />
            <script async src="/cookie3-integration.js" />
          </>
        ) : (
          <></>
        )}
      </Head>
      <Body>
        <Suspense fallback={<p>Loading...</p>}>
          <ErrorBoundary fallback={(e) => <div>{e.message}</div>}>
            <NavProvider>
              <WalletProvider projectId={projectId}>
                <Routes>
                  <Route path="/" component={Index}>
                    <Route
                      path={["/", "/:baseAssetId/:quoteAssetId"]}
                      element={<App />}
                      preload={() => setNav(Nav.App)}
                    />
                    <Route
                      path="/assets"
                      element={<Assets />}
                      preload={() => setNav(Nav.Assets)}
                    />
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
