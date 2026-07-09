use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::entities::section::Section;
use crate::models::dto::task::TaskDto;


#[derive(Debug, Serialize)]
pub struct SectionDto {
    pub id: Uuid,
    pub title: String,
    pub order_index: i32,
    pub tasks: Vec<TaskDto>,
}

#[derive(Debug, Deserialize)]
pub struct CreateSectionRequest {
    pub course_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub order_index: i32,
}

#[derive(Debug, Serialize)]
pub struct SectionResponseDto {
    pub id: Uuid,
    pub course_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub order_index: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateSectionRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub order_index: Option<i32>,
}


impl From<Section> for SectionResponseDto {

    fn from(section: Section) -> Self {
        Self {
            id: section.id,
            course_id: section.course_id,
            title: section.title,
            description: section.description,
            order_index: section.order_index,
        }
    }
}