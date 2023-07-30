import { Outlet } from "react-router-dom";

export default function Root() {
  return (
    <>
      <header>
        <section className="hero is-small is-primary">
          <div className="hero-body">
            <div className="container">
              <p className="title">Graph Drawing Contest Admin</p>
            </div>
          </div>
        </section>
      </header>
      <div className="container">
        <Outlet />
      </div>
      <footer className="footer">
        <p className="has-text-centered">&copy; 2023 Yosuke Onoue</p>
      </footer>
    </>
  );
}
