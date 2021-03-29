// use std::collections::VecDeque;

// use actix::dev::{
//     AsyncContextParts, ContextFut, ContextParts, Envelope, Mailbox, StreamHandler, ToEnvelope,
// };
// use actix::fut::ActorFuture;
// use actix::{
//     Actor, ActorContext, ActorState, Addr, AsyncContext, Handler, Message as ActixMessage,
//     SpawnHandle,
// };
// pub struct RedisContext<A>
// where
//     A: Actor<Context = RedisContext<A>>,
// {
//     inner: ContextParts<A>,
//     messages: VecDeque<Option<Message>>,
// }
