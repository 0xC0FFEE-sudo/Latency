use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    pubkey,
    pubkey::Pubkey,
    system_program, sysvar,
};
use spl_associated_token_account::get_associated_token_address;

pub const PUMP_FUN_PROGRAM_ID: Pubkey = pubkey!("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P");
const GLOBAL_ACCOUNT: Pubkey = pubkey!("4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf");
const FEE_RECIPIENT: Pubkey = pubkey!("CebN5WGQ4g2ffEvz7ErmyrssS4U5H2K2sbLqdePs3KNw");
const MPL_TOKEN_METADATA_PROGRAM: Pubkey = pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");
const EVENT_AUTHORITY: Pubkey = pubkey!("Ce6TQqeHC9E8K2eYJikG9gvuGcsgLqpoN1gRrgbAsFGB");

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct Buy {
    discriminator: [u8; 8],
    amount: u64,
    max_sol_cost: u64,
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct Sell {
    discriminator: [u8; 8],
    amount: u64,
    min_sol_output: u64,
}

pub fn get_buy_instruction(
    user: &Pubkey,
    mint_str: &str,
    token_amount: u64,
    sol_amount: u64,
) -> Result<Instruction, Box<dyn std::error::Error + Send + Sync>> {
    let mint = mint_str.parse::<Pubkey>()?;

    let (bonding_curve, _) =
        Pubkey::find_program_address(&[b"bonding-curve", mint.as_ref()], &PUMP_FUN_PROGRAM_ID);

    let associated_bonding_curve = get_associated_token_address(&bonding_curve, &mint);
    let associated_user = get_associated_token_address(user, &mint);

    let accounts = vec![
        AccountMeta::new_readonly(GLOBAL_ACCOUNT, false),
        AccountMeta::new(FEE_RECIPIENT, false),
        AccountMeta::new_readonly(mint, false),
        AccountMeta::new(bonding_curve, false),
        AccountMeta::new(associated_bonding_curve, false),
        AccountMeta::new(associated_user, false),
        AccountMeta::new(*user, true),
        AccountMeta::new_readonly(system_program::id(), false),
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
        AccountMeta::new_readonly(EVENT_AUTHORITY, false),
        AccountMeta::new_readonly(PUMP_FUN_PROGRAM_ID, false),
    ];

    // Discriminator for "buy" instruction
    let instruction_data = Buy {
        discriminator: [0xf2, 0x32, 0xb1, 0x34, 0x84, 0x0d, 0x85, 0xa3], // sha256("instruction:buy")[..8]
        amount: token_amount,
        max_sol_cost: sol_amount,
    };

    let data = instruction_data.try_to_vec()?;

    Ok(Instruction {
        program_id: PUMP_FUN_PROGRAM_ID,
        accounts,
        data,
    })
}

pub fn get_sell_instruction(
    user: &Pubkey,
    mint_str: &str,
    token_amount: u64,
    min_sol_output: u64,
) -> Result<Instruction, Box<dyn std::error::Error + Send + Sync>> {
    let mint = mint_str.parse::<Pubkey>()?;

    let (bonding_curve, _) =
        Pubkey::find_program_address(&[b"bonding-curve", mint.as_ref()], &PUMP_FUN_PROGRAM_ID);

    let associated_bonding_curve = get_associated_token_address(&bonding_curve, &mint);
    let associated_user = get_associated_token_address(user, &mint);

    let accounts = vec![
        AccountMeta::new_readonly(GLOBAL_ACCOUNT, false),
        AccountMeta::new(FEE_RECIPIENT, false),
        AccountMeta::new_readonly(mint, false),
        AccountMeta::new(bonding_curve, false),
        AccountMeta::new(associated_bonding_curve, false),
        AccountMeta::new(associated_user, false),
        AccountMeta::new(*user, true),
        AccountMeta::new_readonly(system_program::id(), false),
        AccountMeta::new_readonly(spl_associated_token_account::id(), false),
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new_readonly(EVENT_AUTHORITY, false),
        AccountMeta::new_readonly(PUMP_FUN_PROGRAM_ID, false),
    ];

    // Discriminator for "sell" instruction
    let instruction_data = Sell {
        discriminator: [0x33, 0xe6, 0x85, 0xa4, 0x01, 0x7f, 0xc1, 0x6e], // sha256("instruction:sell")[..8]
        amount: token_amount,
        min_sol_output,
    };

    let data = instruction_data.try_to_vec()?;

    Ok(Instruction {
        program_id: PUMP_FUN_PROGRAM_ID,
        accounts,
        data,
    })
} 