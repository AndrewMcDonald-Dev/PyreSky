# Create python script to test receiving messages from HTTP/2 Websocket

import asyncio
import websockets

async def main():
    async with websockets.connect("ws://localhost:8080/ws") as websocket:
        while True:
            message = await websocket.recv()
            print(message)

asyncio.run(main())
