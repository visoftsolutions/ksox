// @refresh reload
import { Suspense } from "solid-js";
import {
  Body,
  ErrorBoundary,
  FileRoutes,
  Head,
  Html,
  Link,
  Meta,
  Routes,
  Scripts,
  Title,
} from "solid-start";
import { joinPaths } from "solid-start/islands/server-router";
import "./root.css";

export const base = import.meta.env.BASE_URL;


export default function Root() {
  return (
    <Html lang="en">
      <Head>
        <Title>KSOX - Cryptocurrency Exchange & Payment Processor</Title>
        <Meta charset="utf-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1" />
        <Meta name="theme-color" content="#0F0D12" />
        <Link rel="icon" type="image/x-icon" href={joinPaths(base, "/favicon.ico")} />
        <Link rel="apple-touch-icon" href={joinPaths(base, "/pwa/apple-touch-icon.png")} />
        {import.meta.env.PROD == true ? (
          <>
            <Link rel="manifest" href={joinPaths(base, "/manifest.webmanifest")} />
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
              <FileRoutes />
            </Routes>
          </ErrorBoundary>
        </Suspense>
        <Scripts />
      </Body>
    </Html>
  );
}
