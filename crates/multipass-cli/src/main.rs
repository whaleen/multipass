use anyhow::Context;
use camino::Utf8PathBuf;
use clap::{Parser, Subcommand};
use multipass_core::{MultipassConfig, SearchQuery};
use multipass_ingest::{default_project_config, ingest_project};
use multipass_store::ShipDb;

#[derive(Parser)]
#[command(name = "multipass-rs")]
struct Cli {
    #[arg(long, global = true)]
    ship: Option<Utf8PathBuf>,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Init {
        dir: Utf8PathBuf,
    },
    Mine {
        dir: Utf8PathBuf,
    },
    Search {
        query: String,
        #[arg(long)]
        wing: Option<String>,
        #[arg(long)]
        room: Option<String>,
        #[arg(long, default_value_t = 5)]
        limit: usize,
    },
    Status,
    WakeUp,
    McpServer,
}

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .without_time()
        .init();

    let cli = Cli::parse();
    match cli.command {
        Command::Init { dir } => cmd_init(dir),
        Command::Mine { dir } => cmd_mine(cli.ship, dir),
        Command::Search {
            query,
            wing,
            room,
            limit,
        } => cmd_search(cli.ship, query, wing, room, limit),
        Command::Status => cmd_status(cli.ship),
        Command::WakeUp => cmd_wakeup(cli.ship),
        Command::McpServer => cmd_mcp_server(cli.ship),
    }
}

fn ship_root(arg: Option<Utf8PathBuf>) -> Utf8PathBuf {
    arg.unwrap_or_else(|| Utf8PathBuf::from(".multipass-rs/ship"))
}

fn cmd_init(dir: Utf8PathBuf) -> anyhow::Result<()> {
    let dir = dir.canonicalize_utf8().unwrap_or(dir);
    let config = default_project_config(&dir);
    config.save(&dir)?;
    println!("initialized {}", dir);
    println!("wing: {}", config.wing);
    for room in config.rooms {
        println!("room: {}", room.name);
    }
    Ok(())
}

fn cmd_mine(ship: Option<Utf8PathBuf>, dir: Utf8PathBuf) -> anyhow::Result<()> {
    let ship_root = ship_root(ship);
    let dir = dir.canonicalize_utf8().unwrap_or(dir);
    let config = MultipassConfig::load(&dir)
        .with_context(|| format!("missing multipass.yaml in {}", dir))?;
    let records = ingest_project(&dir, &config)?;
    let mut db = ShipDb::open(&ship_root)?;
    let count = records.len();
    db.replace_wing_records(&config.wing, &records)?;
    println!("mined {count} records into {}", ship_root);
    Ok(())
}

fn cmd_search(
    ship: Option<Utf8PathBuf>,
    query: String,
    wing: Option<String>,
    room: Option<String>,
    limit: usize,
) -> anyhow::Result<()> {
    let ship_root = ship_root(ship);
    let db = ShipDb::open(&ship_root)?;
    let hits = db.search(&SearchQuery {
        query,
        wing,
        room,
        limit,
    })?;
    for hit in hits {
        println!("[{}] {} / {}", hit.id, hit.wing, hit.room);
        if let Some(source) = hit.source_path {
            println!("source: {source}");
        }
        println!("{}", hit.content.lines().next().unwrap_or_default());
        println!();
    }
    Ok(())
}

fn cmd_status(ship: Option<Utf8PathBuf>) -> anyhow::Result<()> {
    let ship_root = ship_root(ship);
    let db = ShipDb::open(&ship_root)?;
    let stats = db.stats()?;
    println!("ship: {}", ship_root);
    println!("records: {}", stats.total_records);
    println!("wings:");
    for (wing, count) in stats.wings {
        println!("  {wing}: {count}");
    }
    println!("rooms:");
    for (room, count) in stats.rooms {
        println!("  {room}: {count}");
    }
    Ok(())
}

fn cmd_wakeup(ship: Option<Utf8PathBuf>) -> anyhow::Result<()> {
    let ship_root = ship_root(ship);
    let db = ShipDb::open(&ship_root)?;
    let stats = db.stats()?;
    let recent_records = db
        .recent_records(5)?
        .into_iter()
        .map(|record| multipass_aaak::AaakRecentRecord {
            wing: record.wing,
            room: record.room,
            preview: record.preview,
        })
        .collect();
    let brief = multipass_aaak::AaakBrief {
        ship: ship_root.to_string(),
        total_records: stats.total_records,
        wings: stats.wings,
        rooms: stats.rooms,
        recent_records,
    };
    println!("{}", brief.render());
    Ok(())
}

fn cmd_mcp_server(ship: Option<Utf8PathBuf>) -> anyhow::Result<()> {
    let ship_root = ship_root(ship);
    multipass_mcp::serve_stdio(&ship_root)
}
