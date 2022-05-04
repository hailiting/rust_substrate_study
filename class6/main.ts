import { ApiPromise, WsProvider } from "@polkadot/api";
const web_socket = "wss://westend-rpc.polkadot.io/";

const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));
async function main() {
  const connectSubstrate = async () => {
    const wsProvider = new WsProvider(web_socket);
    const api = await ApiPromise.create({
      provider: wsProvider,
      types: {},
    });
    await api.isReady;
    return api;
  };
  let a = 0;
  const api = await connectSubstrate();
  api.derive.chain.bestNumber((bestNumber) => {
    a = bestNumber.toNumber();
    console.log(a);
  });
  sleep(900000);
}
main();
