mod progress;

use serde::{Deserialize, Serialize};

/// An index of the validator, which is for a single height. (Mapping from the actual public key to the index may differ for different heights.)
pub type ValidatorIndex = usize;
/// An identifier of the block, which is uniquely mapped to a block. Like `ValidatorIndex`, it is for a single height. (Mapping from the actual block to the index may differ for different heights.)
pub type BlockIdentifier = usize;
/// A UNIX timestamp measured in milliseconds.
pub type Timestamp = i64;

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ConsensusParams {
    pub timeout_ms: u64,
}

/// An event that (potentially) triggers a state transition of `StateMachine`.
///
/// Note that there is no cryptography-related info here, because it's
/// the lower layer's responsibility to verifiy and refine the raw messages (containing such cryptography-related info) into this abstracted data.
/// Also all the identifiers (for blocks and validators) become integer indices here, and
/// the lower layer will keep the mapping from the actual data to the indices.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConsensusEvent {
    /// Informs that the node has received a block proposal.
    BlockProposal {
        proposal: BlockIdentifier,
        proposer: ValidatorIndex,
        round: usize,
        time: Timestamp,
    },
    /// Informs that the node is in favor of or against a proposal.
    ProposalFavor {
        proposal: BlockIdentifier,
        /// Whether this node is in favor of the proposal.
        favor: bool,
        time: Timestamp,
    },
    /// Informs that `CreateAndBroadcastProposal` has been completed.
    BlockProposalBroadcasted {
        proposal: BlockIdentifier,
        round: usize,
        time: Timestamp,
    },
    /// Informs that the node has received a block prevote.
    Prevote {
        proposal: BlockIdentifier,
        signer: ValidatorIndex,
        round: usize,
        time: Timestamp,
    },
    /// Informs that the node has received a block precommit.
    Precommit {
        proposal: BlockIdentifier,
        signer: ValidatorIndex,
        round: usize,
        time: Timestamp,
    },
    /// Informs that the node has received a nil prevote.
    NilPrevote {
        signer: ValidatorIndex,
        round: usize,
        time: Timestamp,
    },
    /// Informs that the node has received a nil precommit.
    NilPrecommit {
        signer: ValidatorIndex,
        round: usize,
        time: Timestamp,
    },
    /// Informs that time has passed.
    Timer { time: Timestamp },
}

/// A response that the consensus might emit for a given event, which must be properly handled by the lower layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConsensusResponse {
    /// Creation of the actual proposal is not the role of the consensus; the lower layer will take care of it.
    CreateAndBroadcastProposal {
        round: usize,
    },
    BroadcastPrevote {
        proposal: BlockIdentifier,
        round: usize,
    },
    BroadcastPrecommit {
        proposal: BlockIdentifier,
        round: usize,
    },
    BroadcastNilPrevote {
        round: usize,
    },
    BroadcastNilPrecommit {
        round: usize,
    },
    FinalizeBlock {
        proposal: BlockIdentifier,
    },
    ViolationReport {
        violator: ValidatorIndex,
        description: String,
    },
}

/// An immutable set of information that is used to perform the consensus for a single height.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeightInfo {
    /// The list of voting powers sorted by the leader order.
    ///
    /// Important note: `ValidatorIndex` is used to index this list.
    pub validators: Vec<u64>,

    /// The index of this node
    pub this_node_index: ValidatorIndex,

    /// The timestamp of the beginning of the round 0.
    pub timestamp: Timestamp,

    /// The consensus parameters
    pub consensus_params: ConsensusParams,
}

/// The state of the consensus during a single height.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConsensusState {
    round: usize,
    // TODO: One typical implementation would have some kind of a verbose `enum` of the state variants.
}

impl ConsensusState {
    /// Prepares the initial state of the consensus.
    pub fn new(_height_info: HeightInfo) -> Self {
        ConsensusState { round: 0 }
    }
}

pub use progress::progress;
