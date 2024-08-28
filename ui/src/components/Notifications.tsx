import * as React from 'react';
import { Txt, Alert } from 'rendition';

export const Notifications = ({
	hasAvailableNetworks,
	attemptedConnect,
	RestartingApp,
	error,
}: {
	hasAvailableNetworks: boolean;
	attemptedConnect: boolean;
	RestartingApp: boolean;
	error: string;
}) => {
	return (
		<div>
			{attemptedConnect && (
				<Alert m={2} info>
					<Txt.span style={{ display: 'block' }}>
						Applying changes...
						<br />
						Your device will soon be online. If connection is unsuccessful, the
						Data Hub WiFi will be back up in a few seconds, and reloading this
						page will allow you to try again.
					</Txt.span>
				</Alert>
			)}
			{RestartingApp && (
				<Alert m={2} info>
					<Txt.span style={{ display: 'block' }}>
						Reloading WiFi network list...
						<br />
						The Data Hub WiFi will be back up in a few seconds, reload this page
						to show updated WiFi network list.
					</Txt.span>
				</Alert>
			)}
			{!hasAvailableNetworks && (
				<Alert m={2} warning>
					<Txt.span style={{ display: 'block' }}>
						No WiFi network available.
						<br />
						Please ensure your WiFi network is within range, then press the button 'Reload WiFi network list'.
					</Txt.span>
				</Alert>
			)}
			{!!error && (
				<Alert m={2} warning>
					<Txt.span style={{ display: 'block' }}>
						{error}
						<br />
						Reconnect to the Data Hub WiFi and try again. If the issue persists contact technical support.
					</Txt.span>
				</Alert>
			)}
		</div>
	);
};
