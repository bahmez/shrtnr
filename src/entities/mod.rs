pub mod user;
pub mod workspace;
pub mod workspace_member;
pub mod shortened_link;
pub mod link_click;

pub use user::Entity as User;
pub use workspace::Entity as Workspace;
pub use workspace_member::Entity as WorkspaceMember;
pub use shortened_link::Entity as ShortenedLink;
pub use link_click::Entity as LinkClick;

