
pub mod handlers;
pub mod messages;
pub mod session;
pub mod server;


#[derive(Debug, Clone)]
pub enum UserOperation {
  Login,
  GetCaptcha,
  MarkCommentAsRead,
  SaveComment,
  CreateCommentLike,
  CreateCommentReport,
  ResolveCommentReport,
  ListCommentReports,
  CreatePostLike,
  LockPost,
  StickyPost,
  MarkPostAsRead,
  SavePost,
  CreatePostReport,
  ResolvePostReport,
  ListPostReports,
  GetReportCount,
  GetUnreadCount,
  VerifyEmail,
  FollowCommunity,
  GetReplies,
  GetPersonMentions,
  MarkPersonMentionAsRead,
  GetModlog,
  BanFromCommunity,
  AddModToCommunity,
  AddAdmin,
  GetUnreadRegistrationApplicationCount,
  ListRegistrationApplications,
  ApproveRegistrationApplication,
  BanPerson,
  GetBannedPersons,
  Search,
  ResolveObject,
  MarkAllAsRead,
  SaveUserSettings,
  TransferCommunity,
  LeaveAdmin,
  PasswordReset,
  PasswordChange,
  MarkPrivateMessageAsRead,
  UserJoin,
  GetSiteConfig,
  SaveSiteConfig,
  PostJoin,
  CommunityJoin,
  ModJoin,
  ChangePassword,
  GetSiteMetadata,
  BlockCommunity,
  BlockPerson,
}

#[derive(Debug, Clone)]
pub enum UserOperationCrud {
  CreateRoom,
  UpdateRoom,
  DeleteRoom,
  CreateTask,
  UpdateTask,
  DeleteTask,
  CreateEvent,
  AddFollower,
  DeleteFollower,
  

}

pub trait OperationType {}

impl OperationType for UserOperationCrud {}

impl OperationType for UserOperation {}