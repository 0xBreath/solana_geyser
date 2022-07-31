#![allow(clippy::integer_arithmetic)]

use serde_json::json;

/// Integration testing for the PostgreSQL plugin
/// This requires a PostgreSQL database named 'solana' be setup at localhost at port 5432
/// This is automatically setup in the CI environment.
/// To setup manually on Ubuntu Linux, do the following,
/// sh -c 'echo "deb http://apt.postgresql.org/pub/repos/apt $(lsb_release -cs)-pgdg main" > /etc/apt/sources.list.d/pgdg.list'
/// wget --quiet -O - https://www.postgresql.org/media/keys/ACCC4CF8.asc | apt-key add -
/// apt install -y postgresql-14
/// sudo /etc/init.d/postgresql start
///
/// sudo -u postgres psql --command "CREATE USER solana WITH SUPERUSER PASSWORD 'solana';"
/// sudo -u postgres createdb -O solana solana
/// PGPASSWORD=solana psql -U solana -p 5432 -h localhost -w -d solana -f scripts/create_schema.sql
///
/// The test will cover transmitting accounts, transaction and slot,
/// block metadata.
///
/// To clean up the database: run the following, otherwise you may run into duplicate key violations:
/// PGPASSWORD=solana psql -U solana -p 5432 -h localhost -w -d solana -f scripts/drop_schema.sql
///
/// Before running 'cargo test', please run 'cargo build'
use {
    crate::lib::{
        geyser_plugin_postgres::GeyserPluginPostgresConfig, postgres_client::SimplePostgresClient,
    },
    libloading::Library,
    log::*,
    serial_test::serial,
    solana_core::validator::ValidatorConfig,
    solana_local_cluster::{
        cluster::Cluster,
        local_cluster::{ClusterConfig, LocalCluster},
        validator_configs::*,
    },
    solana_runtime::{
        snapshot_archive_info::SnapshotArchiveInfoGetter, snapshot_config::SnapshotConfig,
        snapshot_utils,
    },
    solana_sdk::{
        client::SyncClient, clock::Slot, commitment_config::CommitmentConfig,
        epoch_schedule::MINIMUM_SLOTS_PER_EPOCH, hash::Hash,
    },
    solana_streamer::socket::SocketAddrSpace,
    std::{
        fs::{self, File},
        io::Read,
        io::Write,
        path::{Path, PathBuf},
        thread::sleep,
        time::Duration,
    },
    tempfile::TempDir,
};

mod lib;
mod postgres_client;

fn main() {
    println!("Hello world!");
}
