-- Add migration script here
-- COURSES
CREATE TABLE courses
(
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(255) NOT NULL,
    slug VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    difficulty VARCHAR(50),
    is_published BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- SECTIONS
CREATE TABLE sections
(
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    course_id UUID NOT NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    order_index INT NOT NULL,

    FOREIGN KEY (course_id) REFERENCES courses(id) ON DELETE CASCADE,
    UNIQUE(course_id, order_index)
);

-- TASKS
CREATE TABLE tasks
(
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    section_id UUID NOT NULL,
    scenario_id UUID NULL,

    title VARCHAR(255) NOT NULL,
    content TEXT NOT NULL,

    task_type VARCHAR(50) NOT NULL,
    -- LAB / QUIZ / READING / VIDEO

    order_index INT NOT NULL,
    points INT NOT NULL DEFAULT 10,

    FOREIGN KEY (section_id) REFERENCES sections(id) ON DELETE CASCADE,
    UNIQUE(section_id, order_index)
);

-- USER TASK PROGRESS
CREATE TABLE user_task_progress
(
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    task_id UUID NOT NULL,

    status VARCHAR(50) NOT NULL,
    -- LOCKED / AVAILABLE / IN_PROGRESS / COMPLETED

    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,

    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE CASCADE,

    UNIQUE(user_id, task_id)
);