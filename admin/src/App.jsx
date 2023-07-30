import {
  Route,
  RouterProvider,
  createBrowserRouter,
  createRoutesFromElements,
} from "react-router-dom";
import Root from "./routes/Root";
import ContestList, {
  action as contestListAction,
  loader as contestListLoader,
} from "./routes/ContestList";
import ContestDetail, {
  action as contestDetailAction,
  loader as contestDetailLoader,
} from "./routes/ContestDetail";
import GraphList, {
  action as graphListAction,
  loader as graphListLoader,
} from "./routes/GraphList";
import GraphDetail, {
  action as graphDetailAction,
  loader as graphDetailLoader,
} from "./routes/GraphDetail";

const router = createBrowserRouter(
  createRoutesFromElements(
    <Route element={<Root />}>
      <Route
        path="/"
        element={<ContestList />}
        action={contestListAction}
        loader={contestListLoader}
      />
      <Route
        path="/contests/:contestName/detail"
        element={<ContestDetail />}
        action={contestDetailAction}
        loader={contestDetailLoader}
      />
      <Route
        path="/contests/:contestName/graphs"
        element={<GraphList />}
        action={graphListAction}
        loader={graphListLoader}
      />
      <Route
        path="/contests/:contestName/graphs/:graphName/detail"
        element={<GraphDetail />}
        action={graphDetailAction}
        loader={graphDetailLoader}
      />
    </Route>,
  ),
);

export default function App() {
  return <RouterProvider router={router} />;
}
