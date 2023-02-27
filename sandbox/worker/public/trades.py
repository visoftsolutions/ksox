from sseclient import SSEClient
from worker.public.models import *

URL = "http://localhost:7979/api/public/trades"

messages = SSEClient(URL, json=Request("b89ac651-e3ef-4902-9549-f3d29b582233", "f96083a1-c5d5-4a1b-809b-0fa4eca0b051").__dict__)
for msg in messages:
    print(msg)
