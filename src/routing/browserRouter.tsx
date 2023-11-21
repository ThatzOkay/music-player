import { createBrowserRouter } from "react-router-dom";
import FirstRun from "../pages/firstRun";
import FirstRunRouter from "../firstRunRouter";
import Home from "../pages/home";
import AddSubsonic from "../pages/addSubsonic";

export const router = createBrowserRouter([
    {
      path: "/",
      element: <FirstRunRouter />
    },
    {
      path: "/home",
      element: <Home />
    },
    {
        path: "/firstrun",
        element: <FirstRun />
    },
    {
      path: "/addSubsonic",
      element: <AddSubsonic />
    }
  ])