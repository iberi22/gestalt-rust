import sys
import json
import logging

# Configure logging to stderr
logging.basicConfig(level=logging.DEBUG, stream=sys.stderr, format='%(asctime)s - %(levelname)s - %(message)s')

def main():
    logging.info("Mock MCP server started")
    while True:
        try:
            line = sys.stdin.readline()
            if not line:
                logging.info("Stdin closed, exiting")
                break

            logging.debug(f"Received: {line.strip()}")
            req = json.loads(line)

            # Check if it's a request (has id) or notification (no id)
            msg_id = req.get("id")
            method = req.get("method")

            if msg_id is not None:
                # It's a request
                if method == "initialize":
                    resp = {
                        "jsonrpc": "2.0",
                        "id": msg_id,
                        "result": {
                            "protocolVersion": "2024-11-05", # Use a stable version
                            "capabilities": {},
                            "serverInfo": {"name": "mock-server", "version": "0.1.0"}
                        }
                    }
                    logging.debug(f"Sending initialize response: {resp}")
                    sys.stdout.write(json.dumps(resp) + "\n")
                    sys.stdout.flush()
                elif method == "tools/list":
                    resp = {
                        "jsonrpc": "2.0",
                        "id": msg_id,
                        "result": {
                            "tools": [
                                {"name": "echo", "description": "Echoes back", "inputSchema": {"type": "object"}}
                            ]
                        }
                    }
                    logging.debug(f"Sending tools/list response")
                    sys.stdout.write(json.dumps(resp) + "\n")
                    sys.stdout.flush()
                else:
                    resp = {
                        "jsonrpc": "2.0",
                        "id": msg_id,
                        "result": {}
                    }
                    logging.debug(f"Sending empty response for {method}")
                    sys.stdout.write(json.dumps(resp) + "\n")
                    sys.stdout.flush()
            else:
                # It's a notification
                logging.debug(f"Received notification: {method}")

        except json.JSONDecodeError as e:
            logging.error(f"Failed to decode JSON: {e}")
        except Exception as e:
            logging.error(f"Unexpected error: {e}")

if __name__ == "__main__":
    main()
