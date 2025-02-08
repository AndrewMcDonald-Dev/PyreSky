import asyncio
import websockets

async def chat():
    async with websockets.connect('ws://localhost:8080/ws') as websocket:
        while True:
            message = await websocket.recv()
            print(message)

loop = asyncio.get_event_loop()
loop.run_until_complete(chat())
