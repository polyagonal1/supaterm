use std::io;

pub trait Command {
    fn size_hint(&self) -> Option<usize>;

    fn write_to(&self, database: &terminfo::Database, target: &dyn io::Write) -> io::Result<()>;
}