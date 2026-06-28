# Home Lab Manager - Database Schema

מסד נתונים: PostgreSQL

---

# users

מטרת הטבלה:
שומרת את כל המשתמשים הרשומים במערכת.

| Column | Type | Description |
|---------|------|-------------|
| id | UUID | מזהה ייחודי של המשתמש (Primary Key) |
| user_name | VARCHAR(225) | שם המשתמש |
| email | VARCHAR(255) | כתובת אימייל (Unique) |
| password_hash | VARCHAR(255) | סיסמה מוצפנת (Hash) |
| created_at | TIMESTAMP | תאריך יצירת המשתמש |
| updated_at | TIMESTAMP | תאריך עדכון אחרון |
| total_score | INTEGER | סך כל הנקודות שצבר המשתמש |
| role | VARCHAR(50) | תפקיד המשתמש (user/admin בעתיד) |

Relations

- One User → Many Environments
- One User → Many Solved Flags

---

# scenarios

מטרת הטבלה:
מייצגת Lab/Scenario שניתן להפעיל.

לדוגמה:

- Linux Privilege Escalation
- Active Directory
- Web Exploitation

| Column | Type | Description |
|---------|------|-------------|
| id | UUID | מזהה ייחודי של התרחיש |
| title | VARCHAR(255) | שם התרחיש |
| difficulty | VARCHAR(50) | רמת קושי |
| description | TEXT | תיאור התרחיש |
| created_at | TIMESTAMP | תאריך יצירה |
| vm_template_name | VARCHAR(255) | שם תבנית המכונה שממנה ייווצר ה-Lab |

Relations

- One Scenario → Many Environments
- One Scenario → Many Flags

---

# environments

מטרת הטבלה:
מייצגת סביבת Lab שנוצרה עבור משתמש.

כל Environment שייך למשתמש אחד ומבוסס על Scenario אחד.

| Column | Type | Description |
|---------|------|-------------|
| id | UUID | מזהה הסביבה |
| user_id | UUID | המשתמש שיצר את הסביבה |
| scenario_id | UUID | התרחיש שעליו מבוססת הסביבה |
| network_name | VARCHAR(100) | שם רשת Docker שנוצרה |
| status | VARCHAR(50) | מצב הסביבה (Building / Running / Destroyed וכו') |
| created_at | TIMESTAMP | זמן יצירת הסביבה |

Relations

- Belongs To User
- Belongs To Scenario
- Has Many Instances

---

# instances

מטרת הטבלה:
מייצגת מכונה בודדת בתוך Environment.

לדוגמה:

- Kali
- Victim
- Domain Controller

| Column | Type | Description |
|---------|------|-------------|
| id | UUID | מזהה המכונה |
| environment_id | UUID | הסביבה שאליה המכונה שייכת |
| vm_name | VARCHAR(100) | שם המכונה |
| is_entry_point | BOOLEAN | האם זו המכונה הראשונה שאליה המשתמש מתחבר |
| internal_ip | VARCHAR(50) | כתובת ה-IP הפנימית |
| created_at | TIMESTAMP | זמן יצירת המכונה |

Relations

- Belongs To Environment

---

# flags

מטרת הטבלה:
שומרת את כל ה-Flags של כל Scenario.

| Column | Type | Description |
|---------|------|-------------|
| id | UUID | מזהה ה-Flag |
| scenario_id | UUID | התרחיש שאליו שייך ה-Flag |
| flag_value | VARCHAR(255) | ערך ה-Flag |
| points | INTEGER | מספר הנקודות שה-Flag שווה |

Relations

- Belongs To Scenario
- One Flag → Many User Solves

---

# user_flags

מטרת הטבלה:
שומרת אילו משתמשים פתרו אילו Flags.

זוהי טבלת קשר (Many-to-Many).

| Column | Type | Description |
|---------|------|-------------|
| id | UUID | מזהה הרשומה |
| user_id | UUID | המשתמש שפותר |
| flag_id | UUID | ה-Flag שנפתר |
| solved_at | TIMESTAMP | זמן הפתרון |

Constraints

- UNIQUE(user_id, flag_id)

כלומר:

משתמש לא יכול לפתור את אותו Flag יותר מפעם אחת.

Relations

- Belongs To User
- Belongs To Flag

---

# _sqlx_migrations

מטרת הטבלה:

טבלה פנימית של SQLx.

משמשת למעקב אחר כל ה-Migrations שבוצעו במסד.

אין להשתמש בה ישירות בקוד העסקי.

---

# Entity Relationship

User

↓

Environment

↓

Instance

User

↓

User_Flags

↓

Flag

↓

Scenario

Scenario

↓

Environment

---

# Current Database Summary

Tables

- users
- scenarios
- environments
- instances
- flags
- user_flags
- _sqlx_migrations

Main Features Supported

✔ User Registration

✔ Authentication

✔ Scenario Management

✔ Environment Creation

✔ VM Instance Tracking

✔ Docker Network Tracking

✔ Flag System

✔ User Score Tracking

✔ One-Time Flag Submission

✔ SQLx Migrations