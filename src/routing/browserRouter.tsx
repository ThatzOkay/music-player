import { createBrowserRouter } from "react-router-dom";
import App from "../App";
import FirstRun from "../pages/firstRun";

export const router = createBrowserRouter([
    {
      path: "/",
      element: <App />
    },
    {
        path: "/firstrun",
        element: <FirstRun />
    }
  ])