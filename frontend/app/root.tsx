import {
  isRouteErrorResponse,
  Links,
  Meta,
  Outlet,
  Scripts,
  ScrollRestoration,
} from "react-router";

import type { Route } from "./+types/root";
import "@fontsource-variable/noto-sans";
import "./app.css";

export const links: Route.LinksFunction = () => [];

export function Layout({ children }: { children: React.ReactNode }) {
  return (
    <html lang="en">
      <head>
        <meta charSet="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <Meta />
        <Links />
      </head>
      <body className="h-screen overflow-hidden">
        {children}
        <ScrollRestoration />
        <Scripts />
      </body>
    </html>
  );
}

export default function App() {
  return <Outlet />;
}

export function HydrateFallback() {
  return (
    <div className="flex h-screen items-center justify-center">
      <div className="glass-panel rounded-2xl px-10 py-8 text-center">
        <div className="mb-1 bg-gradient-to-r from-violet-400 to-cyan-400 bg-clip-text text-3xl font-extrabold tracking-tight text-transparent">
          QCTidy
        </div>
        <p className="text-sm text-muted-foreground">Loading circuit editor…</p>
      </div>
    </div>
  );
}

export function ErrorBoundary({ error }: Route.ErrorBoundaryProps) {
  let message = "Something went wrong";
  let details = "An unexpected error occurred.";
  let stack: string | undefined;

  if (isRouteErrorResponse(error)) {
    message = error.status === 404 ? "404" : `Error ${error.status}`;
    details =
      error.status === 404
        ? "The requested page could not be found."
        : error.statusText || details;
  } else if (import.meta.env.DEV && error && error instanceof Error) {
    details = error.message;
    stack = error.stack;
  }

  return (
    <main className="mx-auto flex min-h-screen max-w-xl flex-col justify-center gap-4 p-6">
      <h1 className="text-3xl font-bold tracking-tight">{message}</h1>
      <p className="text-muted-foreground">{details}</p>
      {stack && (
        <pre className="overflow-x-auto rounded-xl border border-border bg-black/40 p-4 text-xs text-muted-foreground">
          <code>{stack}</code>
        </pre>
      )}
    </main>
  );
}
