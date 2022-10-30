# melwalletd-prot

Crate defining the [nanorpc] (JSON-RPC 2.0) protocol exposed by melwalletd, the reference Themelio wallet microservice implementation.

As a user, you almost certainly only care about [MelwalletdClient], which you would generally want to wrap around an HTTP-based [nanorpc::RpcTransport] in order to talk to melwalletd.

License: ISC
