use diesel::{ Connection, RunQueryDsl, Queryable };
use diesel::query_dsl::limit_dsl::LimitDsl;
use diesel::query_dsl::offset_dsl::OffsetDsl;
use diesel::sql_types::HasSqlType;
use diesel::query_builder::{ Query, QueryFragment, QueryId };
use diesel::result::Error;

pub trait BasicMethod<M, Conn>
    where M: LimitDsl + OffsetDsl +
        Queryable<<<M as LimitDsl>::Output as Query>::SqlType,
        <Conn as Connection>::Backend> +
        Queryable<<<<M as LimitDsl>::Output as OffsetDsl>::Output as Query>::SqlType, <Conn as Connection>::Backend>,
        <M as LimitDsl>::Output: RunQueryDsl<Conn> +
        Query +
        QueryFragment<<Conn as Connection>::Backend> +
        QueryId + 
        OffsetDsl,
        <<M as LimitDsl>::Output as OffsetDsl>::Output : RunQueryDsl<Conn> +
        Query +
        QueryFragment<<Conn as Connection>::Backend> +
        QueryId,
        Conn: Connection,
        <Conn as Connection>::Backend: HasSqlType<<<M as LimitDsl>::Output as Query>::SqlType>,
        <Conn as Connection>::Backend: HasSqlType<<<<M as LimitDsl>::Output as OffsetDsl>::Output as Query>::SqlType> {

    fn list(connection: Conn, model: M, limit: i64, offset: i64) -> Result<Vec<M>, Error> {
        model
            .limit(limit)
            .offset(offset)
            .load::<M>(&connection)
    }
}
