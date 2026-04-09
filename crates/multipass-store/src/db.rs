use std::path::Path;

use anyhow::Context;
use camino::{Utf8Path, Utf8PathBuf};
use chrono::Utc;
use rusqlite::{Connection, OptionalExtension, Transaction, params};

use crate::search::{RecentRecord, SearchHit};
use multipass_core::{Record, SearchQuery};

pub struct ShipDb {
    pub ship_root: Utf8PathBuf,
    conn: Connection,
}

pub struct ShipStats {
    pub total_records: usize,
    pub wings: Vec<(String, usize)>,
    pub rooms: Vec<(String, usize)>,
}

impl ShipDb {
    pub fn open(ship_root: &Utf8Path) -> anyhow::Result<Self> {
        std::fs::create_dir_all(ship_root)?;
        let db_path = ship_root.join("ship.sqlite3");
        let conn = Connection::open(Path::new(db_path.as_str()))
            .with_context(|| format!("opening {}", db_path))?;
        let mut db = Self {
            ship_root: ship_root.to_owned(),
            conn,
        };
        db.migrate()?;
        db.ensure_ship_row()?;
        Ok(db)
    }

    fn migrate(&mut self) -> anyhow::Result<()> {
        self.conn.execute_batch(
            "
            create table if not exists ships (
              id text primary key,
              root_path text not null unique,
              created_at text not null
            );

            create table if not exists records (
              id text primary key,
              ship_id text not null,
              wing text not null,
              room text not null,
              corridor text,
              source_path text,
              source_mtime text,
              added_by text not null,
              created_at text not null,
              chunk_index integer not null,
              content text not null,
              foreign key (ship_id) references ships(id)
            );

            create index if not exists idx_records_ship on records(ship_id);
            create index if not exists idx_records_wing on records(wing);
            create index if not exists idx_records_room on records(room);
            create index if not exists idx_records_source on records(source_path);

            create virtual table if not exists records_fts using fts5(
              record_id unindexed,
              content,
              wing,
              room,
              corridor
            );
            ",
        )?;
        Ok(())
    }

    fn ensure_ship_row(&self) -> anyhow::Result<()> {
        self.conn.execute(
            "insert or ignore into ships (id, root_path, created_at) values (?1, ?2, ?3)",
            params!["default", self.ship_root.as_str(), Utc::now().to_rfc3339()],
        )?;
        Ok(())
    }

    pub fn replace_wing_records(&mut self, wing: &str, records: &[Record]) -> anyhow::Result<()> {
        let tx = self.conn.transaction()?;
        delete_fts_for_wing(&tx, wing)?;
        tx.execute(
            "delete from records where ship_id = 'default' and wing = ?1",
            [wing],
        )?;
        for record in records {
            insert_record_tx(&tx, record)?;
        }
        tx.commit()?;
        Ok(())
    }

    pub fn insert_record(&self, record: &Record) -> anyhow::Result<()> {
        let exists: Option<String> = self
            .conn
            .query_row(
                "select id from records where id = ?1",
                [&record.id],
                |row| row.get(0),
            )
            .optional()?;
        if exists.is_some() {
            self.conn
                .execute("delete from records_fts where record_id = ?1", [&record.id])?;
        }
        self.conn.execute(
            "insert or replace into records
             (id, ship_id, wing, room, corridor, source_path, source_mtime, added_by, created_at, chunk_index, content)
             values (?1, 'default', ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                record.id,
                record.wing,
                record.room,
                record.corridor,
                record.source_path.as_ref().map(|p| p.as_str()),
                record.source_mtime.map(|m| m.to_rfc3339()),
                record.added_by,
                record.created_at.to_rfc3339(),
                record.metadata.chunk_index,
                record.content,
            ],
        )?;
        self.conn.execute(
            "insert into records_fts (record_id, content, wing, room, corridor)
             values (?1, ?2, ?3, ?4, ?5)",
            params![
                record.id,
                record.content,
                record.wing,
                record.room,
                record.corridor
            ],
        )?;
        Ok(())
    }

    pub fn delete_record(&self, record_id: &str) -> anyhow::Result<bool> {
        self.conn
            .execute("delete from records_fts where record_id = ?1", [record_id])?;
        let deleted = self
            .conn
            .execute("delete from records where id = ?1", [record_id])?;
        Ok(deleted > 0)
    }

    pub fn recent_records(&self, limit: usize) -> anyhow::Result<Vec<RecentRecord>> {
        let limit = limit.max(1) as i64;
        let mut stmt = self.conn.prepare(
            "select id, wing, room, corridor, source_path, created_at, content
             from records
             where ship_id = 'default'
             order by datetime(created_at) desc, rowid desc
             limit ?1",
        )?;
        let rows = stmt.query_map([limit], |row| {
            let content: String = row.get(6)?;
            Ok(RecentRecord {
                id: row.get(0)?,
                wing: row.get(1)?,
                room: row.get(2)?,
                corridor: row.get(3)?,
                source_path: row.get(4)?,
                created_at: row.get(5)?,
                preview: content
                    .lines()
                    .find(|line| !line.trim().is_empty())
                    .unwrap_or_default()
                    .chars()
                    .take(120)
                    .collect(),
            })
        })?;
        Ok(rows.collect::<Result<Vec<_>, _>>()?)
    }

    pub fn search(&self, query: &SearchQuery) -> anyhow::Result<Vec<SearchHit>> {
        let limit = query.limit.max(1) as i64;
        let sql = "
            select r.id, r.wing, r.room, r.source_path, r.content
            from records_fts f
            join records r on r.id = f.record_id
            where records_fts match ?1
              and (?2 is null or r.wing = ?2)
              and (?3 is null or r.room = ?3)
            order by bm25(records_fts)
            limit ?4
        ";
        let mut stmt = self.conn.prepare(sql)?;
        let rows = stmt.query_map(
            params![
                query.query,
                query.wing.as_deref(),
                query.room.as_deref(),
                limit
            ],
            |row| {
                Ok(SearchHit {
                    id: row.get(0)?,
                    wing: row.get(1)?,
                    room: row.get(2)?,
                    source_path: row.get(3)?,
                    content: row.get(4)?,
                })
            },
        )?;
        Ok(rows.collect::<Result<Vec<_>, _>>()?)
    }

    pub fn stats(&self) -> anyhow::Result<ShipStats> {
        let total_records = self.conn.query_row(
            "select count(*) from records where ship_id = 'default'",
            [],
            |row| row.get::<_, i64>(0),
        )? as usize;

        let wings = {
            let mut stmt = self.conn.prepare(
                "select wing, count(*) from records where ship_id = 'default' group by wing order by wing",
            )?;
            stmt.query_map([], |row| Ok((row.get(0)?, row.get::<_, i64>(1)? as usize)))?
                .collect::<Result<Vec<_>, _>>()?
        };

        let rooms = {
            let mut stmt = self.conn.prepare(
                "select room, count(*) from records where ship_id = 'default' group by room order by room",
            )?;
            stmt.query_map([], |row| Ok((row.get(0)?, row.get::<_, i64>(1)? as usize)))?
                .collect::<Result<Vec<_>, _>>()?
        };

        Ok(ShipStats {
            total_records,
            wings,
            rooms,
        })
    }
}

fn delete_fts_for_wing(tx: &Transaction<'_>, wing: &str) -> anyhow::Result<()> {
    tx.execute(
        "delete from records_fts
         where record_id in (
           select id from records where ship_id = 'default' and wing = ?1
         )",
        [wing],
    )?;
    Ok(())
}

fn insert_record_tx(tx: &Transaction<'_>, record: &Record) -> anyhow::Result<()> {
    tx.execute(
        "insert or replace into records
         (id, ship_id, wing, room, corridor, source_path, source_mtime, added_by, created_at, chunk_index, content)
         values (?1, 'default', ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![
            record.id,
            record.wing,
            record.room,
            record.corridor,
            record.source_path.as_ref().map(|p| p.as_str()),
            record.source_mtime.map(|m| m.to_rfc3339()),
            record.added_by,
            record.created_at.to_rfc3339(),
            record.metadata.chunk_index,
            record.content,
        ],
    )?;
    tx.execute(
        "insert into records_fts (record_id, content, wing, room, corridor)
         values (?1, ?2, ?3, ?4, ?5)",
        params![
            record.id,
            record.content,
            record.wing,
            record.room,
            record.corridor
        ],
    )?;
    Ok(())
}
