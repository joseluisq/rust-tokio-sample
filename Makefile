server:
	@echo "TCP server is listening on 127.0.0.1:6142"
	@socat TCP-LISTEN:6142,fork stdout
.PHONY: server
