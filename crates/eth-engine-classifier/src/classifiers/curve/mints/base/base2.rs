use reaper_eth_engine_macros::action_impl;
use reaper_eth_engine_pricing::Protocol;
use reaper_eth_engine_types::{
    normalized_actions::NormalizedMint, structured_trace::CallInfo, ToScaledRational,
};

action_impl!(
    Protocol::CurveBasePool2,
    crate::CurveBase2::add_liquidityCall,
    Mint,
    [..AddLiquidity],
    logs: true,
    |
    info: CallInfo,
    log: CurveBasePool2Add_liquidityCallLogs,
    db_tx: &DB
    |{
        let log = log.add_liquidity_field?;

        let details = db_tx.get_protocol_details(info.target_address)?;

        let amounts = log.token_amounts;
        let (tokens, token_amts): (Vec<_>, Vec<_>) = details.into_iter()
.enumerate().map(|(i, t)|
        {
            let token = db_tx.try_fetch_token_info(t)?;
            let decimals = token.decimals;
            Ok((token, amounts[i].to_scaled_rational(decimals)))
        }
        ).collect::<eyre::Result<Vec<_>>>()?.into_iter().unzip();

        Ok(NormalizedMint {
            protocol: Protocol::CurveBasePool2,
            trace_index: info.trace_idx,
            pool: info.target_address,
            from: info.from_address,
            recipient: info.from_address,
            token: tokens,
            amount: token_amts,
        })

    }
);
