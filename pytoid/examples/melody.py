import time

import toid

if __name__ == "__main__":
    store = toid.MusicStateStore()
    sound_state_manager = toid.MusicStateManager(store)
    portaudio_outputter = toid.PortAudioOutputter(sound_state_manager)
    reducer = toid.Reducer(store)

    portaudio_outputter.run()

    toid.add_new_note_on(reducer, 60.0, 0 * (44100 // 4))
    toid.add_new_note_on(reducer, 62.0, 1 * (44100 // 4))
    toid.add_new_note_on(reducer, 64.0, 2 * (44100 // 4))
    toid.add_new_note_on(reducer, 65.0, 3 * (44100 // 4))
    toid.add_new_note_on(reducer, 67.0, 4 * (44100 // 4))
    toid.add_new_note_on(reducer, 69.0, 6 * (44100 // 4))
    toid.add_new_note_on(reducer, 65.0, 7 * (44100 // 4))
    toid.add_new_note_on(reducer, 64.0, 8 * (44100 // 4))
    toid.add_new_note_off(reducer, 9 * (44100 // 4))
    toid.add_new_note_on(reducer, 62.0, 10 * (44100 // 4))
    toid.add_new_note_off(reducer, 11 * (44100 // 4))
    toid.add_new_note_on(reducer, 60.0, 12 * (44100 // 4))

    time.sleep(4)
    portaudio_outputter.stop()
