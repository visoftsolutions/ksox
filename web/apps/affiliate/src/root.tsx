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
import { Nav, NavProvider, setNav } from "~/components/providers/NavProvider";
import { joinPaths } from "solid-start/islands/server-router";
import { WalletProvider } from "@packages/components/providers/WalletProvider";

export const base = import.meta.env.BASE_URL;
export const api = joinPaths(base, "/api");
export const projectId = import.meta.env
  .VITE_KSOX_WEB_WALLET_CONNECT_PROJECT_ID;

const Index = lazy(() => import("~/routes"));

export default function Root() {
  return (
    <Html lang="en">
      <Head>
        <Title>KSOX - Cryptocurrency Exchange</Title>
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
        <Link rel="apple-touch-icon" href="/pwa/apple-touch-icon.png" />
        {import.meta.env.PROD == true ? (
          <>
            {/* <Link
              rel="manifest"
              href={joinPaths(base, "/manifest.webmanifest")}
            />
            <script src={joinPaths(base, "/registerSW.js")} /> */}
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
        <Suspense>
          <ErrorBoundary>
            <NavProvider>
              <WalletProvider projectId={projectId}>
                <Routes>
                  <Route path="/" component={Index}>
                    <Route
                      path={"/:code"}
                      element={<Index />}
                      preload={() => setNav(Nav.Landing)}
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
