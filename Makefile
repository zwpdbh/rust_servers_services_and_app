
TCP_SERVER_PORT := 8000 
# For ch02_tcp_server
ch02_server:
	cargo build && cargo run -- server --port $(TCP_SERVER_PORT)
ch02_client:
	cargo build && cargo run -- client --port $(TCP_SERVER_PORT)