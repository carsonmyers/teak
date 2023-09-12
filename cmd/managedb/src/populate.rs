use anyhow::Result;
use chrono::Utc;
use teak_data::tree::{Tree, TreeState, TreeType};
use teak_db::tree::create_tree;
use uuid::{self, Uuid};

use crate::PopulateArgs;

type TreeSpec = (&'static str, TreeType, TreeState);

trait ToTree {
    fn to_tree(self, parent_path: &Vec<Uuid>) -> Tree;
}

impl ToTree for TreeSpec {
    fn to_tree(self, parent_path: &Vec<Uuid>) -> Tree {
        let (title, typ, state) = self;

        let id = Uuid::new_v4();
        let path = parent_path.clone().into_iter().chain(vec![id]).collect();

        Tree {
            id,
            path,
            title: title.to_string(),
            typ,
            state,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

pub fn run(args: &PopulateArgs) -> Result<()> {
    let mut pool = teak_db::connection_pool()?;
    let mut conn = pool.get()?;

    let root: Vec<TreeSpec> = vec![
        ("Team Inbox", TreeType::Inbox, TreeState::Open),
        ("Project", TreeType::Project, TreeState::Open),
        ("Meetings", TreeType::Folder, TreeState::Open),
        ("RFC", TreeType::Folder, TreeState::Open),
    ];

    let inbox: Vec<TreeSpec> = vec![
        ("Visual bug", TreeType::Bug, TreeState::Open),
        ("Client error", TreeType::Bug, TreeState::Open),
        ("New Feature", TreeType::Feature, TreeState::Open),
    ];

    let project: Vec<TreeSpec> = vec![
        ("Login page", TreeType::Feature, TreeState::Open),
        ("User list", TreeType::Feature, TreeState::Open),
        ("User profile", TreeType::Feature, TreeState::Open),
    ];

    let user_list: Vec<TreeSpec> = vec![
        ("Profile images", TreeType::Feature, TreeState::Open),
        ("Pagination", TreeType::Feature, TreeState::Open),
        ("user names truncated", TreeType::Bug, TreeState::Open),
        (
            "can't deactivate accounts",
            TreeType::Bug,
            TreeState::Closed,
        ),
    ];

    let user_profile: Vec<TreeSpec> = vec![
        ("Profile picture", TreeType::Feature, TreeState::Open),
        ("user links not working", TreeType::Bug, TreeState::Open),
        (
            "user description overflows container",
            TreeType::Bug,
            TreeState::Open,
        ),
        (
            "can't view profile as admin",
            TreeType::Bug,
            TreeState::Closed,
        ),
        ("Edits aren't saved", TreeType::Bug, TreeState::Closed),
        (
            "user can't edit their own profile",
            TreeType::Bug,
            TreeState::Closed,
        ),
    ];

    let meetings: Vec<TreeSpec> = vec![
        ("Daily standup Aug 31", TreeType::Meeting, TreeState::Closed),
        ("Daily standup Sept 1", TreeType::Meeting, TreeState::Closed),
        ("Daily standup Sept 2", TreeType::Meeting, TreeState::Open),
        (
            "Sprint planning Week 36",
            TreeType::Meeting,
            TreeState::Open,
        ),
    ];

    let rfc: Vec<TreeSpec> = vec![
        (
            "Email notifications spec",
            TreeType::Document,
            TreeState::Open,
        ),
        ("Friend list spec", TreeType::Document, TreeState::Open),
    ];

    let root_result = root
        .into_iter()
        .map(|spec| create_tree(&mut conn, spec.to_tree(&vec![])))
        .collect::<Result<Vec<Tree>, _>>()?;

    inbox
        .into_iter()
        .map(|spec| create_tree(&mut conn, spec.to_tree(&root_result[0].path)))
        .collect::<Result<Vec<Tree>, _>>()?;

    let project_result = project
        .into_iter()
        .map(|spec| create_tree(&mut conn, spec.to_tree(&root_result[1].path)))
        .collect::<Result<Vec<Tree>, _>>()?;

    user_list
        .into_iter()
        .map(|spec| create_tree(&mut conn, spec.to_tree(&project_result[1].path)))
        .collect::<Result<Vec<Tree>, _>>()?;

    user_profile
        .into_iter()
        .map(|spec| create_tree(&mut conn, spec.to_tree(&project_result[2].path)))
        .collect::<Result<Vec<Tree>, _>>()?;

    meetings
        .into_iter()
        .map(|spec| create_tree(&mut conn, spec.to_tree(&root_result[2].path)))
        .collect::<Result<Vec<Tree>, _>>()?;

    rfc.into_iter()
        .map(|spec| create_tree(&mut conn, spec.to_tree(&root_result[3].path)))
        .collect::<Result<Vec<Tree>, _>>()?;

    Ok(())
}
