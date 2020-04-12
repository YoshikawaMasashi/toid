import time

import toid

if __name__ == "__main__":
    store = toid.MusicStore()
    resource_manager = toid.ResourceManager()
    resource_manager.register("../resource/sf2/sf2.toml")
    resource_manager.load_sf2("sf2.test")

    wave_reader = toid.WaveReader(store, resource_manager)

    player = toid.LocalPlayer(store, resource_manager)
    portaudio_outputter = toid.PortAudioOutputter(wave_reader)

    player.set_sf2_name("sf2.test")

    player.send_num_lang(
        "12345 643 2 1",
        0.0,
        "main",
    )

    player.send_num_lang(
        "1   4   5   1",
        -1.0,
        "sub",
    )

    portaudio_outputter.run()
    time.sleep(12)
    portaudio_outputter.stop()
