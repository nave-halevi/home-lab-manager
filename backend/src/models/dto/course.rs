use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::dto::section::SectionDto;
use crate::models::entities::course::Course;

#[derive(Debug, Deserialize)]
pub struct CreateCourseRequest {
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub difficulty: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CourseFullDto {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub difficulty: Option<String>,
    pub is_published: bool,
    pub created_at: DateTime<Utc>,
    pub sections: Vec<SectionDto>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCourseRequest {
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub difficulty: Option<String>,
    pub is_published: bool,
}

#[derive(Debug, Serialize)]
pub struct CourseResponseDto {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub description: Option<String>,
    pub difficulty: Option<String>,
    pub is_published: bool,
    pub created_at: DateTime<Utc>,
}

impl From<Course> for CourseResponseDto {
    fn from(course: Course) -> Self {
        Self {
            id: course.id,
            title: course.title,
            slug: course.slug,
            description: course.description,
            difficulty: course.difficulty,
            is_published: course.is_published,
            created_at: course.created_at,
        }
    }
}
