import React from "react";
import ReactDOM from "react-dom/client";
import 'react-toastify/dist/ReactToastify.css';
import "./styles.css";
import './i18n';
import { RouterProvider } from "react-router-dom";
import { router } from "./routing/browserRouter";
import { ToastContainer } from "react-toastify";
import { ChakraProvider } from '@chakra-ui/react'

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <ChakraProvider>
      <RouterProvider router={router} />
      <ToastContainer />
    </ChakraProvider>
  </React.StrictMode>,
);
