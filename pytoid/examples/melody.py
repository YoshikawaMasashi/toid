import time

import toid

if __name__ == "__main__":
    player = toid.LocalPlayer()
    player.resource_register("../resource/sf2/sf2.toml")
    player.load_sf2("sf2.test")
    portaudio_outputter = toid.PortAudioOutputter(player.get_toid_player())

    player.set_sf2_name("sf2.test")

    # player.send_num_lang("12345 643 2 1", 0.0, "main",)
    # player.send_num_lang("1   4   5   1", -1.0, "sub",)
    player['main'] = "12345 643 2 1"
    player['sub'] = "1   4   5   1", -1.0

    portaudio_outputter.run()
    time.sleep(12)
    portaudio_outputter.stop()
