pub mod link_click;
pub mod shortened_link;
pub mod user;
pub mod workspace;
pub mod workspace_member;

pub use link_click::Entity as LinkClick;
pub use shortened_link::Entity as ShortenedLink;
pub use user::Entity as User;
pub use workspace::Entity as Workspace;
pub use workspace_member::Entity as WorkspaceMember;
