import React from "react";
import ReactDOM from "react-dom/client";
import 'react-toastify/dist/ReactToastify.css';
import "./styles.css";
import './i18n';
import { RouterProvider } from "react-router-dom";
import { router } from "./routing/browserRouter";
import { ThemeProvider, createTheme } from "@mui/material";
import { ToastContainer } from "react-toastify";

const defaultTheme = createTheme();

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <ThemeProvider theme={defaultTheme}>
    <RouterProvider router={router} />
    <ToastContainer />
    </ThemeProvider>
  </React.StrictMode>,
);
