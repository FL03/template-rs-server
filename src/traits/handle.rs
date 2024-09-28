/*
    Appellation: handlers <module>
    Contrib: FL03 <jo3mccain@icloud.com>
*/
use async_trait::async_trait;

/// [AsyncHandle] is a trait used to define the method used to process a particular command,
/// event, etc.
#[async_trait]
pub trait AsyncHandle<T> {
    type Output;

    async fn handle(self, ctx: &mut T) -> Self::Output;
}
