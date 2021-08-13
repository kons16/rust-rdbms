use anyhow::Result;

use rust_rdbms::buffer::{BufferPool, BufferPoolManager, Page};
use rust_rdbms::disk::{DiskManager, PageId};
use rust_rdbms::table::SimpleTable;

fn main() -> Result<()> {
    // ヒープファイルを開き、バッファプールとバッファマネージャーを初期化
    let disk = DiskManager::open("simple.rly")?;
    let pool = BufferPool::new(10);
    let mut bufmgr = BufferPoolManager::new(disk, pool);

    // スキーマ定義。プライマリキーは左端1つだけ
    let mut table = SimpleTable {
        meta_page_id: PageId::INVALID_PAGE_ID,
        num_key_elems: 1,
    };

    // テーブル作成
    table.create(&mut bufmgr)?;

    table.insert(&mut bufmgr, &[b"z", b"Alice", b"Smith"])?;
    table.insert(&mut bufmgr, &[b"x", b"Bob", b"Johnson"])?;
    table.insert(&mut bufmgr, &[b"y", b"Charlie", b"Williams"])?;
    table.insert(&mut bufmgr, &[b"w", b"Dave", b"Miller"])?;
    table.insert(&mut bufmgr, &[b"v", b"Eve", b"Brown"])?;

    // バッファプールの中身を全てファイルに書き出し。
    bufmgr.flush()?;
    Ok(())
}
