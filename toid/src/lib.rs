//! toidはlive codingツールを提供します。
//! パフォーマンス向上のため、toidはrustで実装されていますが、
//! 通常はPython3などのスクリプト言語にバインディングして使うことを想定しています。

pub mod collaboration;
pub mod music_state_manager;
pub mod portaudio_outputter;
pub mod serialize;
pub mod state_management;
pub mod states;

#[cfg(test)]
mod tests;
