import toid

print("please input ip (ex. 127.0.0.1):")
ip = input()
print(ip)
connect_address = "ws://{}:3012".format(ip)

player = toid.WebSocketPlayer(connect_address)
player.resource_register("../resource/sf2/sf2.toml")
player.load_sf2("sf2.test")

portaudio_outputter = toid.PortAudioOutputter(player.get_toid_player())
portaudio_outputter.run()

player.set_sf2_name("sf2.test")

player.send_num_lang("12345 643 2 1", 0.0, "main",)