import "bulma/css/bulma.css";
import { UserProvider, useUser } from "@auth0/nextjs-auth0/client";
import Link from "next/link";
import { useRouter } from "next/router";
import { QueryClient, QueryClientProvider } from "@tanstack/react-query";
import Head from "next/head";
import { useState } from "react";

function Nav() {
  const [active, setActive] = useState(false);
  const { user } = useUser();
  const router = useRouter();
  return (
    <nav className="navbar is-link">
      <div className="container">
        <div className="navbar-brand">
          <span className="navbar-item">Graph Drawing Contests</span>
          <button
            className={`navbar-burger${active ? " is-active" : ""}`}
            aria-label="menu"
            aria-expanded="false"
            onClick={() => {
              setActive(!active);
            }}
          >
            <span aria-hidden="true"></span>
            <span aria-hidden="true"></span>
            <span aria-hidden="true"></span>
          </button>
        </div>
        <div className={`navbar-menu${active ? " is-active" : ""}`}>
          <div className="navbar-end">
            {user ? (
              <Link
                href={`/api/auth/logout?returnTo=${router.asPath}`}
                className="navbar-item"
              >
                Logout
              </Link>
            ) : (
              <Link
                href={`/api/auth/login?returnTo=${router.asPath}`}
                className="navbar-item"
              >
                Login
              </Link>
            )}
          </div>
        </div>
      </div>
    </nav>
  );
}

const queryClient = new QueryClient();

export default function App({ Component, pageProps }) {
  return (
    <>
      <Head>
        <meta name="viewport" content="width=device-width, initial-scale=1" />
      </Head>
      <QueryClientProvider client={queryClient}>
        <UserProvider>
          <header>
            <Nav />
          </header>
          <main className="container">
            <section className="section">
              <Component {...pageProps} />
            </section>
          </main>
          <footer className="footer">
            <p className="has-text-centered">&copy; 2023 VDSLab</p>
          </footer>
        </UserProvider>
      </QueryClientProvider>
    </>
  );
}
