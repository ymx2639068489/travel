use diesel::mysql::Mysql;
use diesel::prelude::*;
use diesel::query_builder::*;
use diesel::query_dsl::methods::LoadQuery;
use diesel::sql_types::BigInt;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct QueryPager {
  pub page: i64,
  pub per_page: i64,
}

#[derive(QueryId)]
pub struct PaginatedQuery<T> {
  pub query: T,
  pub page: i64,
  pub per_page: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseList<T> {
  pub page: i64,
  pub per_page: i64,
  pub total: i64,
  pub last_page: i64,
  pub data: Vec<T>,
}

impl<T> ResponseList<T> {
  pub fn from<A>(source: ResponseList<A>) -> Self
  where
    T: From<A>,
    A: Clone,
  {
    ResponseList {
      page: source.page,
      per_page: source.per_page,
      total: source.total,
      last_page: source.last_page,
      data: source.data.into_iter().map(T::from).collect::<Vec<T>>(),
    }
  }
}
pub trait Paginate: Sized {
  /// Start the pagination process by setting the page number
  fn page(self, page: Option<i64>) -> PaginatedQuery<Self> {
    let mut page = page.unwrap_or(1);

    if page <= 0 {
      page = 1;
    }

    PaginatedQuery {
      query: self,
      page,
      per_page: 15,
    }
  }

  /// Start the pagination process by setting the amount of items per page
  fn per_page(self, per_page: Option<i64>) -> PaginatedQuery<Self> {
    let mut per_page = per_page.unwrap_or(15);

    if per_page <= 0 {
      per_page = 15;
    }

    PaginatedQuery {
      query: self,
      page: 1,
      per_page,
    }
  }
}

impl <T> Paginate for T {}

impl<T> QueryFragment<Mysql> for PaginatedQuery<T>
where
  T: QueryFragment<Mysql>,
{
  fn walk_ast<'query>(&'query self, mut out: AstPass<'_, 'query, Mysql>) -> QueryResult<()> {
    out.push_sql("SELECT *, COUNT(*) OVER () FROM (");
    self.query.walk_ast(out.reborrow())?;
    out.push_sql(") t LIMIT ");
    out.push_bind_param::<BigInt, _>(&self.per_page)?;
    let offset = (self.page - 1) * self.per_page;
    out.push_sql(format!(" OFFSET {}", offset).as_str());
    Ok(())
  }
}

impl<T: Query> Query for PaginatedQuery<T> {
  type SqlType = (T::SqlType, BigInt);
}

impl <T> RunQueryDsl<MysqlConnection> for PaginatedQuery<T> {}

impl<T> PaginatedQuery<T> {
    /// Set page number on the query
  pub fn page(self, page: Option<i64>) -> Self {
    let mut page = page.unwrap_or(1);

    if page <= 0 {
      page = 1;
    }

    PaginatedQuery { page, ..self }
  }

  /// Set per page size on the query
  pub fn per_page(self, per_page: Option<i64>) -> Self {
    let mut per_page = per_page.unwrap_or(15);

    if per_page <= 0 {
      per_page = 15;
    }

    PaginatedQuery { per_page, ..self }
  }

  /// Load paginated data with set page and per_page values
  pub fn paginate<'query, U>(self, conn: &mut MysqlConnection) -> QueryResult<ResponseList<U>>
  where
    Self: LoadQuery<'query, MysqlConnection, (U, i64)>,
  {
    let page = self.page;
    let per_page = self.per_page;
    let results = self.load::<(U, i64)>(conn)?;
    let total = results.get(0).map(|x| x.1).unwrap_or(0);
    let data = results.into_iter().map(|x| x.0).collect();
    let mut last_page = 1;

    if total > 0 {
      last_page = (total as f64 / per_page as f64).ceil() as i64;
    }

    Ok(ResponseList {
      page,
      per_page,
      total,
      last_page,
      data,
    })
  }
}