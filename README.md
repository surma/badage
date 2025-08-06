# badage

A simple command-line decryption tool that allows you to decrypt files encrypted with [age](https://github.com/FiloSottile/age) by providing the encryption passphrase via a commandline flag.

**This is bad. Don’t use it unles you know why you are doing it.**

## Usage

Decrypt a file:

```bash
badage --passphrase "your-passphrase" --input encrypted.age --output decrypted.txt
```

## Compatibility

`badage` should be fully compatible with files encrypted using the standard `age` tool.

## License

Apache 2.0
