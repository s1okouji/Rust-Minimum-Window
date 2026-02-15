# README

## 必要なクレート

コンパイル時間削減のためfeaturesで機能を絞るのが一般的

```toml
windows = { version = "0.62.2", features = [
    "Win32_Foundation",
    "Win32_Graphics_Gdi",
    "Win32_UI_WindowsAndMessaging",
    "Win32_System_LibraryLoader",
] }
```

## ウィンドウ作成の手順

1. インスタンスハンドルの取得をする
1. ウィンドウクラスの定義
1. ウィンドウクラスの登録
1. ウィンドウの作成
1. メッセージループ

## ウィンドウプロシージャ

基本的にCのAPIを使うときと同じように書ける。  
メッセージ名もCのAPIと同じくアッパースネークケースなので、`#[allow(non_snake_case)]`を付与するのが良い
