import toid


if __name__ == '__main__':
    print("please input ip (ex. 127.0.0.1):")
    ip = input()
    print(ip)
    connect_address = "{}:3012".format(ip)
    print("connect_address: ws://{}".format(connect_address))

    server = toid.WebSocketPlayerServer(connect_address)
