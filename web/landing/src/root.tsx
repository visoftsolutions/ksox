// @refresh reload
import { Suspense } from "solid-js";
import { Body, ErrorBoundary, FileRoutes, Head, Html, Link, Meta, Route, Routes, Scripts, Title } from "solid-start";
import "./root.css";
import Index from "./routes";
import { Nav, setNav } from "./utils/providers/NavProvider";
import Landing from "./components/Landing";
import Token from "./components/Token";
import { joinPaths } from "solid-start/islands/server-router";

export const base = import.meta.env.BASE_URL;

export default function Root() {
  return (
    <Html lang="en">
      <Head>
        <Title>KSOX - Cryptocurrency Exchange & Payment Processor</Title>
        <Meta charset="utf-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1" />
        <Meta name="theme-color" content="#0F0D12" />
        <Meta
          name="description"
          content="KSOX - Cryptocurrency Exchange & Payment Processor"
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
            <Routes>
                <Route path="/" component={Index}>
                  <Route path={"/"} element={<Landing />} preload={() => setNav(Nav.Landing)} />
                  <Route path="/token" element={<Token />} preload={() => setNav(Nav.Token)} />
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
