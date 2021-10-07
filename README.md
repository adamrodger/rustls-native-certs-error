This sample app reproduces a problem using `rustls-native-certs` when the OS store contains an invalid
certificate. It parses each cert the way `rustls-native-certs` would and also using `x509-parser`. The
parsed certs are logged so that you can compare to the OS store and see which of them is the invalid
one.

To run:

```bash
RUST_LOG=trace cargo run --quiet
```

On my machine, this cert causes `rustls-native-certs` to fail entirely (instead of, for example, skipping
that invalid cert):

```
-----BEGIN CERTIFICATE-----
MIIB3TCCAUagAwIBAgIQNy/WjTnu4KBEfyDeF6mOLjANBgkqhkiG9w0BAQUFADAt
MSswKQYDVQQDHiIAQwBPAFIAUABcAHMAcgB2AC0AYgB1AGkAbABkAC0AYwBkMB4X
DTE5MDUwMTE1NTMyMFoXDTIwMDQzMDIxNTMyMFowLTErMCkGA1UEAx4iAEMATwBS
AFAAXABzAHIAdgAtAGIAdQBpAGwAZAAtAGMAZDCBnzANBgkqhkiG9w0BAQEFAAOB
jQAwgYkCgYEAtFWjgGjMbSVOCMCWHury61UJZBbESsoM+39j7OBEj8oqDACZW2qG
g9fFpXYcKRinHCb6Xte6YQDs5MxWQAQWLYXyGIHGu+drkS5YioyMo9M4LPIH4h+e
0cDBUW9vHCkM5xguWWRMysgIPqhV0Gly8RRxx8qyCurS4cGjZcSvU/0CAwEAATAN
BgkqhkiG9w0BAQUFAAOBgQBPkkAOKAqCdrIiPELB2qRC67GjVtcNG9jQUBDVPv2g
l1f3/V/nJDa3GKufhb+b3allHmcS+p1ZOgyNC75BuPabFGfIbUw3HgVaCUFExI9k
3aECHiAiX1UtC60H0UyUQOaIoet80ciDwzBg6ou2DNKLp9m5PKYxsnr5ye9+ODlo
7g==
-----END CERTIFICATE-----
```
