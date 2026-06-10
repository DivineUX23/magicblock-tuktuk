use anchor_lang::{InstructionData, prelude::*};
use anchor_lang::solana_program::instruction::Instruction;
use tuktuk_program::{
    compile_transaction,
    tuktuk::{
        cpi::{accounts::QueueTaskV0, queue_task_v0},
        program::Tuktuk,
        types::TriggerV0,
    },
    types::QueueTaskArgsV0,
    TransactionSourceV0,
};

use crate::state::UserAccount;

#[derive(Accounts)]
pub struct Schedule<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"user", user.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(mut)]
    /// CHECK: complete here
    pub oracle_queue: UncheckedAccount<'info>,

    #[account(mut)]
    /// CHECK: use in cpi only
    pub task_queue: UncheckedAccount<'info>,

    /// CHECK: use in cpi only
    pub task_queue_authority: UncheckedAccount<'info>,

    /// CHECK: in cpi
    #[account(mut)]
    pub task: UncheckedAccount<'info>,

    /// CHECK: queue auth
    #[account(
        mut,
        seeds = [b"queue_authority"],
        bump
    )]
    pub queue_authority: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,

    pub tuktuk_program: Program<'info, Tuktuk>,
}


impl<'info> Schedule<'info> {
    pub fn schedule(&mut self, task_id: u16, bumps: ScheduleBumps) -> Result<()> {
        let (compiled_tx, _) = compile_transaction(
            vec![Instruction {
                program_id: crate::ID,
                accounts: crate::__cpi_client_accounts_outside_rand_vrf::OutsideRandVrf {
                    payer: self.user.to_account_info(),
                    user_account: self.user_account.to_account_info(),
                    oracle_queue: self.oracle_queue.to_account_info(), 
                    program_identity: todo!(), 
                    vrf_program: todo!(), 
                    slot_hashes: todo!(), 
                    system_program: todo!() 
                }
                .to_account_metas(None)
                .to_vec(),
                data: crate::instruction::CreateRand { user_seed: 0 }.data(),
            }], 
            vec![],
        )
        .unwrap();

        queue_task_v0(
            CpiContext::new_with_signer(
                self.tuktuk_program.to_account_info(),
                QueueTaskV0 {
                    payer: self.user.to_account_info(),
                    queue_authority: self.queue_authority.to_account_info(),
                    task_queue: self.task_queue.to_account_info(),
                    task_queue_authority: self.task_queue_authority.to_account_info(),
                    task: self.task.to_account_info(),
                    system_program: self.system_program.to_account_info(),
                },
                &[&["queue_authority".as_bytes(), &[bumps.queue_authority]]],
            ),
            QueueTaskArgsV0 { 
                id: task_id,
                trigger: TriggerV0::Now, 
                transaction: TransactionSourceV0::CompiledV0(compiled_tx), 
                crank_reward: Some(1000001), 
                free_tasks: 0, 
                description: "vrf".to_string(), 
            },
        )?;

        Ok(())
    }
}