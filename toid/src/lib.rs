//! toidはlive codingツールを提供します。
//! パフォーマンス向上のため、toidはrustで実装されていますが、
//! 通常はPython3などのスクリプト言語にバインディングして使うことを想定しています。

pub mod data;
pub mod music_state;
pub mod outputters;
pub mod player;
pub mod resource_management;
pub mod state_management;
// pub mod stores;

#[cfg(test)]
mod tests;
