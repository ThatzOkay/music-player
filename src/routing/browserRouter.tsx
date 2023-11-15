import { createBrowserRouter } from "react-router-dom";
import App from "../App";
import FirstRun from "../pages/firstRun";
import FirstRunRouter from "../firstRunRouter";

export const router = createBrowserRouter([
    {
      path: "/",
      element: <FirstRunRouter />
    },
    {
      path: "/home",
      element: <App />
    },
    {
        path: "/firstrun",
        element: <FirstRun />
    }
  ])