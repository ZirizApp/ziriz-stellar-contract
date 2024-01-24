import { ClassOptions, Contract, Wallet, networks } from 'contract-client'

export type ZirizClientProps = {
	network: 'testnet' | 'mainnet'
	wallet?: Wallet
}

interface NetworkConfig {
	contractId: string
	networkPassphrase: string
}

export class ZirizClient {
	private contract: Contract | null = null

	constructor({ network, wallet }: ZirizClientProps) {
		const networkConfig: NetworkConfig =
			'mainnet' in networks && network == 'mainnet'
				? (networks.mainnet as NetworkConfig)
				: (networks.testnet as NetworkConfig)
		const config: ClassOptions =
			network == 'testnet' || 'mainnet' in networks
				? {
						...networkConfig,
						rpcUrl: 'https://soroban-testnet.stellar.org',
						wallet,
				  }
				: {
						...networkConfig,
						rpcUrl: 'https://soroban.stellar.org',
						wallet,
				  }
		this.contract = new Contract(config)
	}

	setContract(contract: Contract) {
		this.contract = contract
	}

	getContract() {
		return this.contract
	}
}
