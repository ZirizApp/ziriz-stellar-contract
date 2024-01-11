import { ZirisClient } from './client'
import * as dotenv from 'dotenv'
import ZirisCMDWallet from './wallet'
import { AxiosError } from 'axios'
dotenv.config()

const main = async () => {
	let secret = process.env.ADMIN_SECRET || ''
	const cmdWallet = new ZirisCMDWallet({ network: 'testnet', secret: secret })
	let client = new ZirisClient({
		network: 'testnet',
		wallet: cmdWallet.wallet(),
	})

	try {
		const tx = await client.getContract()?.initialize({
			name: 'Soribium-X',
			symbol: 'SRBX',
			admin: process.env.ADMIN || '',
			native_token: process.env.NATIVE_TOKEN || '',
		})

		tx?.signAndSend()
	} catch (e) {
		const error = e as AxiosError
		if (error.response?.data) {
			console.log(JSON.stringify(error.response?.data))
		}
	}
}

main()
