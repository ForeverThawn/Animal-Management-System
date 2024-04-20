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


// pub async fn get_shoot_by_id_db(
//     pool: &PgPool,
//     bamboo_id: i32,
// // ) -> Result<Vec<Animal>, sqlx::Error> {
// ) -> Vec<Bamboo> {
//     let rows = sqlx::query!(
//         // Bamboo,  // query_as!()
//         r#"
//         SELECT id, time, position_x, position_y, position_z
//         FROM Bamboos 
//         WHERE id = $1
//         "#,
//         bamboo_id
//     )
//     .fetch_all(pool)
//     .await
//     .unwrap();

//     rows.iter()
//         .map(|r| Bamboo {
//             id: r.id,
//             name: r.name.clone().unwrap(),  // 简单parse下
//             time: r.time,
//         })
//         .collect() // 之后用result
// }

// pub async fn post_new_shoot_db(
//     pool: &PgPool,
//     new_bamboo: Bamboo // parsed json进来
// ) -> Bamboo {
//     let row = sqlx::query!(
//         r#"
//         INSERT INTO Bamboos (x, y, z)
//         VALUES ($1, $2, $3)
//         RETURNING id, time, x, y, z
//         "#,
//         new_bamboo.x,
//         new_bamboo.y,
//         new_bamboo.z,
//     )
//     .fetch_one(pool)
//     .await
//     .unwrap();

//     Bamboo {
//         id: row.id,
//         name: row.name.clone().unwrap(),  // 简单parse下
//         time: row.time,
//     }
// }