### `cargo run`

```
TransactionError::AccountInUse 0x00000000
TransactionError::AccountLoadedTwice 0x01000000
TransactionError::AccountNotFound 0x02000000
TransactionError::ProgramAccountNotFound 0x03000000
TransactionError::InsufficientFundsForFee 0x04000000
TransactionError::InvalidAccountForFee 0x05000000
TransactionError::AlreadyProcessed 0x06000000
TransactionError::BlockhashNotFound 0x07000000
TransactionError::InstructionError(0, InstructionError::GenericError) 0x080000000000000000
TransactionError::InstructionError(0, InstructionError::InvalidArgument) 0x080000000001000000
TransactionError::InstructionError(0, InstructionError::InvalidInstructionData) 0x080000000002000000
TransactionError::InstructionError(0, InstructionError::InvalidAccountData) 0x080000000003000000
TransactionError::InstructionError(0, InstructionError::AccountDataTooSmall) 0x080000000004000000
TransactionError::InstructionError(0, InstructionError::InsufficientFunds) 0x080000000005000000
TransactionError::InstructionError(0, InstructionError::IncorrectProgramId) 0x080000000006000000
TransactionError::InstructionError(0, InstructionError::MissingRequiredSignature) 0x080000000007000000
TransactionError::InstructionError(0, InstructionError::AccountAlreadyInitialized) 0x080000000008000000
TransactionError::InstructionError(0, InstructionError::UninitializedAccount) 0x080000000009000000
TransactionError::InstructionError(0, InstructionError::UnbalancedInstruction) 0x08000000000a000000
TransactionError::InstructionError(0, InstructionError::ModifiedProgramId) 0x08000000000b000000
TransactionError::InstructionError(0, InstructionError::ExternalAccountLamportSpend) 0x08000000000c000000
TransactionError::InstructionError(0, InstructionError::ExternalAccountDataModified) 0x08000000000d000000
TransactionError::InstructionError(0, InstructionError::ReadonlyLamportChange) 0x08000000000e000000
TransactionError::InstructionError(0, InstructionError::ReadonlyDataModified) 0x08000000000f000000
TransactionError::InstructionError(0, InstructionError::DuplicateAccountIndex) 0x080000000010000000
TransactionError::InstructionError(0, InstructionError::ExecutableModified) 0x080000000011000000
TransactionError::InstructionError(0, InstructionError::RentEpochModified) 0x080000000012000000
TransactionError::InstructionError(0, InstructionError::NotEnoughAccountKeys) 0x080000000013000000
TransactionError::InstructionError(0, InstructionError::AccountDataSizeChanged) 0x080000000014000000
TransactionError::InstructionError(0, InstructionError::AccountNotExecutable) 0x080000000015000000
TransactionError::InstructionError(0, InstructionError::AccountBorrowFailed) 0x080000000016000000
TransactionError::InstructionError(0, InstructionError::AccountBorrowOutstanding) 0x080000000017000000
TransactionError::InstructionError(0, InstructionError::DuplicateAccountOutOfSync) 0x080000000018000000
TransactionError::InstructionError(0, InstructionError::Custom(0)) 0x08000000001900000000000000
TransactionError::InstructionError(0, InstructionError::InvalidError) 0x08000000001a000000
TransactionError::InstructionError(0, InstructionError::ExecutableDataModified) 0x08000000001b000000
TransactionError::InstructionError(0, InstructionError::ExecutableLamportChange) 0x08000000001c000000
TransactionError::InstructionError(0, InstructionError::ExecutableAccountNotRentExempt) 0x08000000001d000000
TransactionError::InstructionError(0, InstructionError::UnsupportedProgramId) 0x08000000001e000000
TransactionError::InstructionError(0, InstructionError::CallDepth) 0x08000000001f000000
TransactionError::InstructionError(0, InstructionError::MissingAccount) 0x080000000020000000
TransactionError::InstructionError(0, InstructionError::ReentrancyNotAllowed) 0x080000000021000000
TransactionError::InstructionError(0, InstructionError::MaxSeedLengthExceeded) 0x080000000022000000
TransactionError::InstructionError(0, InstructionError::InvalidSeeds) 0x080000000023000000
TransactionError::InstructionError(0, InstructionError::InvalidRealloc) 0x080000000024000000
TransactionError::InstructionError(0, InstructionError::ComputationalBudgetExceeded) 0x080000000025000000
TransactionError::InstructionError(0, InstructionError::PrivilegeEscalation) 0x080000000026000000
TransactionError::InstructionError(0, InstructionError::ProgramEnvironmentSetupFailure) 0x080000000027000000
TransactionError::InstructionError(0, InstructionError::ProgramFailedToComplete) 0x080000000028000000
TransactionError::InstructionError(0, InstructionError::ProgramFailedToCompile) 0x080000000029000000
TransactionError::InstructionError(0, InstructionError::Immutable) 0x08000000002a000000
TransactionError::InstructionError(0, InstructionError::IncorrectAuthority) 0x08000000002b000000
TransactionError::InstructionError(0, InstructionError::BorshIoError("hi")) 0x08000000002c00000002000000000000006869
TransactionError::InstructionError(0, InstructionError::AccountNotRentExempt) 0x08000000002d000000
TransactionError::InstructionError(0, InstructionError::InvalidAccountOwner) 0x08000000002e000000
TransactionError::InstructionError(0, InstructionError::ArithmeticOverflow) 0x08000000002f000000
TransactionError::InstructionError(0, InstructionError::UnsupportedSysvar) 0x080000000030000000
TransactionError::InstructionError(0, InstructionError::IllegalOwner) 0x080000000031000000
TransactionError::InstructionError(0, InstructionError::MaxAccountsDataAllocationsExceeded) 0x080000000032000000
TransactionError::InstructionError(0, InstructionError::MaxAccountsExceeded) 0x080000000033000000
TransactionError::InstructionError(0, InstructionError::MaxInstructionTraceLengthExceeded) 0x080000000034000000
TransactionError::InstructionError(0, InstructionError::BuiltinProgramsMustConsumeComputeUnits) 0x080000000035000000
TransactionError::InstructionError(1, InstructionError::GenericError) 0x080000000100000000
TransactionError::InstructionError(1, InstructionError::InvalidArgument) 0x080000000101000000
TransactionError::InstructionError(1, InstructionError::InvalidInstructionData) 0x080000000102000000
TransactionError::InstructionError(1, InstructionError::InvalidAccountData) 0x080000000103000000
TransactionError::InstructionError(1, InstructionError::AccountDataTooSmall) 0x080000000104000000
TransactionError::InstructionError(1, InstructionError::InsufficientFunds) 0x080000000105000000
TransactionError::InstructionError(1, InstructionError::IncorrectProgramId) 0x080000000106000000
TransactionError::InstructionError(1, InstructionError::MissingRequiredSignature) 0x080000000107000000
TransactionError::InstructionError(1, InstructionError::AccountAlreadyInitialized) 0x080000000108000000
TransactionError::InstructionError(1, InstructionError::UninitializedAccount) 0x080000000109000000
TransactionError::InstructionError(1, InstructionError::UnbalancedInstruction) 0x08000000010a000000
TransactionError::InstructionError(1, InstructionError::ModifiedProgramId) 0x08000000010b000000
TransactionError::InstructionError(1, InstructionError::ExternalAccountLamportSpend) 0x08000000010c000000
TransactionError::InstructionError(1, InstructionError::ExternalAccountDataModified) 0x08000000010d000000
TransactionError::InstructionError(1, InstructionError::ReadonlyLamportChange) 0x08000000010e000000
TransactionError::InstructionError(1, InstructionError::ReadonlyDataModified) 0x08000000010f000000
TransactionError::InstructionError(1, InstructionError::DuplicateAccountIndex) 0x080000000110000000
TransactionError::InstructionError(1, InstructionError::ExecutableModified) 0x080000000111000000
TransactionError::InstructionError(1, InstructionError::RentEpochModified) 0x080000000112000000
TransactionError::InstructionError(1, InstructionError::NotEnoughAccountKeys) 0x080000000113000000
TransactionError::InstructionError(1, InstructionError::AccountDataSizeChanged) 0x080000000114000000
TransactionError::InstructionError(1, InstructionError::AccountNotExecutable) 0x080000000115000000
TransactionError::InstructionError(1, InstructionError::AccountBorrowFailed) 0x080000000116000000
TransactionError::InstructionError(1, InstructionError::AccountBorrowOutstanding) 0x080000000117000000
TransactionError::InstructionError(1, InstructionError::DuplicateAccountOutOfSync) 0x080000000118000000
TransactionError::InstructionError(1, InstructionError::Custom(0)) 0x08000000011900000000000000
TransactionError::InstructionError(1, InstructionError::InvalidError) 0x08000000011a000000
TransactionError::InstructionError(1, InstructionError::ExecutableDataModified) 0x08000000011b000000
TransactionError::InstructionError(1, InstructionError::ExecutableLamportChange) 0x08000000011c000000
TransactionError::InstructionError(1, InstructionError::ExecutableAccountNotRentExempt) 0x08000000011d000000
TransactionError::InstructionError(1, InstructionError::UnsupportedProgramId) 0x08000000011e000000
TransactionError::InstructionError(1, InstructionError::CallDepth) 0x08000000011f000000
TransactionError::InstructionError(1, InstructionError::MissingAccount) 0x080000000120000000
TransactionError::InstructionError(1, InstructionError::ReentrancyNotAllowed) 0x080000000121000000
TransactionError::InstructionError(1, InstructionError::MaxSeedLengthExceeded) 0x080000000122000000
TransactionError::InstructionError(1, InstructionError::InvalidSeeds) 0x080000000123000000
TransactionError::InstructionError(1, InstructionError::InvalidRealloc) 0x080000000124000000
TransactionError::InstructionError(1, InstructionError::ComputationalBudgetExceeded) 0x080000000125000000
TransactionError::InstructionError(1, InstructionError::PrivilegeEscalation) 0x080000000126000000
TransactionError::InstructionError(1, InstructionError::ProgramEnvironmentSetupFailure) 0x080000000127000000
TransactionError::InstructionError(1, InstructionError::ProgramFailedToComplete) 0x080000000128000000
TransactionError::InstructionError(1, InstructionError::ProgramFailedToCompile) 0x080000000129000000
TransactionError::InstructionError(1, InstructionError::Immutable) 0x08000000012a000000
TransactionError::InstructionError(1, InstructionError::IncorrectAuthority) 0x08000000012b000000
TransactionError::InstructionError(1, InstructionError::BorshIoError("hi")) 0x08000000012c00000002000000000000006869
TransactionError::InstructionError(1, InstructionError::AccountNotRentExempt) 0x08000000012d000000
TransactionError::InstructionError(1, InstructionError::InvalidAccountOwner) 0x08000000012e000000
TransactionError::InstructionError(1, InstructionError::ArithmeticOverflow) 0x08000000012f000000
TransactionError::InstructionError(1, InstructionError::UnsupportedSysvar) 0x080000000130000000
TransactionError::InstructionError(1, InstructionError::IllegalOwner) 0x080000000131000000
TransactionError::InstructionError(1, InstructionError::MaxAccountsDataAllocationsExceeded) 0x080000000132000000
TransactionError::InstructionError(1, InstructionError::MaxAccountsExceeded) 0x080000000133000000
TransactionError::InstructionError(1, InstructionError::MaxInstructionTraceLengthExceeded) 0x080000000134000000
TransactionError::InstructionError(1, InstructionError::BuiltinProgramsMustConsumeComputeUnits) 0x080000000135000000
TransactionError::CallChainTooDeep 0x09000000
TransactionError::MissingSignatureForFee 0x0a000000
TransactionError::InvalidAccountIndex 0x0b000000
TransactionError::SignatureFailure 0x0c000000
TransactionError::InvalidProgramForExecution 0x0d000000
TransactionError::SanitizeFailure 0x0e000000
TransactionError::ClusterMaintenance 0x0f000000
TransactionError::AccountBorrowOutstanding 0x10000000
TransactionError::WouldExceedMaxBlockCostLimit 0x11000000
TransactionError::UnsupportedVersion 0x12000000
TransactionError::InvalidWritableAccount 0x13000000
TransactionError::WouldExceedMaxAccountCostLimit 0x14000000
TransactionError::WouldExceedAccountDataBlockLimit 0x15000000
TransactionError::TooManyAccountLocks 0x16000000
TransactionError::AddressLookupTableNotFound 0x17000000
TransactionError::InvalidAddressLookupTableOwner 0x18000000
TransactionError::InvalidAddressLookupTableData 0x19000000
TransactionError::InvalidAddressLookupTableIndex 0x1a000000
TransactionError::InvalidRentPayingAccount 0x1b000000
TransactionError::WouldExceedMaxVoteCostLimit 0x1c000000
TransactionError::WouldExceedAccountDataTotalLimit 0x1d000000
TransactionError::DuplicateInstruction(0) 0x1e00000000
TransactionError::DuplicateInstruction(1) 0x1e00000001
TransactionError::InsufficientFundsForRent { account_index: 0 } 0x1f00000000
TransactionError::InsufficientFundsForRent { account_index: 1 } 0x1f00000001
TransactionError::MaxLoadedAccountsDataSizeExceeded 0x20000000
TransactionError::InvalidLoadedAccountsDataSizeLimit 0x21000000
TransactionError::ResanitizationNeeded 0x22000000
TransactionError::ProgramExecutionTemporarilyRestricted { account_index: 0 } 0x2300000000
TransactionError::ProgramExecutionTemporarilyRestricted { account_index: 1 } 0x2300000001
TransactionError::UnbalancedTransaction 0x24000000
```
