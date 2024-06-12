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
		<>
			{attemptedConnect && (
				<Alert m={2} info>
					<Txt.span>Applying changes... </Txt.span>
					<Txt.span>
						Your device will soon be online. If connection is unsuccessful, the
						Access Point will be back up in a few seconds, and reloading this
						page will allow you to try again.
					</Txt.span>
				</Alert>
			)}
			{RestartingApp && (
				<Alert m={2} info>
					<Txt.span>Reloading WiFi list... </Txt.span>
					<Txt.span>
						The Access Point will be back up in a few seconds, reload this page
						to show updated WiFi list.
					</Txt.span>
				</Alert>
			)}
			{!hasAvailableNetworks && (
				<Alert m={2} warning>
					<Txt.span>No wifi networks available.&nbsp;</Txt.span>
					<Txt.span>
						Please ensure there is a network within range and press the button
						"Reload WiFi List".
					</Txt.span>
				</Alert>
			)}
			{!!error && (
				<Alert m={2} danger>
					<Txt.span>{error}</Txt.span>
				</Alert>
			)}
		</>
	);
};
