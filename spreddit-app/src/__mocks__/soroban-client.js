const xdr = {
  LedgerKey: { contractData: () => ({}) },
  LedgerKeyContractData: function(){},
  LedgerEntryData: { fromXDR: () => ({}) },
  ScVal: { scvSymbol: () => ({}), scvObject: () => ({}), scvI32: () => ({}) },
  ScObject: { scoBytes: () => ({}) },
  LedgerFootprint: { fromXDR: () => ({}) },
};

class Server {
  async getLedgerEntry() { throw new Error('mock'); }
  async getAccount() { return { id: 'G', sequence: '1' }; }
  async simulateTransaction() { return { footprint: '' }; }
  async sendTransaction() { return { status: 'PENDING' }; }
}
class Contract { call() { return {}; } }
class Account { constructor(id, seq){ this.id=id; this.sequence=seq; } }
class TransactionBuilder {
  constructor(){ }
  addOperation(){ return this; }
  setTimeout(){ return this; }
  build(){ return { sign(){}, toXDR(){return ''}, operations:[{function:{},parameters:[]}], sequence:'2' }; }
}
const Operation = { invokeHostFunction: (x)=>x };
const Keypair = { fromSecret: ()=>({ publicKey: ()=> 'G' }) };
const Networks = { FUTURENET: 'Test SDF Future Network ; October 2022' };

module.exports = { xdr, Server, Contract, Account, TransactionBuilder, Operation, Keypair, Networks, TimeoutInfinite: 0 };
