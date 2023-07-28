import "bulma/css/bulma.css";
import { UserProvider, useUser } from "@auth0/nextjs-auth0/client";
import Link from "next/link";
import { useRouter } from "next/router";

function Nav() {
  const { user } = useUser();
  const router = useRouter();
  return (
    <nav className="navbar is-link">
      <div className="container">
        <div className="navbar-brand">
          <span className="navbar-item">Graph Drawing Contests</span>
        </div>
        <div className="navbar-menu">
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

export default function App({ Component, pageProps }) {
  return (
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
  );
}
