use std::io;

pub trait Command {
    fn size_hint(&self) -> Option<usize>;

    fn write_to(
        &self,
        database: &terminfo::Database,
        ctx: &mut terminfo::expand::Context,
        target: &mut dyn io::Write
    ) -> io::Result<()>;
}