use wire_weaver::ww_api;
use wire_weaver_client_common::CommandSender;

pub struct IndicationClient<F, E> {
    args_scratch: [u8; 512],
    cmd_tx: CommandSender<F, E>,
}

ww_api!(
    "../ww_indication/src/lib.rs" as ww_indication::Indication for IndicationClient,
    client = true,
    no_alloc = false,
    use_async = true,
    //derive = "Debug",
    debug_to_file = "./target/ww_no_alloc.rs"
);
