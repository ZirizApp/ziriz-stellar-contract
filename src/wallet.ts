import { Wallet } from 'contract-client'
import * as StellarSdk from 'stellar-sdk'

export type ZirizWalletProps = {
	network: 'testnet' | 'mainnet'
	secret: string
}
class ZirizCMDWallet implements Wallet {
	private network: 'testnet' | 'mainnet' = 'testnet'
	private pubKey
	private keypair: StellarSdk.Keypair
	private server: StellarSdk.Horizon.Server
	public static Wallet: ZirizCMDWallet
	constructor({ network, secret }: ZirizWalletProps) {
		this.network = network
		this.keypair = StellarSdk.Keypair.fromSecret(secret)
		this.pubKey = this.keypair.publicKey()
		if (network == 'testnet') {
			this.server = new StellarSdk.Horizon.Server(
				'https://horizon-testnet.stellar.org'
			)
		} else {
			this.server = new StellarSdk.Horizon.Server('https://horizon.stellar.org')
		}
		this.server.loadAccount(this.pubKey)
		ZirizCMDWallet.Wallet = this
	}

	async getBalances() {
		const account = await this.server.loadAccount(this.pubKey)
		return account.balances
	}

	async setHomeDomain(domain: string) {
		const account = await this.server.loadAccount(this.pubKey)
		var transaction = new StellarSdk.TransactionBuilder(account, {
			fee: '100',
			networkPassphrase: StellarSdk.Networks.TESTNET,
		})
			.addOperation(
				StellarSdk.Operation.setOptions({
					homeDomain: domain,
				})
			)
			// setTimeout is required for a transaction
			.setTimeout(100)
			.build()
		transaction.sign(this.keypair)
		await this.server.submitTransaction(transaction)
	}

	async getConnectionStatus() {
		return true
	}

	async getAllowance() {
		return true
	}

	async getAccount() {
		return {
			publicKey: this.pubKey,
			network: this.network,
		}
	}

	async signTx(
		tx: StellarSdk.Transaction,
		options: StellarSdk.Horizon.Server.SubmitTransactionOptions | undefined
	) {
		tx.sign(this.keypair)
		return tx.toEnvelope().toXDR('base64')
	}

	async signAuth() {
		return ''
	}

	isConnected() {
		return ZirizCMDWallet.Wallet.getConnectionStatus()
	}

	isAllowed() {
		return ZirizCMDWallet.Wallet.getAllowance()
	}

	getUserInfo() {
		return ZirizCMDWallet.Wallet.getAccount()
	}

	signTransaction(
		tx: string,
		opts?:
			| {
					network?: string | undefined
					networkPassphrase?: string | undefined
					accountToSign?: string | undefined
			  }
			| undefined
	) {
		if (!opts) {
			const defaultTransaction = new StellarSdk.Transaction(
				tx,
				this.network == 'testnet'
					? StellarSdk.Networks.TESTNET
					: StellarSdk.Networks.PUBLIC
			)
			return ZirizCMDWallet.Wallet.signTx(defaultTransaction, undefined)
		}

		const transaction = new StellarSdk.Transaction(
			tx,
			opts.networkPassphrase as string
		)
		return ZirizCMDWallet.Wallet.signTx(
			transaction,
			opts as StellarSdk.Horizon.Server.SubmitTransactionOptions
		)
	}
	signAuthEntry(
		entryXdr: string,
		opts?: { accountToSign?: string | undefined } | undefined
	) {
		return ZirizCMDWallet.Wallet.signAuth()
	}

	wallet() {
		return {
			isConnected: this.isConnected,
			isAllowed: this.isAllowed,
			getUserInfo: this.getUserInfo,
			signTransaction: this.signTransaction,
			signAuthEntry: this.signAuthEntry,
		}
	}
}

export default ZirizCMDWallet
