use std::{error::Error, io, path::PathBuf, sync::Arc};
use clap::Args;
use rustyjsonserver::{
    config::{manager::ConfigManager, resolver::get_config_path_cwd}, filewatcher::watcher, http::server, rjscript::evaluator::runtime::runtime_globals::RuntimeGlobals, rjsdb::{TableDb, db::JsonTableDb}
};
use tracing::info;

/// Run the HTTP server (with optional file-watcher).
#[derive(Args, Debug)]
pub struct ServeArgs {
    /// Config file to watch and serve
    #[arg(short, long, value_name = "FILE")]
    pub config: PathBuf,

    /// Disable file-watching
    #[arg(long)]
    pub no_watch: bool,
}

pub async fn run(args: ServeArgs) -> Result<(), Box<dyn Error>> {
    let cfg = get_config_path_cwd(&args.config.to_string_lossy());
    info!(%cfg, watch_enabled = !args.no_watch, "serving configuration");

    // init persistence
    let path = std::env::var("RJS_DB_DIR").unwrap_or_else(|_| "./data".into());
    let db = JsonTableDb::open(path)?;
    let db_arc: Arc<dyn TableDb> = Arc::new(db);
    RuntimeGlobals::init_with_db(Some(db_arc));

    // Initialize manager, mapping String→io::Error
    let manager = ConfigManager::new(cfg.clone())
        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("ConfigManager::new failed: {}", e)))?;

    // Spawn file-watcher if requested
    if !args.no_watch {
        watcher::spawn_watcher(manager.clone());
    }

    let addr = format!("0.0.0.0:{}", manager.port());
    info!(%addr, "starting HTTP server");
    
    server::run(&addr, manager.routes_handle()).await?;

    Ok(())
}
