use solana_sdk::{transaction::TransactionError, instruction::InstructionError};

fn encode(error: TransactionError) -> String {
    hex::encode(bincode::serialize(&error).unwrap())
}

fn main() {
    println!("TransactionError::AccountInUse 0x{}", encode(TransactionError::AccountInUse));
    println!("TransactionError::AccountLoadedTwice 0x{}", encode(TransactionError::AccountLoadedTwice));
    println!("TransactionError::AccountNotFound 0x{}", encode(TransactionError::AccountNotFound));
    println!("TransactionError::ProgramAccountNotFound 0x{}", encode(TransactionError::ProgramAccountNotFound));
    println!("TransactionError::InsufficientFundsForFee 0x{}", encode(TransactionError::InsufficientFundsForFee));
    println!("TransactionError::InvalidAccountForFee 0x{}", encode(TransactionError::InvalidAccountForFee));
    println!("TransactionError::AlreadyProcessed 0x{}", encode(TransactionError::AlreadyProcessed));
    println!("TransactionError::BlockhashNotFound 0x{}", encode(TransactionError::BlockhashNotFound));
    for i in 0..2 {
        println!("TransactionError::InstructionError({i}, InstructionError::GenericError) 0x{}", encode(TransactionError::InstructionError(i, InstructionError::GenericError)));
        println!("TransactionError::InstructionError({i}, InstructionError::InvalidArgument) 0x{}", encode(TransactionError::InstructionError(i, InstructionError::InvalidArgument)));
        println!("TransactionError::InstructionError({i}, InstructionError::InvalidInstructionData) 0x{}", encode(TransactionError::InstructionError(i, InstructionError::InvalidInstructionData)));
        println!("TransactionError::InstructionError({i}, InstructionError::InvalidAccountData) 0x{}", encode(TransactionError::InstructionError(i, InstructionError::InvalidAccountData)));
        println!("TransactionError::InstructionError({i}, InstructionError::AccountDataTooSmall) 0x{}", encode(TransactionError::InstructionError(i, InstructionError::AccountDataTooSmall)));
        println!("TransactionError::InstructionError({i}, InstructionError::InsufficientFunds) 0x{}", encode(TransactionError::InstructionError(i, InstructionError::InsufficientFunds)));
        println!("TransactionError::InstructionError({i}, InstructionError::IncorrectProgramId) 0x{}", encode(TransactionError::InstructionError(i, InstructionError::IncorrectProgramId)));
        println!("TransactionError::InstructionError({i}, InstructionError::MissingRequiredSignature) 0x{}", encode(TransactionError::InstructionError(i, InstructionError::MissingRequiredSignature)));
        println!("TransactionError::InstructionError({i}, InstructionError::AccountAlreadyInitialized) 0x{}", encode(TransactionError::InstructionError(i, InstructionError::AccountAlreadyInitialized)));
        println!("TransactionError::InstructionError({i}, InstructionError::UninitializedAccount) 0x{}", encode(TransactionError::InstructionError(i, InstructionError::UninitializedAccount)));
        println!("TransactionError::InstructionError({i}, InstructionError::UnbalancedInstruction) 0x{}", encode(TransactionError::InstructionError(i, InstructionError::UnbalancedInstruction)));
        println!("TransactionError::InstructionError({i}, InstructionError::ModifiedProgramId) 0x{}", encode(TransactionError::InstructionError(i, InstructionError::ModifiedProgramId)));
        println!("TransactionError::InstructionError({i}, InstructionError::ExternalAccountLamportSpend) 0x{}", encode(TransactionError::InstructionError(i, InstructionError::ExternalAccountLamportSpend)));
        println!("TransactionError::InstructionError({i}, InstructionError::ExternalAccountDataModified) 0x{}", encode(TransactionError::InstructionError(i, InstructionError::ExternalAccountDataModified)));
        println!("TransactionError::InstructionError({i}, InstructionError::ReadonlyLamportChange) 0x{}", encode(TransactionError::InstructionError(i, InstructionError::ReadonlyLamportChange)));
        println!("TransactionError::InstructionError({i}, InstructionError::ReadonlyDataModified) 0x{}", encode(TransactionError::InstructionError(i, InstructionError::ReadonlyDataModified)));
        println!("TransactionError::InstructionError({i}, InstructionError::DuplicateAccountIndex) 0x{}", encode(TransactionError::InstructionError(i, InstructionError::DuplicateAccountIndex)));
        println!("TransactionError::InstructionError({i}, InstructionError::ExecutableModified) 0x{}", encode(TransactionError::InstructionError(i, InstructionError::ExecutableModified)));
        println!("TransactionError::InstructionError({i}, InstructionError::RentEpochModified) 0x{}", encode(TransactionError::InstructionError(i, InstructionError::RentEpochModified)));
        println!("TransactionError::InstructionError({i}, InstructionError::NotEnoughAccountKeys) 0x{}", encode(TransactionError::InstructionError(i, InstructionError::NotEnoughAccountKeys)));
        println!("TransactionError::InstructionError({i}, InstructionError::AccountDataSizeChanged) 0x{}", encode(TransactionError::InstructionError(i, InstructionError::AccountDataSizeChanged)));
        println!("TransactionError::InstructionError({i}, InstructionError::AccountNotExecutable) 0x{}", encode(TransactionError::InstructionError(i, InstructionError::AccountNotExecutable)));
        println!("TransactionError::InstructionError({i}, InstructionError::AccountBorrowFailed) 0x{}", encode(TransactionError::InstructionError(i, InstructionError::AccountBorrowFailed)));
        println!("TransactionError::InstructionError({i}, InstructionError::AccountBorrowOutstanding) 0x{}", encode(TransactionError::InstructionError(i, InstructionError::AccountBorrowOutstanding)));
        println!("TransactionError::InstructionError({i}, InstructionError::DuplicateAccountOutOfSync) 0x{}", encode(TransactionError::InstructionError(i, InstructionError::DuplicateAccountOutOfSync)));
        println!("TransactionError::InstructionError({i}, InstructionError::Custom(0)) 0x{}", encode(TransactionError::InstructionError(i, InstructionError::Custom(0))));
        println!("TransactionError::InstructionError({i}, InstructionError::InvalidError) 0x{}", encode(TransactionError::InstructionError(i, InstructionError::InvalidError)));
        println!("TransactionError::InstructionError({i}, InstructionError::ExecutableDataModified) 0x{}", encode(TransactionError::InstructionError(i, InstructionError::ExecutableDataModified)));
        println!("TransactionError::InstructionError({i}, InstructionError::ExecutableLamportChange) 0x{}", encode(TransactionError::InstructionError(i, InstructionError::ExecutableLamportChange)));
        println!("TransactionError::InstructionError({i}, InstructionError::ExecutableAccountNotRentExempt) 0x{}", encode(TransactionError::InstructionError(i, InstructionError::ExecutableAccountNotRentExempt)));
        println!("TransactionError::InstructionError({i}, InstructionError::UnsupportedProgramId) 0x{}", encode(TransactionError::InstructionError(i, InstructionError::UnsupportedProgramId)));
        println!("TransactionError::InstructionError({i}, InstructionError::CallDepth) 0x{}", encode(TransactionError::InstructionError(i, InstructionError::CallDepth)));
        println!("TransactionError::InstructionError({i}, InstructionError::MissingAccount) 0x{}", encode(TransactionError::InstructionError(i, InstructionError::MissingAccount)));
        println!("TransactionError::InstructionError({i}, InstructionError::ReentrancyNotAllowed) 0x{}", encode(TransactionError::InstructionError(i, InstructionError::ReentrancyNotAllowed)));
        println!("TransactionError::InstructionError({i}, InstructionError::MaxSeedLengthExceeded) 0x{}", encode(TransactionError::InstructionError(i, InstructionError::MaxSeedLengthExceeded)));
        println!("TransactionError::InstructionError({i}, InstructionError::InvalidSeeds) 0x{}", encode(TransactionError::InstructionError(i, InstructionError::InvalidSeeds)));
        println!("TransactionError::InstructionError({i}, InstructionError::InvalidRealloc) 0x{}", encode(TransactionError::InstructionError(i, InstructionError::InvalidRealloc)));
        println!("TransactionError::InstructionError({i}, InstructionError::ComputationalBudgetExceeded) 0x{}", encode(TransactionError::InstructionError(i, InstructionError::ComputationalBudgetExceeded)));
        println!("TransactionError::InstructionError({i}, InstructionError::PrivilegeEscalation) 0x{}", encode(TransactionError::InstructionError(i, InstructionError::PrivilegeEscalation)));
        println!("TransactionError::InstructionError({i}, InstructionError::ProgramEnvironmentSetupFailure) 0x{}", encode(TransactionError::InstructionError(i, InstructionError::ProgramEnvironmentSetupFailure)));
        println!("TransactionError::InstructionError({i}, InstructionError::ProgramFailedToComplete) 0x{}", encode(TransactionError::InstructionError(i, InstructionError::ProgramFailedToComplete)));
        println!("TransactionError::InstructionError({i}, InstructionError::ProgramFailedToCompile) 0x{}", encode(TransactionError::InstructionError(i, InstructionError::ProgramFailedToCompile)));
        println!("TransactionError::InstructionError({i}, InstructionError::Immutable) 0x{}", encode(TransactionError::InstructionError(i, InstructionError::Immutable)));
        println!("TransactionError::InstructionError({i}, InstructionError::IncorrectAuthority) 0x{}", encode(TransactionError::InstructionError(i, InstructionError::IncorrectAuthority)));
        println!("TransactionError::InstructionError({i}, InstructionError::BorshIoError(\"hi\")) 0x{}", encode(TransactionError::InstructionError(i, InstructionError::BorshIoError("hi".to_owned()))));
        println!("TransactionError::InstructionError({i}, InstructionError::AccountNotRentExempt) 0x{}", encode(TransactionError::InstructionError(i, InstructionError::AccountNotRentExempt)));
        println!("TransactionError::InstructionError({i}, InstructionError::InvalidAccountOwner) 0x{}", encode(TransactionError::InstructionError(i, InstructionError::InvalidAccountOwner)));
        println!("TransactionError::InstructionError({i}, InstructionError::ArithmeticOverflow) 0x{}", encode(TransactionError::InstructionError(i, InstructionError::ArithmeticOverflow)));
        println!("TransactionError::InstructionError({i}, InstructionError::UnsupportedSysvar) 0x{}", encode(TransactionError::InstructionError(i, InstructionError::UnsupportedSysvar)));
        println!("TransactionError::InstructionError({i}, InstructionError::IllegalOwner) 0x{}", encode(TransactionError::InstructionError(i, InstructionError::IllegalOwner)));
        println!("TransactionError::InstructionError({i}, InstructionError::MaxAccountsDataAllocationsExceeded) 0x{}", encode(TransactionError::InstructionError(i, InstructionError::MaxAccountsDataAllocationsExceeded)));
        println!("TransactionError::InstructionError({i}, InstructionError::MaxAccountsExceeded) 0x{}", encode(TransactionError::InstructionError(i, InstructionError::MaxAccountsExceeded)));
        println!("TransactionError::InstructionError({i}, InstructionError::MaxInstructionTraceLengthExceeded) 0x{}", encode(TransactionError::InstructionError(i, InstructionError::MaxInstructionTraceLengthExceeded)));
        println!("TransactionError::InstructionError({i}, InstructionError::BuiltinProgramsMustConsumeComputeUnits) 0x{}", encode(TransactionError::InstructionError(i, InstructionError::BuiltinProgramsMustConsumeComputeUnits)));
    }
    println!("TransactionError::CallChainTooDeep 0x{}", encode(TransactionError::CallChainTooDeep));
    println!("TransactionError::MissingSignatureForFee 0x{}", encode(TransactionError::MissingSignatureForFee));
    println!("TransactionError::InvalidAccountIndex 0x{}", encode(TransactionError::InvalidAccountIndex));
    println!("TransactionError::SignatureFailure 0x{}", encode(TransactionError::SignatureFailure));
    println!("TransactionError::InvalidProgramForExecution 0x{}", encode(TransactionError::InvalidProgramForExecution));
    println!("TransactionError::SanitizeFailure 0x{}", encode(TransactionError::SanitizeFailure));
    println!("TransactionError::ClusterMaintenance 0x{}", encode(TransactionError::ClusterMaintenance));
    println!("TransactionError::AccountBorrowOutstanding 0x{}", encode(TransactionError::AccountBorrowOutstanding));
    println!("TransactionError::WouldExceedMaxBlockCostLimit 0x{}", encode(TransactionError::WouldExceedMaxBlockCostLimit));
    println!("TransactionError::UnsupportedVersion 0x{}", encode(TransactionError::UnsupportedVersion));
    println!("TransactionError::InvalidWritableAccount 0x{}", encode(TransactionError::InvalidWritableAccount));
    println!("TransactionError::WouldExceedMaxAccountCostLimit 0x{}", encode(TransactionError::WouldExceedMaxAccountCostLimit));
    println!("TransactionError::WouldExceedAccountDataBlockLimit 0x{}", encode(TransactionError::WouldExceedAccountDataBlockLimit));
    println!("TransactionError::TooManyAccountLocks 0x{}", encode(TransactionError::TooManyAccountLocks));
    println!("TransactionError::AddressLookupTableNotFound 0x{}", encode(TransactionError::AddressLookupTableNotFound));
    println!("TransactionError::InvalidAddressLookupTableOwner 0x{}", encode(TransactionError::InvalidAddressLookupTableOwner));
    println!("TransactionError::InvalidAddressLookupTableData 0x{}", encode(TransactionError::InvalidAddressLookupTableData));
    println!("TransactionError::InvalidAddressLookupTableIndex 0x{}", encode(TransactionError::InvalidAddressLookupTableIndex));
    println!("TransactionError::InvalidRentPayingAccount 0x{}", encode(TransactionError::InvalidRentPayingAccount));
    println!("TransactionError::WouldExceedMaxVoteCostLimit 0x{}", encode(TransactionError::WouldExceedMaxVoteCostLimit));
    println!("TransactionError::WouldExceedAccountDataTotalLimit 0x{}", encode(TransactionError::WouldExceedAccountDataTotalLimit));
    for i in 0..2 {
        println!("TransactionError::DuplicateInstruction({i}) 0x{}", encode(TransactionError::DuplicateInstruction(i)));        
    }
    for i in 0..2 {
        println!("TransactionError::InsufficientFundsForRent {{ account_index: {i} }} 0x{}", encode(TransactionError::InsufficientFundsForRent { account_index: i }));
    }
    println!("TransactionError::MaxLoadedAccountsDataSizeExceeded 0x{}", encode(TransactionError::MaxLoadedAccountsDataSizeExceeded));
    println!("TransactionError::InvalidLoadedAccountsDataSizeLimit 0x{}", encode(TransactionError::InvalidLoadedAccountsDataSizeLimit));
    println!("TransactionError::ResanitizationNeeded 0x{}", encode(TransactionError::ResanitizationNeeded));
    for i in 0..2 {
        println!("TransactionError::ProgramExecutionTemporarilyRestricted {{ account_index: {i} }} 0x{}", encode(TransactionError::ProgramExecutionTemporarilyRestricted { account_index: i }));
    }
    println!("TransactionError::UnbalancedTransaction 0x{}", encode(TransactionError::UnbalancedTransaction));
}
