// Copyright 2020 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

// (Re-)generated by schema tool
//////// DO NOT CHANGE THIS FILE! ////////
// Change the json schema instead

use fairroulette::*;
use schema::*;
use wasmlib::*;

mod fairroulette;
mod schema;
mod types;

#[no_mangle]
fn on_load() {
    let exports = ScExports::new();
    exports.add_func(FUNC_LOCK_BETS, func_lock_bets_thunk);
    exports.add_func(FUNC_PAY_WINNERS, func_pay_winners_thunk);
    exports.add_func(FUNC_PLACE_BET, func_place_bet_thunk);
    exports.add_func(FUNC_PLAY_PERIOD, func_play_period_thunk);
}

pub struct FuncLockBetsParams {}

fn func_lock_bets_thunk(ctx: &ScFuncContext) {
    // only SC itself can invoke this function
    ctx.require(ctx.from(&ctx.contract_id().as_agent_id()), "no permission");

    let params = FuncLockBetsParams {};
    func_lock_bets(ctx, &params);
}

pub struct FuncPayWinnersParams {}

fn func_pay_winners_thunk(ctx: &ScFuncContext) {
    // only SC itself can invoke this function
    ctx.require(ctx.from(&ctx.contract_id().as_agent_id()), "no permission");

    let params = FuncPayWinnersParams {};
    func_pay_winners(ctx, &params);
}

pub struct FuncPlaceBetParams {
    pub number: ScImmutableInt, // the number a better bets on
}

fn func_place_bet_thunk(ctx: &ScFuncContext) {
    let p = ctx.params();
    let params = FuncPlaceBetParams {
        number: p.get_int(PARAM_NUMBER),
    };
    ctx.require(params.number.exists(), "missing mandatory number");
    func_place_bet(ctx, &params);
}

pub struct FuncPlayPeriodParams {
    pub play_period: ScImmutableInt, // number of minutes in one playing round
}

fn func_play_period_thunk(ctx: &ScFuncContext) {
    // only SC creator can update the play period
    ctx.require(ctx.from(&ctx.contract_creator()), "no permission");

    let p = ctx.params();
    let params = FuncPlayPeriodParams {
        play_period: p.get_int(PARAM_PLAY_PERIOD),
    };
    ctx.require(params.play_period.exists(), "missing mandatory playPeriod");
    func_play_period(ctx, &params);
}
