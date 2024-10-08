import type { JSONSchema7 as JSONSchema } from 'json-schema';
import * as React from 'react';
import type { RenditionUiSchema } from 'rendition';
import { Flex, Form, Heading, Button } from 'rendition';
import type { Network, NetworkInfo } from './App';

const getSchema = (availableNetworks: Network[]): JSONSchema => ({
	type: 'object',
	properties: {
		ssid: {
			title: 'WiFi network name (SSID)',
			type: 'string',
			default: availableNetworks[0]?.ssid,
			oneOf: availableNetworks.map((network) => ({
				const: network.ssid,
				title: network.ssid,
			})),
		},
		identity: {
			title: 'User',
			type: 'string',
			default: '',
		},
		passphrase: {
			title: 'Password',
			type: 'string',
			default: '',
		},
	},
	required: ['ssid'],
});

const getUiSchema = (isEnterprise: boolean): RenditionUiSchema => ({
	ssid: {
		'ui:placeholder': 'Select WiFi Network',
		'ui:options': {
			emphasized: true,
		},
	},
	identity: {
		'ui:options': {
			emphasized: true,
		},
		'ui:widget': !isEnterprise ? 'hidden' : undefined,
	},
	passphrase: {
		'ui:widget': 'password',
		'ui:options': {
			emphasized: true,
		},
	},
});

const isEnterpriseNetwork = (
	networks: Network[],
	selectedNetworkSsid?: string,
) => {
	return networks.some(
		(network) =>
			network.ssid === selectedNetworkSsid && network.security === 'enterprise',
	);
};

interface NetworkInfoFormProps {
	availableNetworks: Network[];
	onSubmit: (data: NetworkInfo) => void;
	onRestartButtonClick: () => void;
}

export const NetworkInfoForm = ({
	availableNetworks,
	onSubmit,
	onRestartButtonClick,
}: NetworkInfoFormProps) => {
	const [data, setData] = React.useState<NetworkInfo>({});

	const isSelectedNetworkEnterprise = isEnterpriseNetwork(
		availableNetworks,
		data.ssid,
	);

	return (
		<Flex
			flexDirection="column"
			alignItems="center"
			justifyContent="center"
			m={4}
			mt={5}
		>
			<Heading.h2 align="center" mb={4}>
				Please choose your WiFi network from the list
			</Heading.h2>
			<Heading.h3 align="center" mb={4}>
				If your WiFi network is not shown in the list, ensure your router is turned on
				and in range. Then Press the button 'Reload WiFi network list'. You will
				have to reconnect to the Data Hub WiFi again (gaitq-data-hub-*).
			</Heading.h3>
			<Button
				onClick={onRestartButtonClick}
				mb={4}
				style={{
					background: '#e5554f',
					color: 'white',
					border: '0',
				}}
			>
				Reload WiFi network list
			</Button>

			<Form
				width={['100%', '80%', '60%', '40%']}
				onFormChange={({ formData }) => {
					setData(formData);
				}}
				onFormSubmit={({ formData }) => onSubmit(formData)}
				value={data}
				schema={getSchema(availableNetworks)}
				uiSchema={getUiSchema(isSelectedNetworkEnterprise)}
				submitButtonProps={{
					width: '60%',
					mx: '20%',
					mt: 3,
					disabled: availableNetworks.length <= 0,
					style: {
						background: '#e5554f',
						opacity: availableNetworks.length > 0 ? '1' : '0.6',
						color: 'white',
					},
				}}
				submitButtonText={'Connect'}
			/>
		</Flex>
	);
};
