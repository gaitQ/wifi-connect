# wifi connect
Taken from: `https://github.com/balena-os/wifi-connect`

- Build docker image:
	```
	docker build -t wifi-connect .
	```
- Run container:
  	```
	docker run -d --restart always --network=host --privileged -v /var/run/dbus/system_bus_socket:/host/run/dbus/system_bus_socket wifi-connect
	```
- Make sure firewall allows port 80

## TODO
- The WiFi card/driver does not support scanning in AP mode, which means that `wifi-connect` can only search for available APs once before starting the AP.
- We need a way for `wifi-connect` to timeout or a button to stop the program to allow rescanning
- Add connection check for `wifi-connect` in case an ethernet connection has been added while the AP is active