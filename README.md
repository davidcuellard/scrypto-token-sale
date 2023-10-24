# Scrypto project

Basic token sale created using Scrypto v1.0 - Radix DLT

## Available Scripts

In the project directory, you can run:

### `resim reset`

Reset simulator

### `resim new-account`

Create new account as default

### `resim publish .`

Publish the blueprint located in src > lib.rs

### `resim call-function <PACKAGE_ADDRESS> <BLUEPRINT_NAME> <FUNCTION_NAME>`

Call function, for example `resim call-function package_sim1pk3cmat8st4ja2ms8mjqy2e9ptk8y6cx40v4qnfrkgnxcp2krkpr92 TokenSale new 10`

### `resim call-function  <COMPONENT_ADDRESS> <METHOD_NAME>`

Call method, for example `resim call-method component_sim1cqcvt0x68n5f3saykchrh3gmhnqay7x5z82cr5vrqr87vz84z3p5k9 buy resource_sim1tknxxxxxxxxxradxrdxxxxxxxxx009923554798xxxxxxxxxakj8n3:100`

### `resim run test.rtm`

Run the manifest to withdraw the tokens

## Useful links about Radix and Scrypto

- https://docs.radixdlt.com/docs

- https://docs-babylon.radixdlt.com/main/index.html

## License

[MIT](https://choosealicense.com/licenses/mit/)
