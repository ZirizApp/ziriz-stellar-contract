import { ZirizClient } from '../src/client'
import ZirizCMDWallet from '../src/wallet'
import * as dotenv from 'dotenv'
dotenv.config()

describe('Integrity Test (Testnet)', () => {
	let secret = process.env.ADMIN_SECRET || ''
	const cmdWallet = new ZirizCMDWallet({ network: 'testnet', secret: secret })

	let client = new ZirizClient({
		network: 'testnet',
		wallet: cmdWallet.wallet(),
	})

	const contract = client.getContract()

	test('test create series', async () => {
		const createSeries = await contract?.createSeries({
			creator: (await cmdWallet.getAccount()).publicKey,
			uri: 'test-uri',
			base_price: BigInt(10 ** 7),
			creator_curve: BigInt(1**7),
			fan_base_price: BigInt(10 ** 7),
			fan_decay_rate: BigInt(900),
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

		expect(info?.result?.price).toBeGreaterThanOrEqual(BigInt(10 ** 7))
	}, 20000)

	test('claim reward', async () => {
		const buy = await contract?.buy({
			buyer: (await cmdWallet.getAccount()).publicKey,
			series_id: BigInt(1),
		})

		await buy?.signAndSend()

		const shareBalance = await contract?.shareBalance({
			account: (await cmdWallet.getAccount()).publicKey,
			series_id: BigInt(1),
		})

		expect(shareBalance?.result).toBeDefined()
		expect(shareBalance?.result).toBeGreaterThanOrEqual(BigInt(0))

		const claimBalance = await contract?.claimShare({
			account: (await cmdWallet.getAccount()).publicKey,
			series_id: BigInt(1),
		})

		await claimBalance?.signAndSend()
	}, 20000)
})
