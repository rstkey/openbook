#![no_main]

use arbitrary::Arbitrary;
use libfuzzer_sys::{fuzz_target, Corpus};
use openbook_v2_fuzz::{FuzzContext, UserId};

#[derive(Debug, Arbitrary, Clone)]
struct FuzzData {
    instructions: Vec<FuzzInstruction>,
}

#[derive(Debug, Arbitrary, Clone)]
enum FuzzInstruction {
    PlaceOrder {
        user_id: UserId,
        data: openbook_v2::instruction::PlaceOrder,
    },
    PlaceOrderPegged {
        user_id: UserId,
        data: openbook_v2::instruction::PlaceOrderPegged,
    },
    PlaceTakeOrder {
        user_id: UserId,
        data: openbook_v2::instruction::PlaceTakeOrder,
    },
}

fuzz_target!(|fuzz_data: FuzzData| -> Corpus { run_fuzz(fuzz_data) });

fn run_fuzz(fuzz_data: FuzzData) -> Corpus {
    let mut corpus = Corpus::Keep;
    if fuzz_data.instructions.is_empty() {
        return Corpus::Reject;
    }

    let mut ctx = FuzzContext::new();
    ctx.initialize();

    for fuzz_instruction in fuzz_data.instructions {
        let has_valid_inputs = match fuzz_instruction {
            FuzzInstruction::PlaceOrder { user_id, data } => ctx
                .place_order(user_id, data)
                .map_or_else(error_filter::place_order, |_| true),

            FuzzInstruction::PlaceOrderPegged { user_id, data } => ctx
                .place_order_pegged(user_id, data)
                .map_or_else(error_filter::place_order_pegged, |_| true),

            FuzzInstruction::PlaceTakeOrder { user_id, data } => ctx
                .place_take_order(user_id, data)
                .map_or_else(error_filter::place_take_order, |_| true),
        };

        if !has_valid_inputs {
            corpus = Corpus::Reject;
        };
    }

    corpus
}

mod error_filter {
    use openbook_v2::error::OpenBookError;
    use solana_program::program_error::ProgramError;
    use spl_token::error::TokenError;

    pub fn place_order(err: ProgramError) -> bool {
        match err {
            e if e == OpenBookError::InvalidInputLots.into() => false,
            e if e == OpenBookError::InvalidInputPriceLots.into() => false,
            e if e == OpenBookError::InvalidOrderSize.into() => true,
            e if e == OpenBookError::OpenOrdersFull.into() => true,
            e if e == OpenBookError::WouldSelfTrade.into() => true,
            e if e == TokenError::InsufficientFunds.into() => true,
            _ => panic!("{}", err),
        }
    }

    pub fn place_order_pegged(err: ProgramError) -> bool {
        match err {
            e if e == OpenBookError::InvalidInputLots.into() => false,
            e if e == OpenBookError::InvalidInputPegLimit.into() => false,
            e if e == OpenBookError::InvalidInputPriceLots.into() => false,
            e if e == OpenBookError::InvalidInputStaleness.into() => false,
            e if e == OpenBookError::InvalidOrderPostIOC.into() => true,
            e if e == OpenBookError::InvalidOrderPostMarket.into() => true,
            e if e == OpenBookError::InvalidOrderSize.into() => true,
            e if e == OpenBookError::InvalidPriceLots.into() => true,
            e if e == OpenBookError::WouldSelfTrade.into() => true,
            e if e == TokenError::InsufficientFunds.into() => true,
            _ => panic!("{}", err),
        }
    }

    pub fn place_take_order(err: ProgramError) -> bool {
        match err {
            e if e == OpenBookError::InvalidInputLots.into() => false,
            e if e == OpenBookError::InvalidInputOrderType.into() => false,
            e if e == OpenBookError::InvalidInputPriceLots.into() => false,
            e if e == OpenBookError::InvalidOrderSize.into() => true,
            e if e == TokenError::InsufficientFunds.into() => true,
            _ => panic!("{}", err),
        }
    }
}
