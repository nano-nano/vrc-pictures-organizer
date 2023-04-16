import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./styles.css";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);

// 右クリックで表示されるデフォルトコンテキストメニューの無効化
document.addEventListener('contextmenu', event => event.preventDefault());