use std::io;

fn main() -> io::Result<()> {
    let mut cfg = tonic_build::configure();
    #[cfg(feature = "client")]
    {
        cfg = cfg.build_client(true).build_server(false);
    }

    #[cfg(feature = "server")]
    {
        cfg = cfg.build_server(true);
    }

    cfg.compile_protos(
        &[
            "protos/orders.proto",
            "protos/txncb.proto",
            "protos/cabal.proto",
            "protos/common.proto",
            "protos/copytrade.proto",
        ],
        &["protos"],
    )
}
