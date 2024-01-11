import { ZirisClient } from '../src/client'
import ZirisCMDWallet from '../src/wallet'
import * as dotenv from 'dotenv'
dotenv.config()

describe('Integrity Test (Testnet)', () => {
	let secret = process.env.ADMIN_SECRET || ''
	const cmdWallet = new ZirisCMDWallet({ network: 'testnet', secret: secret })

	let client = new ZirisClient({
		network: 'testnet',
		wallet: cmdWallet.wallet(),
	})

	const contract = client.getContract()

	test('test create series', async () => {
		const createSeries = await contract?.createSeries({
			creator: (await cmdWallet.getAccount()).publicKey,
			uri: 'test-uri',
			base_price: BigInt(100),
		})

		await createSeries?.signAndSend()
	}, 20000)

	test('buy series', async () => {
		const buy = await contract?.buy({
			buyer: (await cmdWallet.getAccount()).publicKey,
			series_id: BigInt(1),
		})

		await buy?.signAndSend()
	}, 20000)

	test('get current series price', async () => {
		const info = await contract?.seriesInfo({
			series_id: BigInt(1),
		})

		expect(info?.result?.price).toBeGreaterThanOrEqual(BigInt(1))
	}, 20000)

	test('claim reward', async () => {
		const buy = await contract?.buy({
			buyer: (await cmdWallet.getAccount()).publicKey,
			series_id: BigInt(1),
		})

		await buy?.signAndSend()

		const shareBalance = await contract?.shareBalance({
			account: (await cmdWallet.getAccount()).publicKey,
		})

		expect(shareBalance?.result).toBeDefined()
		expect(shareBalance?.result).toBeGreaterThanOrEqual(BigInt(0))

		const claimBalance = await contract?.claimShare({
			account: (await cmdWallet.getAccount()).publicKey,
		})

		await claimBalance?.signAndSend()
	}, 20000)
})
