pub mod melody_state;
pub mod scheduling_state;

struct MusicState {
    scheduling: scheduling_state::SchedulingState,
    melody: melody_state::MelodyState,
}
