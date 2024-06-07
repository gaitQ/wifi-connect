FROM debian:latest

# Install dependencies
RUN apt-get update && apt-get install -y \
	dnsmasq \
	network-manager \
	wget \
	curl \
	wireless-tools \
	&& rm -rf /var/lib/apt/lists/*

RUN dbus-uuidgen > /var/lib/dbus/machine-id

ENV DBUS_SESSION_BUS_ADDRESS=unix:path=/run/user/1000/bus
ENV DBUS_SYSTEM_BUS_ADDRESS=unix:path=/host/run/dbus/system_bus_socket

# Expose the default port used by WiFi Connect
EXPOSE 80

# copy prebuild UI, wifi-connect program and start script
COPY target/release/wifi-connect /usr/src/app/
COPY start-wifi-connect.sh /usr/src/app/
COPY ui/build /usr/src/app/ui

WORKDIR /usr/src/app/
CMD ["bash", "start-wifi-connect.sh"]
