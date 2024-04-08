use super::models::*;
use chrono::NaiveDateTime;
use sqlx::postgres::PgPool;

pub async fn get_animals_by_id_db(
    pool: &PgPool,
    animal_id: i32,
// ) -> Result<Vec<Animal>, sqlx::Error> {
) -> Vec<Animal> {
    // 编写代码以从数据库中获取动物信息
    // let animals = sqlx::query_as::<_, Animal>(
    //     "SELECT * FROM animals WHERE id = $1",
    // )

    // let animals = sqlx::query!(
    //     r#"
    //     SELECT id, name, time 
    //     FROM Animals
    //     WHERE id = $1
    //     "#,
    //     animal_id
    // )
    // .fetch_all(pool)
    // .await?;

    // 旧的 // let rows = sqlx::query::<sqlx::Postgres>(
    //     "SELECT id, name, time FROM Animals WHERE id = $1"
    // )
    // .bind(animal_id)
    let rows = sqlx::query!(
        // Animal,  // query_as!()
        r#"
        SELECT id, name, time 
        FROM Animals 
        WHERE id = $1
        "#,
        animal_id
    )
    .fetch_all(pool)
    .await
    .unwrap();

    rows.iter()  // 每行
        .map(|r| Animal {
            id: r.id,
            name: r.name.clone().unwrap(),
            time: r.time,
        })
        .collect()
}

pub async fn post_new_animal_db(
    pool: &PgPool,
    new_animal: Animal
) -> Animal {
    let row = sqlx::query!(
        // Animal,  // query_as!()
        r#"
        INSERT INTO Animals (id, name)
        VALUES ($1, $2)
        RETURNING id, name, time
        "#,
        new_animal.id,
        new_animal.name,
    )
    .fetch_one(pool)
    .await
    .unwrap();

    Animal {
        id: row.id,
        name: row.name.clone().unwrap(),
        time: row.time,
    }
}