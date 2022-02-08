use rand::Rng;
use sc_client_db::{Database, DbHash, DatabaseSource};
use sp_database::Transaction;
use sc_service::Configuration;
use std::{
	fmt::Debug,
	sync::Arc,
	time::{Duration, Instant},
};
use frame_benchmarking::BenchmarkResult;
use log::info;
use itertools::Itertools;

use crate::bedrock::*;

const KEY_LEN_BYTES: usize = 48;
const WRITE_LEN_BYTES: usize = 64;
const COLUMN: u32 = 0;
const INIT_SIZE: usize = 1_000_000;

/// Key-Value pair.
type KV = (Vec<u8>, Vec<u8>);
type DB = Arc<dyn Database<DbHash>>;

pub fn block_import(cfg: &BedrockParams, config: &Configuration, db: DB) {
	// new_full_parts would re-open the DB. Instead set the db-source to the one
	// that we already have.
	/*config.database = DatabaseSource::Custom(db);

	let (client, backend, keystore_container, task_manager) =
		sc_service::new_full_parts::<Block, RuntimeApi, _>(
			config,
			telemetry.as_ref().map(|(_, telemetry)| telemetry.handle()),
			executor,
		)?;*/
}

pub fn db_read(cfg: &BedrockParams, db: DB) {

}

pub fn db_write(cfg: &BedrockParams, db: DB) {
	dump_info();
	info!("Populating DB with {} KV pairs...", INIT_SIZE);
	for _ in 0..100 {
		populate(INIT_SIZE, db.clone());
	}
	info!("Done populating");

	for p in 2..10 {
		let n = 1 << (p << 1);
		let r = 2 * 5 + 1; // uneven for median calculation

		let mut times = Vec::<Duration>::new();
		// Create the key and value pairs.
		let kvs = prepare_kvs(n);

		for _ in 0..r {
			let start = Instant::now();
			// Write them and measure the time.
			write_kvs(&kvs, db.clone());
			let elapsed = start.elapsed();

			// Cleanup.
			remove_kvs(&kvs, db.clone());
			times.push(elapsed);
		}
		// Calculate the median of the times.
		times.sort();
		let median = times[times.len() / 2];
		let per = (median.as_nanos() as f64 / n as f64) / 1000.0;
		// Print the results.
		info!("A commit with {} values took {} ms. {:.3} µs each", n, median.as_millis(), per);
	}
}

fn populate(num: usize, db: DB) {
	info!("Populate key create");
	let mut i = 0;
	let kvs = prepare_kvs(num);
	write_kvs(&kvs, db.clone());
}

fn write_kvs(kvs: &Vec<KV>, db: DB) {
	let mut commit = Transaction::new();
	for (k, v) in kvs {
		commit.set(COLUMN, k, v);
	}
	db.commit(commit).unwrap();
}

fn remove_kvs(kvs: &Vec<KV>, db: DB) {
	let mut commit = Transaction::new();
	for (k, _) in kvs {
		commit.remove(COLUMN, &k);
	}
	db.commit(commit).unwrap();
}

fn prepare_kvs(num: usize) -> Vec<KV> {
	let mut ret = Vec::with_capacity(num);
	// Seed for reproducibility.
	let mut rng = rand::thread_rng();

	for i in 0..num {
		let k = random_vec(&mut rng, KEY_LEN_BYTES);
		let v = random_vec(&mut rng, WRITE_LEN_BYTES);
		ret.push((k, v));
	}

	ret
}

fn dump_info() {
	info!("KEY_LEN = {}, VALUE_LEN = {}", KEY_LEN_BYTES, WRITE_LEN_BYTES);
}

// Thanks Nikolay!
fn random_vec<R: Rng>(rng: &mut R, len: usize) -> Vec<u8> {
	let mut val = vec![0u8; len];
	rng.fill_bytes(&mut val[..]);
	val
}
