use std::collections::HashMap;

use anchor_client::ClientError;

use anchor_lang::__private::bytemuck;

use openbook_v2::state::{Market, MarketIndex, OpenOrdersAccountValue, TokenIndex};

use fixed::types::I80F48;
use futures::{stream, StreamExt, TryStreamExt};
use itertools::Itertools;

use crate::gpa::*;

use solana_client::nonblocking::rpc_client::RpcClient as RpcClientAsync;
use solana_sdk::account::Account;
use solana_sdk::instruction::AccountMeta;
use solana_sdk::pubkey::Pubkey;

pub struct MarketContext {
    pub address: Pubkey,
    pub market: Market,
}

pub struct OpenBookContext {
    pub markets: HashMap<MarketIndex, MarketContext>,
    pub market_indexes_by_name: HashMap<String, MarketIndex>,
}

impl OpenBookContext {
    pub fn context(&self, market_index: MarketIndex) -> &MarketContext {
        self.markets.get(&market_index).unwrap()
    }

    pub fn market_address(&self, market_index: MarketIndex) -> Pubkey {
        self.context(market_index).address
    }

    pub async fn new_from_rpc(rpc: &RpcClientAsync) -> anyhow::Result<Self> {
        let program = openbook_v2::ID;

        // markets
        let market_tuples = fetch_markets(rpc, program).await?;
        let markets = market_tuples
            .iter()
            .map(|(pk, pm)| {
                (
                    pm.market_index,
                    MarketContext {
                        address: *pk,
                        market: *pm,
                    },
                )
            })
            .collect::<HashMap<_, _>>();

        // Name lookup tables
        let market_indexes_by_name = markets
            .iter()
            .map(|(i, p)| (p.market.name().to_string(), *i))
            .collect::<HashMap<_, _>>();

        Ok(OpenBookContext {
            markets,
            market_indexes_by_name,
        })
    }

    pub async fn new_markets_listed(&self, rpc: &RpcClientAsync) -> anyhow::Result<bool> {
        let new_markets = fetch_markets(rpc, openbook_v2::id()).await?;
        Ok(new_markets.len() > self.markets.len())
    }
}

async fn fetch_raw_account(rpc: &RpcClientAsync, address: Pubkey) -> Result<Account, ClientError> {
    rpc.get_account_with_commitment(&address, rpc.commitment())
        .await?
        .value
        .ok_or(ClientError::AccountNotFound)
}
