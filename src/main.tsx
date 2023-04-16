import React from "react";
import ReactDOM from "react-dom/client";
import "./styles.css";
import SettingsPage from "./feature/settings/components/SettingsPage";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <SettingsPage />
  </React.StrictMode>
);

// 右クリックで表示されるデフォルトコンテキストメニューの無効化
document.addEventListener('contextmenu', event => event.preventDefault());