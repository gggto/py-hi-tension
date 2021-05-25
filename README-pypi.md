# py-hi-tension, hi-tension for Python

`py-hi-tension` is a Python module designed for *basic* but *fast* network
communication between scientific applications, backed by
[`hi-tension`](https://github.com/gggto/hi-tension). The focus is on
transferring large unsized arrays of `f64` with maximum throughput and minimum
latency. More informations to be found in `hi-tension`'s [github
page](https://github.com/gggto/hi-tension).

## Usage

Using the library is quite simple:
```python
# Connect to a server
socket = HiTensionSocket.connect("127.0.0.1:34487")
# You can make such a server though the following line:
# socket = HiTensionSocket.accept("127.0.0.1:34487")

# Send a simple text message
socket.send(b"status\n")
# Receive a simple text message
text_data = socket.read()

# Send formatted data as simple text message
id = 1
msg = f"do_something {id + 1}\n"
socket.send(msg.encode())

# Send formatted data and receive a high tension message
msg = f"fly {id}\n"
socket.send(msg.encode())
array = socket.hiread() # array is a numpy array

# Send the array back as a high tension message
socket.hisend(array)
socket.hidelimiter()

# You can send the array back
socket.hisend(array)
socket.hidelimiter()

# You may send your data in multible packets
socket.hisend(array[:10])
socket.hisend(array[10:])
socket.hidelimiter()
# This is useful for example if you are calculating your data while
# transferring it.
```
