-- Add migration script here
CREATE TABLE scenarios (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(255) NOT NULL,
    difficulty VARCHAR(50),
    description TEXT,
    vm_template_name VARCHAR(255) DEFAULT 'Ubuntu_Base_Template'
);

CREATE TABLE environments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    scenario_id UUID NOT NULL,
    network_name VARCHAR(100) NOT NULL,
    status VARCHAR(50) NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),

    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (scenario_id) REFERENCES scenarios(id) ON DELETE CASCADE
);

CREATE TABLE instances (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    environment_id UUID NOT NULL,
    vm_name VARCHAR(100) NOT NULL,
    is_entry_point BOOLEAN DEFAULT FALSE,
    internal_ip VARCHAR(50),
    created_at TIMESTAMPTZ DEFAULT NOW(),

    FOREIGN KEY (environment_id) REFERENCES environments(id) ON DELETE CASCADE
);

CREATE TABLE flags (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    scenario_id UUID,
    flag_value VARCHAR(255) NOT NULL,
    points INT NOT NULL DEFAULT 10,

    FOREIGN KEY (scenario_id) REFERENCES scenarios(id) ON DELETE CASCADE
);

CREATE TABLE user_flags (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID,
    flag_id UUID,
    solved_at TIMESTAMPTZ DEFAULT NOW(),

    UNIQUE(user_id, flag_id),
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (flag_id) REFERENCES flags(id) ON DELETE CASCADE
);