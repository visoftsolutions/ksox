import { Suspense, lazy, onMount } from "solid-js";
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
import { WalletProvider } from "@web/components/providers/WalletProvider";
import HomeView from "./components/Views/HomeView";
import TransfersView from "./components/Views/TransfersView";
import { CurrencyProvider } from "./components/providers/CurrencyProvider";
import {
  DarkModeProvider,
  useDarkModeContext,
} from "./components/providers/DarkModeProvider";

export const base = import.meta.env.BASE_URL;
export const api = joinPaths(base, "/api");
export const projectId = import.meta.env.VITE_WALLET_CONNECT_PROJECT_ID;
export const alchemyId = import.meta.env.VITE_ALCHEMY_API_KEY;

const Index = lazy(() => import("~/routes"));
const App = lazy(() => import("./components/Views/HomeView"));

export default function Root() {

  return (
    <DarkModeProvider>
      <Html lang="en" class={useDarkModeContext().darkMode() ? "dark" : ""}>
        <Head>
          <Title>KSOX - Payment Processor</Title>
          <Meta charset="utf-8" />
          <Meta name="viewport" content="width=device-width, initial-scale=1" />
          <Meta name="theme-color" content="#0F0D12" />
          <Meta
            name="description"
            content="KSOX Payment Processor - Revolutionizing crypto payments with NFC technology for seamless fund transfers and secure transactions. Embrace financial freedom today!"
          />
          <Meta
            name="keywords"
            content="ksox, pay, transaction, decentralized, zk, zero knowledge, stark, nfc"
          />
          <Link rel="icon" href="/gfx/logo.svg" />
          <Link rel="apple-touch-icon" href="/pwa/apple-touch-icon.png" />
          {import.meta.env.PROD == true ? (
            <>
              <Link
                rel="manifest"
                href={joinPaths(base, "/manifest.webmanifest")}
              />
              <script src={joinPaths(base, "/registerSW.js")} />
            </>
          ) : (
            <></>
          )}
        </Head>
        <Body>
          <Suspense>
            <ErrorBoundary>
              <NavProvider>
                <CurrencyProvider>
                  <WalletProvider projectId={projectId} alchemyId={alchemyId}>
                    <Routes>
                      <Route path="/" component={Index}>
                        <Route
                          path="/"
                          element={<HomeView />}
                          preload={() => setNav(Nav.Home)}
                        />
                        <Route
                          path="/1"
                          element={<TransfersView />}
                          preload={() => setNav(Nav.Transfer)}
                        />
                        <Route
                          path="/2"
                          element={<HomeView />}
                          preload={() => setNav(Nav.Notifications)}
                        />
                        <Route
                          path="/3"
                          element={<HomeView />}
                          preload={() => setNav(Nav.Settings)}
                        />
                      </Route>
                      <FileRoutes />
                    </Routes>
                  </WalletProvider>
                </CurrencyProvider>
              </NavProvider>
            </ErrorBoundary>
          </Suspense>
          <Scripts />
        </Body>
      </Html>
    </DarkModeProvider>
  );
}
