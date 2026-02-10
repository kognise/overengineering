use chrono::{DateTime, Utc};
use diesel::{
    sql_query,
    sql_types::{Integer, Text},
    Insertable, Queryable, QueryableByName, RunQueryDsl, Selectable, SqliteConnection,
};

#[derive(Queryable, Selectable, Clone, Debug)]
#[diesel(table_name = crate::schema::hits)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Hit {
    pub id: i32,
    pub ip_hash: Vec<u8>,
    pub slug: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Insertable, Debug)]
#[diesel(table_name = crate::schema::hits)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewHit<'a> {
    pub slug: &'a str,
    pub ip_hash: &'a [u8],
    pub timestamp: DateTime<Utc>,
}

#[derive(QueryableByName, Debug, Clone)]
pub struct SiteStats {
    #[diesel(sql_type = Text)]
    pub slug: String,
    #[diesel(sql_type = Integer)]
    pub first_visit_users: i32,
    #[diesel(sql_type = Integer)]
    pub returning_users: i32,
    #[diesel(sql_type = Integer)]
    pub total_unique_visitors: i32,
    #[diesel(sql_type = Integer)]
    pub driven_to_others: i32,
}

impl SiteStats {
    pub fn default_for_slug(slug: &str) -> Self {
        SiteStats {
            slug: slug.to_string(),
            first_visit_users: 0,
            returning_users: 0,
            total_unique_visitors: 0,
            driven_to_others: 0,
        }
    }

    pub fn fetch(conn: &mut SqliteConnection) -> Result<Vec<SiteStats>, diesel::result::Error> {
        sql_query("
            WITH user_activity AS (
                SELECT 
                    ip_hash,
                    slug,
                    -- Slug of the very first visit (lowest ID) for this user
                    FIRST_VALUE(slug) OVER (PARTITION BY ip_hash ORDER BY id ASC) as entry_slug
                FROM hits
            ),
            user_profiles AS (
                SELECT 
                    ip_hash,
                    MAX(entry_slug) as entry_slug, -- entry_slug is constant per IP, MAX extracts it
                    COUNT(DISTINCT slug) as distinct_slugs_visited
                FROM user_activity
                GROUP BY ip_hash
            ),
            unique_hits AS (
                SELECT DISTINCT 
                    slug, 
                    ip_hash 
                FROM hits
            )
            SELECT 
                uh.slug,
                
                -- Count visitors where this slug was their entry point
                COUNT(CASE WHEN up.entry_slug = uh.slug THEN 1 END) AS first_visit_users,
                
                -- Count visitors where this slug was NOT their entry point (they came from elsewhere)
                COUNT(CASE WHEN up.entry_slug != uh.slug THEN 1 END) AS returning_users,
                
                -- Total unique visitors
                COUNT(*) AS total_unique_visitors,
                
                -- Users who started here AND went on to visit at least one other slug
                -- (If distinct_slugs_visited > 1 and this was the entry, the other slug(s) must be 'afterward')
                COUNT(CASE 
                    WHEN up.entry_slug = uh.slug AND up.distinct_slugs_visited > 1 THEN 1 
                END) AS driven_to_others

            FROM unique_hits uh
            JOIN user_profiles up ON uh.ip_hash = up.ip_hash
            GROUP BY uh.slug;
        ").load(conn)
    }
}
