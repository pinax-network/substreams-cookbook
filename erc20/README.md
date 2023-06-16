# ERC-20 Substreams

## References
- [Ethereum Docs: ERC-20 Token Standard](https://ethereum.org/en/developers/docs/standards/tokens/erc-20/)
- [EIPS: ERC-20 Token Standard ](https://eips.ethereum.org/EIPS/eip-20)
- [OpenZeppelin implementation](https://github.com/OpenZeppelin/openzeppelin-contracts/blob/9b3710465583284b8c4c5d2245749246bb2e0094/contracts/token/ERC20/ERC20.sol)
- [ConsenSys implementation](https://github.com/ConsenSys/Tokens/blob/fdf687c69d998266a95f15216b1955a4965a0a6d/contracts/eip20/EIP20.sol)

### Methods

- [ ] `name` - Returns the name of the token - e.g. "MyToken".
  > function name() public view returns (string)
- [ ] `symbol` - Returns the symbol of the token. E.g. "HIX".
  > function symbol() public view returns (string)
- [ ] `decimals` - Returns the number of decimals the token uses - e.g. `8`, means to divide the token amount by `100000000` to get its user representation.
  > function decimals() public view returns (uint8)
- [ ] `totalSupply` - Returns the total token supply.
  > function totalSupply() public view returns (uint256)
- [ ] `balanceOf` - Returns the account balance of another account with address `_owner`.
  > function balanceOf(address _owner) public view returns (uint256 balance)
- [ ] `transfer` - Transfers _value amount of tokens to address `_to`, and MUST fire the `Transfer` event.
  > function transfer(address _to, uint256 _value) public returns (bool success)
- [ ] `transferFrom` - Transfers _value amount of tokens from address `_from` to address `_to`, and MUST fire the `Transfer` event.
  > function transferFrom(address _from, address _to, uint256 _value) public returns (bool success)
- [ ] `approve` - Allows `_spender` to withdraw from your account multiple times, up to the `_value` amount. If this function is called again it overwrites the current allowance with `_value`.
  > function approve(address _spender, uint256 _value) public returns (bool success)
- [ ] `allowance` - Returns the amount which `_spender` is still allowed to withdraw from `_owner`.
  > function allowance(address _owner, address _spender) public view returns (uint256 remaining)

### Events

- [ ] `Transfer` - MUST trigger when tokens are transferred, including zero value transfers.
  > event Transfer(address indexed _from, address indexed _to, uint256 _value)
- [ ] `Approval` - MUST trigger on any successful call to `approve(address _spender, uint256 _value)`.
  > event Approval(address indexed _owner, address indexed _spender, uint256 _value)
