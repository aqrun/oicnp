use loco_rs::prelude::*;
use crate::entities::prelude::NoteActiveModel;

#[async_trait::async_trait]
impl ActiveModelBehavior for NoteActiveModel {}