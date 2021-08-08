pub const PAGE_SIZE: usize = 4096;

pub struct DiskManager {
    // ヒープファイルのファイルディスクリプタ
    heap_file: File,
    // 採番するページIDを決めるカウンタ
    next_page_id: u64,
}

pub struct PageId(pub u64);

impl DiskManager {
    // コンストラクタ
    pub fn new(heap_file: File) -> io::Result<Self> {
        let heap_file_size = heap_file.metadata()?.len();
        let next_page_id = heap_file_size / PAGE_SIZE as u64;
        Ok(Self {
            heap_file,
            next_page_id,
        })
    }

    // ファイルパスを指定して開く
    pub fn open(data_file_path: impl AsRef<Path>) -> io::Result<Self> {
        let headp_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(headp_file_path)?;
        Self::new(heap_file)
    }

    // 新しいページIDを採番する
    // 新しいページを作るメソッドだが、実際の処理はページIDを採番するだけ
    pub fn allocate_page(&mut self) -> PageId {
        let page_id = self.next_page_id;
        self.next_page_id += 1;
        PageId(page_id)
    }

    // ページのデータを読み出す
    pub fn read_page_data(&mut self, page_id: PageId, data: &mut [u8]) -> io::Result<()> {
        let offset = PAGE_SIZE as u64 * page_id.to_u64();
        self.headp_file.seek(SeekFrom::Start(offset))?;
        self.headp_file.read_exact(data)
    }

    // データをページに書き出す
    pub fn write_page_data(&mut self, page_id: PageId, data: &[u8]) -> io::Result<()> {
        let offset = PAGE_SIZE as u64 * page_id.to_u64();
        self.headp_file.seek(SeekFrom::Start(offset))?;
        self.headp_file.write_all(data)
    }
}
