import time

import toid

if __name__ == "__main__":
    store = toid.MusicStore()
    wave_reader = toid.WaveReader(store)
    portaudio_outputter = toid.PortAudioOutputter(wave_reader)

    store.new_melody("main")
    store.new_melody("sub")

    store.load_and_set_sf2("../florestan-subset.sf2")

    main_melody_store = store.get_melody("main")
    sub_melody_store = store.get_melody("sub")

    portaudio_outputter.run()

    main_melody_store.add_note(48.0, 1 * (44100 // 4), 0 * (44100 // 4))
    main_melody_store.add_note(50.0, 1 * (44100 // 4), 1 * (44100 // 4))
    main_melody_store.add_note(52.0, 1 * (44100 // 4), 2 * (44100 // 4))
    main_melody_store.add_note(53.0, 1 * (44100 // 4), 3 * (44100 // 4))
    main_melody_store.add_note(55.0, 2 * (44100 // 4), 4 * (44100 // 4))
    main_melody_store.add_note(57.0, 1 * (44100 // 4), 6 * (44100 // 4))
    main_melody_store.add_note(53.0, 1 * (44100 // 4), 7 * (44100 // 4))
    main_melody_store.add_note(52.0, 1 * (44100 // 4), 8 * (44100 // 4))
    main_melody_store.add_note(50.0, 1 * (44100 // 4), 10 * (44100 // 4))
    main_melody_store.add_note(48.0, 4 * (44100 // 4), 12 * (44100 // 4))

    sub_melody_store.add_note(36.0, 4 * (44100 // 4), 0 * (44100 // 4))
    sub_melody_store.add_note(41.0, 4 * (44100 // 4), 4 * (44100 // 4))
    sub_melody_store.add_note(43.0, 4 * (44100 // 4), 8 * (44100 // 4))
    sub_melody_store.add_note(36.0, 4 * (44100 // 4), 12 * (44100 // 4))

    time.sleep(16)
    portaudio_outputter.stop()
