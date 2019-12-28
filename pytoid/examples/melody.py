import time

import toid

if __name__ == "__main__":
    store = toid.MusicStateStore()
    music_state_manager = toid.MusicStateManager(store)
    portaudio_outputter = toid.PortAudioOutputter(music_state_manager)
    reducer = music_state_manager.get_reducer()

    portaudio_outputter.run()

    reducer.add_new_note_on(60.0, 0 * (44100 // 4))
    reducer.add_new_note_on(62.0, 1 * (44100 // 4))
    reducer.add_new_note_on(64.0, 2 * (44100 // 4))
    reducer.add_new_note_on(65.0, 3 * (44100 // 4))
    reducer.add_new_note_on(67.0, 4 * (44100 // 4))
    reducer.add_new_note_on(69.0, 6 * (44100 // 4))
    reducer.add_new_note_on(65.0, 7 * (44100 // 4))
    reducer.add_new_note_on(64.0, 8 * (44100 // 4))
    reducer.add_new_note_off(9 * (44100 // 4))
    reducer.add_new_note_on(62.0, 10 * (44100 // 4))
    reducer.add_new_note_off(11 * (44100 // 4))
    reducer.add_new_note_on(60.0, 12 * (44100 // 4))

    time.sleep(4)
    portaudio_outputter.stop()
