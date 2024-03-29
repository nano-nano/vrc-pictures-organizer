# VRChat Picture Organizer

VRChat のスクリーンショットを撮影年月日別のフォルダに仕分けするアプリ

![image](https://github.com/nano-nano/vrc-pictures-organizer/assets/29051777/efbc747f-737a-4d95-ac04-08ec848bc04b)

## これはなに？

VRChat Picture Organizer は、
VRChat で撮影した VR カメラやスクリーンショットの画像を、
ファイル名の撮影年月日別のフォルダに仕分けするアプリです。

### ファイルの仕分けについて

ファイルの作成日時（更新日時）と日付変更基準時（設定により変更可能。デフォルトは 12:00）を比較し、
基準時よりも前のファイルは前日とみなします。

次の例のように仕分けされます。（基準時＋ 12:00 設定）

| ファイル作成日時 | 分類されるフォルダ |
| ---------------- | ------------------ |
| 2023/04/30 23:59 | 2023/04/30         |
| 2023/05/01 00:01 | 2023/04/30         |
| 2023/05/01 11:59 | 2023/04/30         |
| 2023/05/01 12:00 | 2023/05/01         |

## 対応 OS

Windows のみ対応しています。

## ダウンロードリンク

[こちら](https://github.com/nano-nano/vrc_pictures_organizer/releases)からどうぞ

## 使い方

...ToDo...

## ライセンス

プログラム本体については [MIT License](https://github.com/tcnksm/tool/blob/master/LICENCE) です。

## 作者

Nano-Nano
[@nano2_aloerina](https://twitter.com/nano2_aloerina)

---

## 開発者向け情報

### 開発ツールのバージョン

- Node.js: v18.16.1
- npm: v9.5.1

- Tauri: v1.2.4

### 各種コマンド

```bash
# 依存関係のインストール
$ npm install

# 開発用モードで起動
$ npm run tauri dev

# リリース用ビルド
$ npm run tauri build
```
