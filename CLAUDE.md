# CLAUDE.md

このファイルは、リポジトリ内のコードを扱う際に Claude Code (claude.ai/code) へ提供するガイダンス。

## アプリの概要

VRC Pictures Organizer は、Tauri 2 + React で作られた Windows デスクトップ常駐アプリ。
`Pictures\VRChat` フォルダ内の VRChat スクリーンショットを日付別サブフォルダ（`YYYY-MM-DD`）に自動整理する。
設定可能なインターバルでバックグラウンド動作し、「日付境界線」（デフォルト 12:00）によって深夜セッションを適切に処理する。

## コマンド

```bash
# フロントエンド開発サーバー（Tauri と併用）
npm run dev

# ホットリロード付きフル開発ビルド（Tauri が Vite をラップ）
npm run tauri dev

# プロダクションビルド
npm run build          # TypeScript チェック + Vite バンドル
npm run tauri build    # Tauri アプリ本体 + インストーラー（WIX）

# Lint
npm run lint           # ESLint（レポートのみ、自動修正なし）
```

テストフレームワークは設定しない。

## アーキテクチャ

### フロントエンド（`src/`）

Mantine v7 を使用したシングルページ React アプリ。実質的な機能は `src/features/app-settings/` のみで、設定ウィンドウとログビューアを描画する。

- `bindings.ts` — specta/tauri-specta が Rust から**自動生成**するファイル。手動編集禁止。`tauri dev` / `tauri build` 時に再生成される。
- `features/app-settings/hooks/useSettings.ts` — Tauri IPC 経由で設定を読み書き
- `features/app-settings/hooks/useAppLog.ts` — Rust バックエンドからプロセスログをポーリング取得

### バックエンド（`src-tauri/src/`）

| ファイル | 役割 |
|---|---|
| `lib.rs` | Tauri アプリのセットアップ、トレイアイコン、ウィンドウ管理、整理スレッドの起動 |
| `organization.rs` | メインループ：`Pictures\VRChat` をスキャン、ファイルメタデータのタイムスタンプ取得、日付境界ロジック適用、ファイル移動 |
| `settings.rs` | OS のアプリ設定ディレクトリへ JSON で設定を読み書き |
| `notifications.rs` | 処理実行後の Windows トースト通知 |

### IPC コントラクト

フロントエンド↔バックエンド間の通信はすべて自動生成の `bindings.ts` を使用。Rust が公開する Tauri コマンドは以下の通り：
- `loadSettingsFromFile()` / `saveSettingsToFile(settings)`
- `fetchAppProcessLog()`

新しい Tauri コマンドを追加する際は `#[specta::specta]` アノテーションを付けて `lib.rs` に登録すること。次のビルド時にバインディングが自動再生成される。

### アプリの状態管理

整理スレッドはウィンドウとは独立して動作する。ウィンドウを非表示にしてもバックグラウンドループは継続する。
UI から変更した設定はディスクに保存され、整理スレッドの次のインターバルタイミングで反映される。

## 主要な設定値

- ウィンドウサイズ：400×300、初期状態は非表示（トレイアイコンから開く）
- デフォルト監視インターバル：60 秒（10〜3600 秒で設定可能）
- デフォルト日付境界線：12:00
- Tauri ターゲット：Windows のみ（WIX インストーラー、日本語ロケール `ja-JP`）
- Vite 開発サーバー：ポート 1420

## Rust に関する注意事項

- Rust エディション：2021
- 非同期ランタイム：tokio（full features）
- エラーハンドリング：全体を通して `anyhow` を使用
- 日付境界線の計算を含むすべての日時処理は `chrono` が担当
