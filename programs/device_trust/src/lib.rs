programs/device_trust/src/lib.rs

```rust
use anchor_lang::prelude::*;

declare_id!("DTrust111111111111111111111111111111111111");

#[program]
pub mod device_trust {
    use super::*;

    // Initialize the global state (only needed once)
    pub fn initialize(ctx: Context<Initialize>, authority: Pubkey) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.authority = authority;
        state.total_attestations = 0;
        Ok(())
    }

    // Record a device trust attestation. Only the designated authority (daemon) can call this.
    pub fn attest_device(ctx: Context<AttestDevice>, device_id: [u8; 32], score: u8) -> Result<()> {
        let state = &mut ctx.accounts.state;
        let attestation = &mut ctx.accounts.attestation;

        // Authority check
        require_keys_eq!(state.authority, ctx.accounts.authority.key(), ErrorCode::Unauthorized);

        // Update global counter
        state.total_attestations += 1;

        // Store attestation
        attestation.device_owner = ctx.accounts.device_owner.key();
        attestation.device_id = device_id;
        attestation.score = score;
        attestation.timestamp = Clock::get()?.unix_timestamp;

        emit!(DeviceAttested {
            device_owner: attestation.device_owner,
            device_id,
            score,
        });
        Ok(())
    }

    // Allow a user to request an attestation reset (for re‑evaluation)
    pub fn request_reset(ctx: Context<RequestReset>, _device_id: [u8; 32]) -> Result<()> {
        let attestation = &mut ctx.accounts.attestation;
        require_keys_eq!(attestation.device_owner, ctx.accounts.device_owner.key(), ErrorCode::Unauthorized);
        attestation.score = 0;   // reset score, daemon will re‑attest
        Ok(())
    }
}

// --------- Accounts ---------

#[account]
pub struct GlobalState {
    pub authority: Pubkey,
    pub total_attestations: u64,
}

#[account]
pub struct DeviceAttestation {
    pub device_owner: Pubkey,
    pub device_id: [u8; 32],
    pub score: u8,
    pub timestamp: i64,
}

// --------- Contexts ---------

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = payer, space = 8 + 32 + 8)]
    pub state: Account<'info, GlobalState>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AttestDevice<'info> {
    #[account(mut, has_one = authority)]
    pub state: Account<'info, GlobalState>,
    #[account(
        init_if_needed,
        payer = authority,
        space = 8 + 32 + 32 + 1 + 8,
        seeds = [b"attest", device_owner.key().as_ref()],
        bump
    )]
    pub attestation: Account<'info, DeviceAttestation>,
    pub authority: Signer<'info>,
    /// CHECK: device owner is not required to sign, only identified
    pub device_owner: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RequestReset<'info> {
    #[account(mut, has_one = device_owner)]
    pub attestation: Account<'info, DeviceAttestation>,
    pub device_owner: Signer<'info>,
}

// --------- Events ---------

#[event]
pub struct DeviceAttested {
    pub device_owner: Pubkey,
    pub device_id: [u8; 32],
    pub score: u8,
}

// --------- Errors ---------

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized")]
    Unauthorized,
}
```

