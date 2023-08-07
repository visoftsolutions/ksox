// @refresh reload
import { Suspense, lazy, onMount } from "solid-js";
import {
  Body,
  ErrorBoundary,
  FileRoutes,
  Head,
  Html,
  Meta,
  Routes,
  Scripts,
  Title,
  Link,
  Route,
} from "solid-start";
import "./root.css";
import { Nav, NavProvider, setNav } from "./components/providers/NavProvider";
import { joinPaths } from "solid-start/islands/server-router";
import { WalletProvider } from "@web/components/providers/WalletProvider";

export const base = import.meta.env.BASE_URL;
export const api = joinPaths(base, "/api");
export const projectId = import.meta.env.VITE_WALLET_CONNECT_PROJECT_ID;
export const alchemyId = import.meta.env.VITE_ALCHEMY_API_KEY;

const Index = lazy(() => import("~/routes"));
const App = lazy(() => import("./components/App"));

export default function Root() {
  return (
    <Html lang="en">
      <Head>
        <Title>KSOX - Dashboard</Title>
        <Meta charset="utf-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1" />
        <Meta name="theme-color" content="#0F0D12" />
        <Meta
          name="description"
          content="KSOX dashboard check your badges and engagement rate."
        />
        <Meta
          name="keywords"
          content="KSOX, Crypto, decentralized exchange, zk-STARKs, cryptocurrency exchange, buy, sell, trade, transaction fees, arbitrage, transaction processing, Token Crowdsale, KSXT, KSX, investor, crowdfunding, secure, seamless, crypto payments, badges, engagement, dashboard"
        />
        <Link rel="icon" href="/gfx/logo.svg" />
      </Head>
      <Body>
        <Suspense>
          <ErrorBoundary>
            <NavProvider>
              <WalletProvider projectId={projectId} alchemyId={alchemyId}>
                <Routes>
                  <Route path="/" component={Index}>
                    <Route
                      path={["/"]}
                      element={<App />}
                      preload={() => setNav(Nav.App)}
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
