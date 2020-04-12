//! toidはlive codingツールを提供します。
//! パフォーマンス向上のため、toidはrustで実装されていますが、
//! 通常はPython3などのスクリプト言語にバインディングして使うことを想定しています。

pub mod data;
pub mod high_layer_trial;
pub mod music_state;
pub mod outputters;
pub mod players;
pub mod resource_management;
pub mod state_management;

#[cfg(test)]
mod tests;
