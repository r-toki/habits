#[macro_export]
macro_rules! table {
    (
        $table_name:expr,
        $struct_vis:vis struct $struct_name:ident {
            $id_vis:vis id : String,
            $($field_vis:vis $field_name:ident : $field_type:ty,)*
        }
    ) => {
        use derive_new::new;
        use sqlx::{query, PgExecutor, Result, Row};

        #[derive(new, Debug)]
        $struct_vis struct $struct_name {
            $id_vis id : String,
            $($field_vis $field_name : $field_type),*
        }

        impl $struct_name {
            #[allow(dead_code)]
            pub async fn find(executer: impl PgExecutor<'_>, id: String) -> Result<$struct_name> {
                let sql = format!("select * from {} where id = $1", $table_name);
                let row = query(&sql).bind(id).fetch_one(executer).await?;

                Ok($struct_name::new(row.get("id"), $(row.get(stringify!($field_name))),*))
            }

            #[allow(dead_code)]
            pub async fn find_by_id(executer: impl PgExecutor<'_>, id: String) -> Result<Option<$struct_name>> {
                let sql = format!("select * from {} where id = $1", $table_name);
                let row = query(&sql).bind(id).fetch_optional(executer).await?;

                match row {
                    Some(row) => Ok(Some($struct_name::new(row.get("id"), $(row.get(stringify!($field_name))),*))),
                    None => Ok(None),
                }
            }

            #[allow(dead_code)]
            pub async fn delete_by_id(executer: impl PgExecutor<'_>, id: String) -> Result<()> {
                let sql = format!("delete from {} where id = $1", $table_name);
                query(&sql).bind(id).execute(executer).await?;

                Ok(())
            }

            #[allow(dead_code)]
            pub async fn upsert(&self, executer: impl PgExecutor<'_>) -> Result<()> {
                let fields = ["id".to_string(), $(stringify!($field_name).to_string()),*];

                let fields_str = fields.join(", ");
                let values_str = fields.iter().enumerate().map(|(i, _)| format!("${}", i+1)).collect::<Vec<String>>().join(", ");
                let set_str = fields.iter().enumerate().map(|(i, f)| format!("{} = ${}", f, i+1)).filter(|s| !s.starts_with("id")).collect::<Vec<String>>().join(", ");

                let sql = format!("insert into {} ({}) values ({}) on conflict (id) do update set {}", $table_name, fields_str, values_str, set_str);
                query(&sql).bind(self.id.clone())$(.bind(self.$field_name.clone()))*.execute(executer).await?;

                Ok(())
            }

            #[allow(dead_code)]
            pub async fn delete(&self, executer: impl PgExecutor<'_>) -> Result<()> {
                let sql = format!("delete from {} where id = $1", $table_name);
                query(&sql).bind(self.id.clone()).execute(executer).await?;

                Ok(())
            }
        }
    };
}
