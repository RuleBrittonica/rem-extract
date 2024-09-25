import subprocess
import re
import threading

# Function to send a raw LSP request to rust-analyzer
def send_lsp_request(process, message):
    # Calculate the content length and construct the header
    content_length = len(message)
    header = f"Content-Length: {content_length}\r\n\r\n"
    # Send the header and the message with \r\n endings
    process.stdin.write(header.encode('utf-8') + message.encode('utf-8'))
    process.stdin.flush()

# Function to listen to rust-analyzer's stdout
def listen_to_stdout(process):
    buffer = b""
    while True:
        chunk = process.stdout.read(1)  # Read one byte at a time
        if chunk:
            buffer += chunk
            if b"\r\n\r\n" in buffer:
                # Once headers are complete, extract the Content-Length
                headers, body = buffer.split(b"\r\n\r\n", 1)
                content_length_match = re.search(b"Content-Length: (\d+)", headers)
                if content_length_match:
                    content_length = int(content_length_match.group(1))
                    # Read the exact number of bytes from the body
                    while len(body) < content_length:
                        body += process.stdout.read(content_length - len(body))
                    print(f"rust-analyzer full response: {body.decode('utf-8')}")
                    buffer = b""  # Reset buffer after processing the response

# Start rust-analyzer as a subprocess
process = subprocess.Popen(
    ['rust-analyzer'],
    stdin=subprocess.PIPE,
    stdout=subprocess.PIPE,
    stderr=subprocess.PIPE
)

# Start listening to stdout in a separate thread
listener_thread = threading.Thread(target=listen_to_stdout, args=(process,))
listener_thread.daemon = True
listener_thread.start()

# Minimal initialize request in LSP format with \r\n line endings
initialize_request = (
    '{\r\n'
    '  "jsonrpc": "2.0",\r\n'
    '  "id": 1,\r\n'
    '  "method": "initialize",\r\n'
    '  "params": {\r\n'
    '    "rootUri": null,\r\n'
    '    "capabilities": {},\r\n'
    '    "processId": null\r\n'
    '  }\r\n'
    '}\r\n'
)

# Send the initialize request to rust-analyzer
send_lsp_request(process, initialize_request)

# Keep the main thread alive so we can keep receiving responses
try:
    listener_thread.join()
except KeyboardInterrupt:
    process.terminate()
