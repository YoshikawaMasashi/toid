//! toidはlive codingツールを提供します。
//! パフォーマンス向上のため、toidはrustで実装されていますが、
//! 通常はPython3などのスクリプト言語にバインディングして使うことを想定しています。

pub mod state;

#[cfg(test)]
mod tests;
