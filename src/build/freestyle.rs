use helpers::Class;

use super::{Artifact, Build, BuildStatus};
use action::CommonAction;
use changeset;
use user::ShortUser;

build_with_common_fields_and_impl!(/// A `Build` from a FreeStyleProject
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FreeStyleBuild {
    /// Which slave was it build on
    pub built_on: String,
    /// Change set for this build
    pub change_set: changeset::CommonChangeSetList,
    /// List of user ids who made a change since the last non-broken build
    pub culprits: Vec<ShortUser>,
});
register_class!("hudson.model.FreeStyleBuild" => FreeStyleBuild);

impl FreeStyleBuild {}
