## Decipher `method` calls

> EVM `calldata` has a minimum of 36 bytes of raw data.
>
> The first 4 bytes is the method selector.
>
> The rest of the input data are method arguments in chunks of 32 bytes.

```rust
for calls in block.calls() {
  let input = calls.call.clone().input;
  if input.len() < 36 { continue; } // skip if not 36 bytes
  let method: String = Hex::encode(&input[0..4]);
  // => method
  //    0xa9059cbb => transfer(address,uint256)
  //    0x23b872dd => transferFrom(address,address,uint256)
}
```

### Convert `method` function to `SHA-3` method selector

> First 4 bytes of the SHA-3 hash
>
> Each byte is represented by 2 characters in hex string

**Typescript**
```typescript
import { utils } from 'web3';

function stringToSha3(str: string) {
  return utils.sha3(str)?.slice(0, 10)
}
console.log(stringToSha3("transfer(address,uint256)"))
// => 0xa9059cbb
```

### References

- [How To Decipher A Smart Contract Method Call
](https://medium.com/@hayeah/how-to-decipher-a-smart-contract-method-call-8ee980311603)