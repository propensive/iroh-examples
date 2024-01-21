//! Command line arguments.
use clap::{Parser, Subcommand};
use iroh_bytes::{Hash, HashAndFormat};
use iroh_mainline_content_discovery::TrackerId;
use iroh_net::{ticket::BlobTicket, NodeId};
use std::{fmt::Display, str::FromStr};

#[derive(Parser, Debug)]
pub struct Args {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Announce(AnnounceArgs),
    Query(QueryArgs),
    QueryDht(QueryDhtArgs),
}

/// Various ways to specify content.
#[derive(Debug, Clone, derive_more::From)]
pub enum ContentArg {
    Hash(Hash),
    HashAndFormat(HashAndFormat),
    Ticket(BlobTicket),
}

impl ContentArg {
    /// Get the hash and format of the content.
    pub fn hash_and_format(&self) -> HashAndFormat {
        match self {
            ContentArg::Hash(hash) => HashAndFormat::raw(*hash),
            ContentArg::HashAndFormat(haf) => *haf,
            ContentArg::Ticket(ticket) => HashAndFormat {
                hash: ticket.hash(),
                format: ticket.format(),
            },
        }
    }

    /// Get the host of the content. Only defined for tickets.
    pub fn host(&self) -> Option<NodeId> {
        match self {
            ContentArg::Hash(_) => None,
            ContentArg::HashAndFormat(_) => None,
            ContentArg::Ticket(ticket) => Some(ticket.node_addr().node_id),
        }
    }
}

impl Display for ContentArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContentArg::Hash(hash) => Display::fmt(hash, f),
            ContentArg::HashAndFormat(haf) => Display::fmt(haf, f),
            ContentArg::Ticket(ticket) => Display::fmt(ticket, f),
        }
    }
}

impl FromStr for ContentArg {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(hash) = Hash::from_str(s) {
            Ok(hash.into())
        } else if let Ok(haf) = HashAndFormat::from_str(s) {
            Ok(haf.into())
        } else if let Ok(ticket) = BlobTicket::from_str(s) {
            Ok(ticket.into())
        } else {
            anyhow::bail!("invalid hash and format")
        }
    }
}

#[derive(Parser, Debug)]
pub struct AnnounceArgs {
    /// the tracker to announce to
    #[clap(long)]
    pub tracker: NodeId,

    /// The host to announce. Not needed if content is a ticket.
    #[clap(long)]
    pub host: Option<NodeId>,

    /// The content to announce.
    ///
    /// Content can be specified as a hash, a hash and format, or a ticket.
    /// If a hash is specified, the format is assumed to be raw.
    /// Unless a ticket is specified, the host must be specified.
    pub content: Vec<ContentArg>,

    /// Announce that the peer has only partial data.
    #[clap(long)]
    pub partial: bool,

    /// the port to use for announcing
    #[clap(long)]
    pub magic_port: Option<u16>,
}

#[derive(Parser, Debug)]
pub struct QueryArgs {
    /// the tracker to query
    #[clap(long)]
    pub tracker: TrackerId,

    /// The content to find hosts for.
    pub content: ContentArg,

    /// Ask for hosts that were announced as having just partial data
    #[clap(long)]
    pub partial: bool,

    /// Ask for hosts that were recently checked and found to have some data
    #[clap(long)]
    pub verified: bool,

    /// the port to use for querying
    #[clap(long)]
    pub magic_port: Option<u16>,
}

#[derive(Parser, Debug)]
pub struct QueryDhtArgs {
    /// The content to find hosts for.
    pub content: ContentArg,

    /// Ask for hosts that were announced as having just partial data
    #[clap(long)]
    pub partial: bool,

    /// Ask for hosts that were recently checked and found to have some data
    #[clap(long)]
    pub verified: bool,

    /// Parallelism for querying the dht
    #[clap(long)]
    pub query_parallelism: Option<usize>,

    /// the port to use for querying
    #[clap(long)]
    pub quinn_port: Option<u16>,
}