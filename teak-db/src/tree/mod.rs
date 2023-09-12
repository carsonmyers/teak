use diesel::prelude::*;
use teak_data::tree::Tree;
use uuid::Uuid;

use crate::models;
use crate::Error;

pub fn get_tree(c: &mut PgConnection, id: Uuid) -> Result<Tree, Error> {
    use crate::schema::tree;

    let tree = tree::table
        .filter(tree::id.eq(id))
        .first::<models::tree::Tree>(c)?;

    Ok(tree.into())
}

pub fn list_tree(c: &mut PgConnection, path: Vec<Uuid>) -> Result<Vec<Tree>, Error> {
    use crate::schema::tree;

    let trees = tree::table
        .filter(tree::path.eq(path))
        .load::<models::tree::Tree>(c)?;

    Ok(trees.into_iter().map(|t| t.into()).collect())
}

pub fn create_tree(c: &mut PgConnection, tree: Tree) -> Result<Tree, Error> {
    use crate::schema::tree;

    let new_tree: models::tree::NewTree = (&tree).into();
    let query = diesel::insert_into(tree::table)
        .values(new_tree)
        .returning(models::tree::Tree::as_returning());

    let debug = diesel::debug_query::<diesel::pg::Pg, _>(&query);
    println!("{}", debug.to_string());

    let result = query.get_result(c)?;

    Ok(result.into())
}
