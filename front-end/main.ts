import { ApiPromise, WsProvider, Keyring } from "@polkadot/api"
import "@polkadot/api-augment"
import type { FrameSystemAccountInfo } from "@polkadot/types/lookup"


const WEB_SOCKET = 'ws://127.0.0.1:9944'

const connect = async () => {
  const wsProvider = new WsProvider(WEB_SOCKET)
  const api: any = await ApiPromise.create({ provider: wsProvider })
  await api.isReady
  return api
}

const getConst = async (api: ApiPromise) => {
  const consts = await api.consts.balances.existentialDeposit.toHuman()
  console.log('consts', consts)
  return consts
}

const getFreeBalance  = async (api: ApiPromise, address: string) => {
  const {data: { free }}: FrameSystemAccountInfo = await api.query.system.account(address)
  return free
}

const subscribe  = async (api: ApiPromise, address: string) => {
  const {data: { free }}: FrameSystemAccountInfo = await api.query.system.events(events => {
    events.forEach(event => {
      console.log('index', event['event']['index'].toHuman())
      console.log('data', event['event']['data'].toHuman())
    });
  })
  return free
}

const subscribeEvent  = async (api: ApiPromise, address: string) => {
  const {data: { free }}: FrameSystemAccountInfo = await api.query.system.account(address, alice => {
    console.log('address', address)
    console.log('alice', alice.data.free)
  })
  return free
}

const getMetadata = async (api: ApiPromise) => {
  const metadata = await api.rpc.state.getMetadata()
  return metadata.toString()
}

const main =async () => {
  console.log('main function')
  const api = await connect()
  const deposit = getConst(api)
  console.log('deposit', deposit)

  const keyring = new Keyring({ type: 'sr25519' })
  const alice = keyring.addFromUri('//Alice') 
  const bob = keyring.addFromUri('//Bob') 
  const alice_free = getFreeBalance(api, alice.address)
  console.log('alice_free', alice_free)

  // await transfer(api, alice, bob.address, 10**10 + 1)

  const bob_free = getFreeBalance(api, bob.address)
  console.log('bob_free', bob_free)

  console.log('metadata', await getMetadata(api)) 

  subscribe(api, alice.address)
  subscribeEvent(api, alice.address)
}

main().then(() => {
  console.log('main function finished')
  process.exit(0)
}).catch((e) => {
  console.log('main function error', e)
  process.exit(1)
})