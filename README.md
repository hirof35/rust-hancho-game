# Rust版 半丁ゲーム (Hancho Game)

Siv3Dで作成した「丁半（半丁）博打」のゲームを、Rustの軽量ゲームフレームワーク `macroquad` を用いて移植した作品です。
<img width="1110" height="786" alt="スクリーンショット 2026-05-19 074037" src="https://github.com/user-attachments/assets/22c0583f-a5b1-4515-9fbf-f63799c2e724" />
## 特徴
- Rustの強力な `enum` を活用した状態管理
- `macroquad` によるシンプルな2D描画とUI
- 日本語フォント（Windowsのメイリオ等）の読み込みに対応

## 実行方法
```bash
cargo run
